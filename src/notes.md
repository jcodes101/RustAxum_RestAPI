[dependencies]

axum:
    -   a web framework for rust, that can be used
    for making REST-APIs
    -   used for defining routes, and handling HTTP reqs., return responses

tokio:
    -   an async runtime
    -   used for running asynchronous code
    -   without this axum won't work, as is async

tower:
    -   middleware and abstraction library
    -   used for adding reusable layers -> logging, rate limiting, timeouts

tracing:
    -   logging framework
    -   log apps activity (requests, errors, timing, etc)

tracing-subscriber:
    -   logging output for tracing
    -   determines how logs are displayed (console, formatting, filtering)
    -   tracing alone doesn't print anything, -subscriber is needed

serde;
    -   for serializing and de-serializing Rust data structures
    -   converts Rust struct/enums into formats like JSON, YAML or binary