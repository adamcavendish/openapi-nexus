# Response Headers

Generated clients retain response headers for both successful and non-successful
HTTP responses. Header preservation is not limited to headers declared in the
OpenAPI document: an undeclared `Retry-After`, request ID, tracing header,
cookie, or vendor header remains available through the HTTP backend's native
header collection.

OpenAPI-declared response headers additionally receive generated convenience
accessors. Simple `string`, `integer`, `number`, and `boolean` schemas map to the
corresponding language type. Accessors are always optional because a server can
omit a declared header or send a value that does not match its schema. A missing
or malformed value does not turn an otherwise valid response into an error.
Scalar parsing uses the same wire policy across generators: booleans are the
lowercase literals `true` and `false`, integers are signed decimal digits, and
numbers are finite decimal values with an optional exponent. Other spellings
return the accessor's empty result while remaining available in the native
header collection.

`Retry-After` should normally be declared as a string because HTTP allows either
delay seconds or an HTTP date:

```yaml
components:
  headers:
    RetryAfter:
      description: Delay in seconds or an HTTP date before retrying.
      schema:
        type: string

paths:
  /jobs:
    post:
      responses:
        "429":
          description: Too many requests.
          headers:
            Retry-After:
              $ref: "#/components/headers/RetryAfter"
```

## Generated access

| Generator | Successful response headers | Error response headers |
|---|---|---|
| TypeScript Fetch | raw response wrapper `headers`; generated `get...Header` functions | `ResponseError.response.headers`; generated methods on the operation error |
| Go HTTP | `response.Raw.Header`; generated `(value, ok)` methods | `error.Header`; generated `(value, ok)` methods |
| Rust reqwest, ureq, aioduct | native `HeaderMap` in `response.headers`; generated `Option<T>` methods | native `HeaderMap` from `ApiError::headers()`; generated `Option<T>` methods on the operation error |
| Python httpx, requests | `method_with_http_info(...).headers`; generated optional properties | `ApiError.headers`; generated optional properties |
| Java OkHttp | `response.getRaw().headers()`; generated nullable getters | `exception.headers()`; generated nullable getters |
| Kotlin OkHttp | `response.raw.headers`; generated nullable methods | `exception.headers`; generated nullable methods |

The native collection remains the authority when a caller needs undeclared
headers or backend-specific parsing. A generated scalar accessor reads one
value only. Duplicate-field behavior follows the HTTP backend; some backends
retain each field while others coalesce duplicate fields into one value.

## Browser visibility

TypeScript Fetch follows the browser Fetch API. For cross-origin requests,
JavaScript can only read CORS-safelisted response headers plus headers named by
the server's `Access-Control-Expose-Headers` response header. The browser may
therefore hide `Retry-After` even though the generated client does not drop it.
