# Test_AI_WASI — Edge ML embedding (Spin / WASI)

Spin HTTP component: POST an image → get a **512-dim embedding** (CLIP-style vision encoder via [tract-onnx](https://github.com/sonos/tract) in Rust/WASM). Reimplementation of the **Rust ML inference fingerprinting** demo from the [Fermyon/Adobe talk](https://www.youtube.com/watch?v=KGNNE_imU5g&t=2817s); see also [WASM for AI panel](https://www.youtube.com/watch?v=UODo4Q5ykd8).

**Inference runs in Rust** inside the WASM component — no Python or external ML service.

---

## Context (from talks)

- **Languages:** Python dominates prototyping; production inference is often C/C++ (llama.cpp, vLLM) or **Rust** for performance and **memory safety**. Many models see higher usage in GGUF/native runtimes than in Python-only form.
- **WASM for AI:** WebAssembly abstracts hardware and model differences behind one runtime/bytecode — “write once, run anywhere” for inference at the edge, in browsers (e.g. WebGPU + WASM), or in serverless.
- **Prod use cases:** Local coding assistants (private code), pipelines (e.g. LlamaEdge processing large video volumes), client-side inference for privacy-sensitive apps, and existing products (e.g. Photoshop, Meet) using WASM for real-time image/background features.
- **Rule of thumb:** Use WASM when you need sandboxed, portable, fast-start code — AI inference, RAG glue, serverless/agent tools.

**Demos from the Adobe/Spin talk:** (1) **Background removal** — Java microservice reimplemented in Rust/WASM; ~25 ms end-to-end with Spin. (2) **ML fingerprinting** (this repo) — image → embedding “fingerprint” for Content Authenticity; in the talk ~1 s per request with naïve per-request model load; we **cache** the runnable model so only the first request pays load cost.

---

## Project layout (matches this repo)

| Path | Role |
|------|------|
| `spin.toml` | Spin 2 app manifest; HTTP trigger `/embed`, component `embedder` |
| `model.onnx` | CLIP vision encoder (~50–100 MB), embedded at compile time. **Download:** [Qdrant/clip-ViT-B-32-vision](https://huggingface.co/Qdrant/clip-ViT-B-32-vision) (Hugging Face) — get the ONNX file and place as `model.onnx` in this directory. |
| `src/lib.rs` | HTTP handler: decode image → preprocess 224×224 → run Tract → return JSON |
| `Cargo.toml` | spin-sdk 3, tract-onnx 0.22, image, serde, anyhow, once_cell; `[patch]` for tract-onnx |
| `patches/tract-onnx/` | Patched Pad op etc. if needed for your ONNX |

Target: **wasm32-wasip1**. Build: `cargo build --target wasm32-wasip1 --release` (or `spin build`).

---

## Handler (aligned with current code)

- **Model:** Loaded once via `once_cell::sync::OnceCell`; `get_model()` returns the cached `SimplePlan`. No per-request load.
- **Request:** POST body = raw image bytes (JPEG/PNG/BMP).
- **Preprocess:** Resize to 224×224, CLIP normalisation (mean/std), NCHW tensor `[1, 3, 224, 224]` f32.
- **Inference:** `model.run(tvec!(input_tensor))` → one output tensor.
- **Response:** JSON `{ "shape": [1, 512], "embedding": [f32, ...] }`.

Pseudo-flow (matches `lib.rs`):

```rust
static MODEL_BYTES: &[u8] = include_bytes!("../model.onnx");
static MODEL: OnceCell<RunnableModel> = OnceCell::new();

fn get_model() -> Result<&'static RunnableModel, _> {
    MODEL.get_or_try_init(|| {
        tract_onnx::onnx()
            .model_for_read(&mut std::io::Cursor::new(MODEL_BYTES))?
            .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 3, 224, 224)))?
            .into_optimized()?
            .into_runnable()
    })
}

#[http_component]
fn handle_embed(req: Request) -> Result<impl IntoResponse, _> {
    let img_bytes = req.into_body();
    let input_tensor = preprocess(image::load_from_memory(&img_bytes)?); // 224×224, CLIP norm
    let model = get_model()?;
    let outputs = model.run(tvec!(input_tensor.into()))?;
    let embedding: Vec<f32> = outputs[0].to_array_view::<f32>()?.iter().cloned().collect();
    Ok(json_response(200, serde_json::to_vec(&json!({ "shape": [1, 512], "embedding": embedding }))?))
}
```

---

## What to measure (for the paper)

| Metric | How | This repo |
|--------|-----|-----------|
| Model file size | `ls -lh model.onnx` | ~50–100 MB depending on encoder |
| Cold (first request after `spin up`) | `time curl -X POST … --data-binary @img.jpg` | **~1 min** (one-time load + optimize + compile, then inference) |
| Warm (later requests) | Same curl again | **~4.4 s** on this machine (sample run: `time curl … image.jpeg` reported `4.356 total`; dominated by decode + preprocess + inference with the model cached in `OnceCell`) |

Without caching, every request would pay the ~1 min cost; the talk’s “about 1 second” was for a different setup. Here we cache the optimized/runnable model so only the first request pays the heavy load/compile cost; subsequent requests are on the order of a few seconds.

---

## Typical next steps with the embedding

- **Visual search:** Top-k similar images (“search by image”, similar stock photos).
- **Recommendations:** “Users who viewed this also liked…” via embedding proximity.
- **Tagging / clustering:** Group by embedding distance; suggest or assign categories at the edge.
- **Deduplication / filtering:** Detect near-duplicates; hide or merge.
- **Downstream ML / analytics:** Store embeddings and run offline models (anomaly detection, quality scoring, trends) in the cloud later.

---

## Prerequisites, build, usage

- **Spin:** [Install](https://developer.fermyon.com/spin/install) (e.g. `brew install fermyon/tap/spin` — not the PROMELA/SPIN binary).
- **Rust target:** `rustup target add wasm32-wasip1`.

```bash
cd experiments/Test_AI_WASI
spin build && spin up
# → http://127.0.0.1:3000
curl -X POST http://localhost:3000/embed --data-binary @image.jpg -H "Content-Type: image/jpeg"
```

Response: `{ "shape": [1, 512], "embedding": [ ... ] }`.

---

## Limitations

- First request ~1 min; use `OnceCell` caching (as in this code) so warm requests are ~2–3 s.
- Large model in WASM memory; allow enough for Spin/Wasmtime (e.g. 256 MB+).
- Pad/other ONNX ops may require the patched tract-onnx; if you see panics, `cargo clean && spin build`.
- Spin “guest invocation failed” = panic inside the WASM handler or a dependency; check the backtrace.
