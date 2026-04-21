## WhatsApp Analyzer - Next Steps Roadmap

End goal: users upload a WhatsApp ZIP from the frontend, backend analyzes it, and returns structured stats JSON.

---

## 1) Stabilize the Core Analysis Engine (before adding HTTP)

### 1.1 Extract one reusable entry point

- Create a pure function in the library layer (not in `main`) that becomes the single analysis pipeline:
  - Input: ZIP bytes or a readable stream
  - Output: `WhatsResult`
- Suggested shape:
  - `analyze_zip_bytes(bytes: &[u8]) -> Result<WhatsResult, AppError>`
- Why: this lets both CLI and API call the exact same logic.

### 1.2 Improve parser robustness

- In `read.rs`, support common WhatsApp export variations:
  - 12h vs 24h time formats
  - Different date separators/order if needed
  - System lines ("Messages and calls are end-to-end encrypted")
  - Multi-line messages
- Keep regex parsing isolated and testable.

### 1.3 Add explicit app error types

- Introduce `AppError` enum (using `thiserror`) with variants like:
  - invalid ZIP
  - missing chat text file
  - parse failure
  - empty chat
- Return `Result<T, AppError>` across read/parse/compute layers.

### 1.4 Make response models serializable

- Add `serde` + `serde_json`.
- Derive `Serialize` (and optionally `Deserialize`) for:
  - `Chat`, `User`, `UserStat`, `WhatsResult`
- Why: API endpoints can return JSON directly.

### 1.5 Add tests around current behavior

- Unit tests:
  - line parsing
  - user aggregation
  - busiest day/month
  - average message length
- Integration tests with small sample chat fixtures (plain `.txt` and zipped).

---

## 2) Turn the Project into API + CLI

### 2.1 Keep CLI, add web server binary

- Keep current CLI as `src/bin/main.rs` (good for local debugging).
- Add new binary for server, for example `src/bin/server.rs`.

### 2.2 Add backend dependencies

- Add web stack:
  - `axum`
  - `tokio` (full)
  - `tower-http` (CORS, trace)
  - `serde`, `serde_json`
  - `thiserror`
  - `tracing`, `tracing-subscriber`

### 2.3 Define API contract first

- Request:
  - `POST /api/v1/analysis`
  - `multipart/form-data` with field `file` (ZIP)
- Response (success):
  - `200 OK` + `WhatsResult` JSON
- Response (error):
  - structured error JSON with code + message

---

## 3) Implement Upload + Analysis Endpoint

### 3.1 Add upload handler

- Parse multipart upload.
- Validate:
  - file present
  - `.zip` extension/content type
  - max size limit (example: 10MB or 25MB)

### 3.2 Wire endpoint to core engine

- Read uploaded bytes.
- Call `analyze_zip_bytes`.
- Return JSON result.

### 3.3 Map internal errors to HTTP status

- Suggested mapping:
  - bad input -> `400 Bad Request`
  - unsupported format -> `422 Unprocessable Entity`
  - unexpected issue -> `500 Internal Server Error`

### 3.4 Add middleware essentials

- CORS policy allowing your frontend origin.
- Request timeout.
- Request body limit.
- Structured tracing logs.

---

## 4) Frontend Integration Path

### 4.1 Simple upload flow

- Frontend sends multipart form with ZIP file.
- Shows loading state while waiting.
- Renders returned stats on success.
- Shows server error messages on failure.

### 4.2 Keep output schema stable

- Treat API response as a versioned contract.
- If response shape changes later, release under `/api/v2/...` or add compatibility fields.

### 4.3 Add endpoint docs

- Write concise API docs in README:
  - endpoint
  - request example
  - response example
  - error codes

---

## 5) Optional but High-Value Next Steps

### 5.1 Background job model (for larger files)

- `POST /api/v1/analysis` returns `job_id` quickly.
- `GET /api/v1/analysis/{job_id}` returns job status/result.
- Good if analysis time grows.

### 5.2 Persist analysis history

- Save result summaries + timestamp in DB (SQLite/Postgres).
- Add `GET /api/v1/history` for user dashboard.

### 5.3 Auth and per-user isolation

- Add login/session/JWT.
- Ensure each user sees only their own analyses.

---

## 6) Suggested Implementation Order (Do This First)

1. Refactor current code into one reusable `analyze_zip_bytes` pipeline.
2. Add `AppError` and `serde` derives on models.
3. Add parser/stats tests with fixture files.
4. Add `server.rs` with `POST /api/v1/analysis` upload endpoint.
5. Add CORS + body size limits + tracing.
6. Connect frontend upload form to the endpoint.
7. Add README API examples and error code table.

---

## 7) Definition of Done for Your Goal

You are done when:

- frontend can upload a WhatsApp ZIP,
- backend returns structured JSON analysis,
- invalid files are handled with clear error responses,
- and the same core analysis logic is covered by automated tests.
