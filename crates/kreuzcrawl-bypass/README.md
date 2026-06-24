# kreuzcrawl-bypass

Configurable HTTP-based `BypassProvider` implementations for [kreuzcrawl](https://github.com/xberg-io/kreuzcrawl), driven by per-vendor YAML configs.

Drop a YAML file in `configs/`, set the env vars it references, and wire up a `SimpleHttpProvider` — no Rust code required per vendor.

## Adding a new vendor

1. Copy one of the bundled configs (e.g. `configs/bright_data.yaml`) to `configs/<vendor>.yaml`.
2. Fill in the endpoint, auth, request, and response fields. See the schema reference below.
3. Set the env vars your config references (e.g. `BYPASS_<VENDOR>_API_KEY`).
4. Load and construct:

```rust
use std::path::Path;
use kreuzcrawl_bypass::{load_with_process_env, SimpleHttpProvider};

let config = load_with_process_env(Path::new("configs/my_vendor.yaml"))?;
let provider = SimpleHttpProvider::new(config)?;
```

This crate is the OSS extension point for bypass vendors: you can ship your own configs without licensing kreuzberg-cloud.

---

## YAML schema reference

### Top-level fields

| Field | Type | Required | Description |
|---|---|---|---|
| `vendor_name` | string | yes | Stable lowercase identifier used in spans and metrics |
| `endpoint` | string | yes | Base URL of the vendor's extraction API |
| `method` | `GET` or `POST` | yes | HTTP method |
| `auth` | mapping | yes | Authentication scheme (see below) |
| `request` | mapping | yes | Request shape (see below) |
| `response` | mapping | yes | Response decoding + cost extraction (see below) |
| `status_mapping` | array | no | Per-status-code error overrides (see below) |

All string values support `${ENV_VAR_NAME}` interpolation.

---

### `auth`

#### `kind: bearer`

```yaml
auth:
  kind: bearer
  token: "${MY_API_TOKEN}"
```

Adds `Authorization: Bearer <token>` to every request.

#### `kind: basic_username`

```yaml
auth:
  kind: basic_username
  username: "${MY_API_KEY}"
```

HTTP Basic Auth with the key as the username and an empty password. Used by Zyte.

#### `kind: header`

```yaml
auth:
  kind: header
  name: "Spb-Api-Key"
  value: "${SCRAPINGBEE_KEY}"
```

Adds a custom request header.

#### `kind: query_param`

```yaml
auth:
  kind: query_param
  name: "api_key"
  value: "${MY_KEY}"
```

Appends `?api_key=<value>` to the request URL.

#### `kind: none`

```yaml
auth:
  kind: none
```

No authentication.

---

### `request`

```yaml
request:
  body:              # optional; POST requests only
    kind: json
    template: '{"url": "{{url}}", "format": "raw"}'
  query:             # optional; fixed query params appended before url_param
    - name: render_js
      value: "true"
  url_param:
    kind: query_param   # or body_field
    name: url           # required for query_param
```

#### `url_param.kind: query_param`

Appends `?url=<url-encoded-target>` to the request URL.

#### `url_param.kind: body_field`

Substitutes `{{url}}` in the JSON body template with the target URL.

---

### `response`

```yaml
response:
  kind:
    kind: raw_body          # or json_field
  cost_extraction:
    kind: static            # or none, header, json_field
  fallback_cost_usd: 0.003
```

#### `response.kind.kind: raw_body`

The response body is used directly as the page HTML.

#### `response.kind.kind: json_field`

```yaml
response:
  kind:
    kind: json_field
    html_field: browserHtml
```

The response body is JSON; extract `html_field` as a top-level string.

#### `cost_extraction.kind: none`

Cost is never extracted; `fallback_cost_usd` is ignored.

#### `cost_extraction.kind: static`

```yaml
cost_extraction:
  kind: static
fallback_cost_usd: 0.003
```

Use `fallback_cost_usd` for every request. Appropriate for vendors that do not surface per-request cost.

#### `cost_extraction.kind: header`

```yaml
cost_extraction:
  kind: header
  name: "x-request-cost"
  currency:
    kind: usd           # or credits
```

Read cost from a response header. Use `currency.kind: credits` with `conversion_rate_to_usd: 0.001` for credit-denominated vendors.

#### `cost_extraction.kind: json_field`

```yaml
cost_extraction:
  kind: json_field
  field: "cost"
```

Read cost from a top-level JSON field in the response body.

---

### `status_mapping`

```yaml
status_mapping:
  - http: 401
    error: unauthorized
    message: "my_vendor auth failure"   # optional; defaults to "<vendor> status <code>"
  - http: 429
    error: rate_limited
  - http: 402
    error: unauthorized
```

Supported `error` values: `unauthorized`, `rate_limited`, `server_error`, `bad_request`.

Entries are checked before the default mapping (401–403 → unauthorized, 429 → rate_limited, 5xx → server_error).
