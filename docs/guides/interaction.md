# Page Interaction

Kreuzcrawl supports programmatic page interaction through a sequence of browser actions.
This enables workflows like logging in, filling forms, scrolling infinite lists, and
extracting content that only appears after user interaction.

## PageAction types

Actions are defined as a `Vec<PageAction>` and executed sequentially in the browser context.
Each action is serialized with a `type` tag using camelCase naming.

### Click

Click on an element matching a CSS selector.

```json
{ "type": "click", "selector": "#login-button" }
```

### Type

Type text into an input element.

```json
{ "type": "type", "selector": "#email-input", "text": "user@example.com" }
```

### Press

Press a keyboard key by name (e.g. `Enter`, `Tab`, `Escape`).

```json
{ "type": "press", "key": "Enter" }
```

### Scroll

Scroll the page or a specific scrollable element.

```json
{ "type": "scroll", "direction": "down", "amount": 500 }
{ "type": "scroll", "direction": "up", "selector": ".scrollable-div" }
```

The `direction` field accepts `"up"` or `"down"`. The optional `selector` targets a
specific scrollable container; omit it to scroll the page itself. The optional `amount`
specifies pixels (defaults to a browser-determined value).

### Wait

Wait for a duration or for an element to appear.

```json
{ "type": "wait", "milliseconds": 2000 }
{ "type": "wait", "selector": ".loaded-content" }
```

When `selector` is provided, the wait blocks until that element appears in the DOM.
When only `milliseconds` is provided, it pauses for that duration.

### Screenshot

Capture a screenshot of the current page.

```json
{ "type": "screenshot" }
{ "type": "screenshot", "fullPage": true }
```

Set `fullPage` to `true` to capture the entire scrollable page rather than just the viewport.

### ExecuteJs

Execute arbitrary JavaScript in the page context.

```json
{ "type": "executeJs", "script": "document.querySelector('#app').dataset.loaded" }
```

!!! warning "Security"
    The script runs with full page privileges. Only execute scripts from trusted sources.
    The maximum script length is 1 MB.

### Scrape

Capture the current page HTML at this point in the action sequence.

```json
{ "type": "scrape" }
```

## Validation rules

All action sequences are validated before execution. The following limits are enforced:

| Constraint | Limit |
|-----------|-------|
| Maximum actions per sequence | 100 |
| Maximum total wait time | 300 seconds (5 minutes) |
| Maximum single wait duration | 300,000 ms (5 minutes) |
| Maximum CSS selector length | 4,096 bytes |
| Maximum JavaScript script length | 1,048,576 bytes (1 MB) |
| Maximum text length (Type action) | 1,048,576 bytes (1 MB) |
| Maximum scroll amount | 100,000 pixels |

Additional validations:

- `Click` and `Type` selectors must not be empty.
- `Press` key must not be empty.
- `ExecuteJs` script must not be empty.

Validation errors return `CrawlError::InvalidConfig` with a message indicating the
offending action index and the violated constraint.

## InteractionResult structure

After all actions execute, the result contains:

```rust
pub struct InteractionResult {
    /// Results from each executed action.
    pub action_results: Vec<ActionResult>,
    /// Final page HTML after all actions completed.
    pub final_html: String,
    /// Final page URL (may have changed due to navigation).
    pub final_url: String,
    /// Screenshot taken after all actions, if requested.
    pub screenshot: Option<Vec<u8>>,
}

pub struct ActionResult {
    /// Zero-based index of the action in the sequence.
    pub action_index: usize,
    /// The type of action that was executed (e.g. "click", "type").
    pub action_type: Cow<'static, str>,
    /// Whether the action completed successfully.
    pub success: bool,
}
```

## Examples

### Login flow

```json
[
  { "type": "type", "selector": "#username", "text": "admin" },
  { "type": "type", "selector": "#password", "text": "s3cret" },
  { "type": "click", "selector": "#login-btn" },
  { "type": "wait", "selector": ".dashboard" },
  { "type": "scrape" }
]
```

### Infinite scroll

```json
[
  { "type": "wait", "selector": ".feed-item" },
  { "type": "scroll", "direction": "down", "amount": 2000 },
  { "type": "wait", "milliseconds": 1500 },
  { "type": "scroll", "direction": "down", "amount": 2000 },
  { "type": "wait", "milliseconds": 1500 },
  { "type": "scroll", "direction": "down", "amount": 2000 },
  { "type": "wait", "milliseconds": 1500 },
  { "type": "scrape" }
]
```

### Form fill with JavaScript validation

```json
[
  { "type": "type", "selector": "#name", "text": "Jane Doe" },
  { "type": "type", "selector": "#email", "text": "jane@example.com" },
  { "type": "click", "selector": "select#country" },
  { "type": "executeJs", "script": "document.querySelector('select#country').value = 'DE'" },
  { "type": "press", "key": "Tab" },
  { "type": "wait", "milliseconds": 500 },
  { "type": "click", "selector": "#submit" },
  { "type": "wait", "selector": ".confirmation-message" },
  { "type": "screenshot", "fullPage": true }
]
```

!!! tip "Combining with browser profiles"
    Use a `BrowserProfile` to persist the session across multiple interaction runs.
    This avoids re-authenticating on every crawl. See the
    [Browser Automation guide](browser.md#browser-profiles-persistent-sessions) for details.
