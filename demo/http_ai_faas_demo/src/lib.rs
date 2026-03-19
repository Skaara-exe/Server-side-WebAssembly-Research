use once_cell::sync::OnceCell;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use tract_onnx::prelude::*;

static MODEL_BYTES: &[u8] = include_bytes!("../model.onnx");
const IMG_SIZE: u32 = 224;

// CLIP normalisation (OpenAI), 0–255 range. 0–1 equivalents: mean ≈ [0.48, 0.46, 0.41], std ≈ [0.27, 0.26, 0.28].
const PIXEL_MEAN: [f32; 3] = [122.7709, 116.7460, 104.0937];
const PIXEL_STD: [f32; 3] = [68.5005, 66.6322, 70.3232];

type RunnableModel = SimplePlan<TypedFact, Box<dyn TypedOp>, TypedModel>;
static MODEL: OnceCell<RunnableModel> = OnceCell::new();

fn get_model() -> anyhow::Result<&'static RunnableModel> {
    MODEL.get_or_try_init(|| {
        tract_onnx::onnx()
            .model_for_read(&mut std::io::Cursor::new(MODEL_BYTES))
            .map_err(|e| anyhow::anyhow!("model load failed: {e}"))?
            .with_input_fact(
                0,
                InferenceFact::dt_shape(
                    f32::datum_type(),
                    tvec!(1, 3, IMG_SIZE as i64, IMG_SIZE as i64),
                ),
            )
            .map_err(|e| anyhow::anyhow!("model input fact failed: {e}"))?
            .into_optimized()
            .map_err(|e| anyhow::anyhow!("model optimize failed: {e}"))?
            .into_runnable()
            .map_err(|e| anyhow::anyhow!("model compile failed: {e}"))
    })
}

/// POST /embed: body = raw image bytes (JPEG/PNG/BMP). Response = JSON with 512-dim CLIP vision embedding.
#[http_component]
fn handle_embed(req: Request) -> anyhow::Result<impl IntoResponse> {
    let body = req.into_body();
    if body.is_empty() {
        return Ok(json_response(
            400,
            serde_json::json!({"error": "request body is empty; POST a JPEG, PNG, or BMP image"})
                .to_string()
                .into_bytes(),
        ));
    }
    match run_inference(&body) {
        Ok(json_bytes) => Ok(json_response(200, json_bytes)),
        Err(e) => Ok(json_response(
            500,
            serde_json::json!({"error": format!("{e:#}")})
                .to_string()
                .into_bytes(),
        )),
    }
}

// #region agent log — H3/H4: capture inference step
fn run_inference(img_bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    let img = image::load_from_memory(img_bytes)
        .map_err(|e| anyhow::anyhow!("image decode failed: {e}"))?;
    let input_tensor: Tensor = preprocess(img).into();
    let model = get_model()?;

    let outputs = model
        .run(tvec!(input_tensor.into()))
        .map_err(|e| anyhow::anyhow!("inference failed: {e}"))?;

    let view = outputs[0]
        .to_array_view::<f32>()
        .map_err(|e| anyhow::anyhow!("output is not f32: {e}"))?;
    let shape: Vec<usize> = view.shape().to_vec();
    let embedding: Vec<f32> = view.iter().cloned().collect();

    let payload = serde_json::json!({ "shape": shape, "embedding": embedding });
    Ok(serde_json::to_vec(&payload)?)
}

/// Resize to 224×224, CLIP normalise, NCHW [1, 3, 224, 224].
fn preprocess(img: image::DynamicImage) -> tract_ndarray::Array4<f32> {
    let resized = img
        .resize_exact(IMG_SIZE, IMG_SIZE, image::imageops::FilterType::Lanczos3)
        .to_rgb8();
    tract_ndarray::Array4::from_shape_fn(
        (1, 3, IMG_SIZE as usize, IMG_SIZE as usize),
        |(_, c, y, x)| {
            let pixel = resized.get_pixel(x as u32, y as u32).0;
            (pixel[c] as f32 - PIXEL_MEAN[c]) / PIXEL_STD[c]
        },
    )
}

fn json_response(status: u16, body: Vec<u8>) -> Response {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build()
}
