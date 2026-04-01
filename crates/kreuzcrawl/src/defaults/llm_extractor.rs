//! LLM-powered content extraction using liter-llm.
//!
//! Requires the `ai` feature flag.

#[cfg(feature = "ai")]
mod inner {
    use async_trait::async_trait;
    use serde_json::Value;

    use crate::error::CrawlError;
    use crate::traits::ContentFilter;
    use crate::types::{CrawlPageResult, ExtractionMeta};

    /// Extracts structured data from crawled pages using an LLM.
    pub struct LlmExtractor {
        client: liter_llm::DefaultClient,
        model: String,
        schema: Option<Value>,
        instruction: Option<String>,
    }

    impl LlmExtractor {
        /// Create a new LLM extractor.
        ///
        /// - `api_key`: API key for the LLM provider
        /// - `model`: Model identifier (e.g. `"openai/gpt-4o-mini"`, `"anthropic/claude-sonnet-4-20250514"`)
        /// - `schema`: Optional JSON schema for structured extraction
        /// - `instruction`: Optional extraction instruction
        pub fn new(
            api_key: &str,
            model: &str,
            schema: Option<Value>,
            instruction: Option<String>,
        ) -> Result<Self, CrawlError> {
            let config = liter_llm::ClientConfig::new(api_key);
            let client = liter_llm::DefaultClient::new(config, Some(model))
                .map_err(|e| CrawlError::Other(format!("failed to create LLM client: {e}")))?;
            Ok(Self {
                client,
                model: model.to_owned(),
                schema,
                instruction,
            })
        }
    }

    #[async_trait]
    impl ContentFilter for LlmExtractor {
        async fn filter(
            &self,
            mut page: CrawlPageResult,
        ) -> Result<Option<CrawlPageResult>, CrawlError> {
            use liter_llm::LlmClient;

            // Use markdown if available, fall back to HTML.
            let content = page
                .markdown
                .as_ref()
                .map(|m| m.content.as_str())
                .unwrap_or(&page.html);

            // Build prompt.
            let mut prompt = String::new();
            if let Some(ref instruction) = self.instruction {
                prompt.push_str(instruction);
                prompt.push_str("\n\n");
            }
            if let Some(ref schema) = self.schema {
                prompt.push_str("Extract data matching this JSON schema:\n");
                prompt.push_str(&serde_json::to_string_pretty(schema).unwrap_or_default());
                prompt.push_str("\n\n");
            }
            prompt.push_str("Content:\n");
            prompt.push_str(content);

            // Build request.
            let mut request = liter_llm::ChatCompletionRequest::default();
            request.model = self.model.clone();
            request.messages = vec![
                liter_llm::Message::System(liter_llm::SystemMessage {
                    content: "You are a data extraction assistant. Extract structured data from the provided content. Return valid JSON only.".into(),
                    name: None,
                }),
                liter_llm::Message::User(liter_llm::UserMessage {
                    content: liter_llm::UserContent::Text(prompt),
                    name: None,
                }),
            ];
            request.response_format =
                self.schema
                    .as_ref()
                    .map(|s| liter_llm::ResponseFormat::JsonSchema {
                        json_schema: liter_llm::JsonSchemaFormat {
                            name: "extraction".to_owned(),
                            description: None,
                            schema: s.clone(),
                            strict: Some(true),
                        },
                    });

            // Call LLM.
            let response = self
                .client
                .chat(request)
                .await
                .map_err(|e| CrawlError::Other(format!("LLM extraction failed: {e}")))?;

            // Extract cost and token usage.
            let cost = response.estimated_cost();
            let usage = response.usage.as_ref();

            page.extraction_meta = Some(ExtractionMeta {
                cost,
                prompt_tokens: usage.map(|u| u.prompt_tokens),
                completion_tokens: usage.map(|u| u.completion_tokens),
                model: Some(self.model.clone()),
                chunks_processed: 1,
            });

            // Parse response.
            if let Some(choice) = response.choices.first()
                && let Some(ref text) = choice.message.content
            {
                let extracted: Value =
                    serde_json::from_str(text).unwrap_or_else(|_| Value::String(text.clone()));
                page.extracted_data = Some(extracted);
            }

            Ok(Some(page))
        }
    }
}

#[cfg(feature = "ai")]
pub use inner::LlmExtractor;
