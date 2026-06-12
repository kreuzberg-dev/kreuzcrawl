//! OTel initialisation helpers and W3C TraceContext propagation bridges.
//!
//! # Feature-gated initialisation
//!
//! `init_otlp` is compiled only when the `telemetry-init` feature is active.
//! Cloud consumers wire their own `TracerProvider` / `MeterProvider` and skip
//! this feature entirely.
//!
//! # Unconditional propagation helpers
//!
//! `with_traceparent` and `current_traceparent` are **always** compiled.
//! They let every language binding propagate a W3C `traceparent` header from
//! the host-language OTel SDK (Python's `opentelemetry-sdk`, Node's
//! `@opentelemetry/api`, etc.) into kreuzcrawl Rust calls so spans appear as
//! children of host-language spans in the same collector trace.

use std::collections::HashMap;

use opentelemetry::propagation::{Extractor, Injector};

// ---------------------------------------------------------------------------
// W3C TraceContext propagation bridges (unconditional)
// ---------------------------------------------------------------------------

struct SingleHeaderMap(HashMap<String, String>);

impl Extractor for SingleHeaderMap {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }
}

impl Injector for SingleHeaderMap {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }
}

/// Extract a W3C TraceContext from a `traceparent` header string and execute
/// `f` with that context as the active OpenTelemetry context.
///
/// This is the binding-layer bridge that lets host-language OTel SDKs (Python,
/// Node, Ruby, Go, …) pass a parent span context into kreuzcrawl Rust calls.
/// The Rust span created inside `f` will be a child of the host span in the
/// collector.
///
/// If `traceparent` is invalid or empty, or if no propagator has been
/// registered, the call behaves identically to calling `f()` directly —
/// no panic, no error.
///
/// When `telemetry-init` is active and `init_otlp` has been called, the
/// W3C TraceContext propagator is registered globally and this function
/// correctly parses W3C `traceparent` values.  In other deployment
/// configurations (e.g. cloud consumers that own their own SDK initialisation)
/// the registered propagator is whatever the application installed via
/// `opentelemetry::global::set_text_map_propagator`.
pub fn with_traceparent<F, R>(traceparent: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let mut carrier = SingleHeaderMap(HashMap::new());
    carrier.0.insert("traceparent".to_owned(), traceparent.to_owned());
    let parent_cx = opentelemetry::global::get_text_map_propagator(|p| p.extract(&carrier));
    let _guard = opentelemetry::Context::attach(parent_cx);
    f()
}

/// Encode the active OpenTelemetry context as a W3C `traceparent` header value.
///
/// Returns `None` when there is no active remote span context (i.e. no span
/// is in-flight or the span is not sampled), or when no propagator has been
/// registered.
///
/// Use this in language bindings to hand the current kreuzcrawl trace context
/// back to the host-language OTel SDK, so host-side code can continue the
/// same trace after a kreuzcrawl call returns.
pub fn current_traceparent() -> Option<String> {
    let cx = opentelemetry::Context::current();
    let mut carrier = SingleHeaderMap(HashMap::new());
    opentelemetry::global::get_text_map_propagator(|p| p.inject_context(&cx, &mut carrier));
    carrier.0.remove("traceparent")
}

// ---------------------------------------------------------------------------
// Full OTLP initialisation (telemetry-init feature only)
// ---------------------------------------------------------------------------

#[cfg(feature = "telemetry-init")]
mod otlp {
    use opentelemetry::KeyValue;
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_otlp::{SpanExporter, WithExportConfig};
    use opentelemetry_sdk::Resource;
    use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use thiserror::Error;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    /// Configuration for [`init_otlp`].
    pub struct TelemetryConfig {
        /// `service.name` resource attribute (required).
        pub service_name: String,
        /// `service.version` resource attribute (optional).
        pub service_version: Option<String>,
        /// OTLP gRPC endpoint, e.g. `"http://localhost:4317"`.
        pub otlp_endpoint: String,
        /// Additional resource attributes as `(key, value)` pairs.
        pub resource_attrs: Vec<(String, String)>,
    }

    /// Errors returned by [`init_otlp`].
    #[derive(Debug, Error)]
    pub enum InitError {
        /// Failed to build the OTLP span exporter.
        #[error("failed to build OTLP span exporter: {0}")]
        SpanExporterBuild(#[from] opentelemetry_otlp::ExporterBuildError),
        /// Failed to build the OTLP metric exporter.
        #[error("failed to build OTLP metric exporter: {0}")]
        MetricExporterBuild(opentelemetry_otlp::ExporterBuildError),
        /// Failed to initialise the `tracing` subscriber.
        #[error("failed to initialise tracing subscriber: {0}")]
        SubscriberInit(#[from] tracing_subscriber::util::TryInitError),
    }

    /// Returned by [`init_otlp`]; shuts down the tracer and meter providers on drop.
    pub struct TelemetryGuard {
        tracer_provider: SdkTracerProvider,
        meter_provider: SdkMeterProvider,
    }

    impl Drop for TelemetryGuard {
        fn drop(&mut self) {
            if let Err(e) = self.tracer_provider.shutdown() {
                tracing::warn!(error = %e, "error shutting down tracer provider");
            }
            if let Err(e) = self.meter_provider.shutdown() {
                tracing::warn!(error = %e, "error shutting down meter provider");
            }
        }
    }

    /// Initialise a TracerProvider + MeterProvider wired to an OTLP collector,
    /// register the W3C TraceContext propagator, and bridge `tracing` spans to OTel.
    ///
    /// # Errors
    ///
    /// Returns [`InitError`] if the OTLP exporter cannot be built or if a
    /// `tracing` subscriber is already registered.
    pub fn init_otlp(config: TelemetryConfig) -> Result<TelemetryGuard, InitError> {
        // ---- resource -------------------------------------------------------
        let mut resource_builder = Resource::builder().with_service_name(config.service_name);
        if let Some(version) = config.service_version {
            resource_builder = resource_builder.with_attribute(KeyValue::new("service.version", version));
        }
        for (key, value) in config.resource_attrs {
            resource_builder = resource_builder.with_attribute(KeyValue::new(key, value));
        }
        let resource = resource_builder.build();

        // ---- tracer provider ------------------------------------------------
        let span_exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(&config.otlp_endpoint)
            .build()?;
        let tracer_provider = SdkTracerProvider::builder()
            .with_batch_exporter(span_exporter)
            .with_resource(resource.clone())
            .build();
        let tracer = tracer_provider.tracer("kreuzcrawl");
        opentelemetry::global::set_tracer_provider(tracer_provider.clone());

        // ---- meter provider -------------------------------------------------
        let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_tonic()
            .with_endpoint(&config.otlp_endpoint)
            .build()
            .map_err(InitError::MetricExporterBuild)?;
        let reader = PeriodicReader::builder(metric_exporter)
            .with_interval(std::time::Duration::from_secs(15))
            .build();
        let meter_provider = SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(resource)
            .build();
        opentelemetry::global::set_meter_provider(meter_provider.clone());

        // ---- W3C TraceContext propagator ------------------------------------
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        // ---- tracing bridge (no-op if already initialised) ------------------
        let fmt_layer = tracing_subscriber::fmt::layer().json();
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(fmt_layer)
            .with(otel_layer)
            .try_init()?;

        Ok(TelemetryGuard {
            tracer_provider,
            meter_provider,
        })
    }
}

#[cfg(feature = "telemetry-init")]
pub use otlp::{InitError, TelemetryConfig, TelemetryGuard, init_otlp};
