# Rust CLI design decisions

Major decisions to think through, roughly ordered by how much they lock you in.

## 1. Crate layout: library + thin CLI, from day one

Put everything in a `data-dict` core crate and make the `data-dict-cli` binary a thin wrapper. You'll almost certainly want R/Python bindings later (extendr, pyo3) and a WASM build for web playgrounds. Retrofitting this split is painful.

## 2. YAML parser: span-preserving, not `serde_yaml`

`serde_yaml` is ergonomic but throws away line/column info the moment you deserialize — which makes good validation errors impossible. Options: `saphyr` (modern fork of yaml-rust2) or `marked-yaml` to keep spans, then lower into your typed model yourself. More code, but error quality is the whole point of a validator. Pair this with `miette` for the "squiggle under the offending token" reports — this is where the CLI earns its keep over ad hoc scripts.

## 3. Two-pass model: parse → lower → validate

Keep the parsed representation close to the YAML (with spans), then *lower* it into a normalized internal model (e.g. `number(id)` becomes `{kind: Number, measure: Id}`, enum `values` lists vs maps are unified). Validation runs on the lowered model but reports errors with spans from the parsed layer. This cleanly separates "is the YAML well-formed against the spec" from "does the data match the schema", and lets you reuse the normalized model for export, site gen, and bindings.
  
## 4. Source abstraction for data validation

Define a `Source` trait that yields an Arrow `RecordBatch` stream. Start with `parquet` only (via the `parquet` + `arrow` crates). `SQL` is tractable later via `connectorx` or per-driver crates but opens a dependency can of worms. `R` / `Python` / `pin` sources are probably not validatable from a Rust CLI at all — decide now whether to (a) skip them with a clear "unvalidatable source" message, or (b) shell out to R/Python. Skipping and documenting is safer; shelling out is a support nightmare.

## 5. Validation: collect, don't fail-fast; cap the output

Users want to see every error on one run, but an 80-column-wide schema with all-wrong data can produce thousands. Accumulate errors into a typed report (serializable to JSON so editors/CI can consume it), render to the terminal with a configurable limit, and exit non-zero if any errors. This also makes LSP-style integration trivial later.

## 6. Export / site generation: pick a templating story early

`minijinja` (runtime templates, Jinja-compatible, great errors) vs `askama` (compile-time, fast, less flexible). For a static site generator that users might theme, minijinja wins. For JSON Schema / Markdown export, hand-write serializers. Don't build a plugin system until you have two real consumers asking for it.

## 7. Spec versioning

Add a top-level `version: 1` field to `data-dict.yaml` *now*, before anyone has files in the wild. The CLI reads it, warns on unknown versions, and you get a clean lever for breaking changes. Cheap to add, impossible to add later.

## 8. Distribution

`cargo install` + prebuilt binaries from GitHub Actions (use `cargo-dist`) covers 90%. Homebrew tap is cheap once you have releases. Skip an npm wrapper unless the JS ecosystem is an actual target audience.
