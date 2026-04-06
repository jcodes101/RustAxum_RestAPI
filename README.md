# rust_axum_restapi

A small learning-oriented REST API built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/). It exposes a few GET routes that return plain text or JSON for a sample list of documents, including a filtered view of compliant documents.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with support for **edition 2024** (a recent stable toolchain, e.g. 1.85 or newer).

## Run the server

From the project root:

```bash
cargo run
```

The server listens on **http://127.0.0.1:3000** and prints a short message when it starts.

## API

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Health-style message: `API is running` |
| `GET` | `/documents` | JSON array of all sample documents |
| `GET` | `/documents/compliant` | JSON array of documents where `compliant` is `true` |

### Example responses

**`GET /`**

```text
API is running
```

**`GET /documents`** — JSON body shape:

```json
[
  { "id": 1, "title": "Doc A", "compliant": true },
  { "id": 2, "title": "Doc B", "compliant": false },
  { "id": 3, "title": "Doc C", "compliant": true }
]
```

**`GET /documents/compliant`** — same structure, only entries with `"compliant": true`.

### Try it

```bash
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/documents
curl http://127.0.0.1:3000/documents/compliant
```

## Project layout

| Path | Role |
|------|------|
| `Cargo.toml` | Crate metadata and dependencies |
| `src/main.rs` | Application entrypoint, routes, and handlers |
| `src/notes.md` | Optional local notes (not required to run the app) |

## Stack

- **axum** — HTTP router and server integration
- **tokio** — Async runtime (`#[tokio::main]`, TCP listener, `axum::serve`)
- **serde** — `Serialize` for JSON responses
- **tower** — Middleware ecosystem (pulled in for typical Axum stacks)
- **tracing** / **tracing-subscriber** — Structured logging (available for future use)

Data in handlers is currently **in-memory and static**; there is no database or persistence.

## License

If you add a license file, describe it here. Until then, treat licensing as unspecified.
