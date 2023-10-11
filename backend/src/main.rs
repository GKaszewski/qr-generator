use std::collections::HashMap;

use axum::{Router, routing::get, extract::Query, response::IntoResponse, http::{header::CONTENT_TYPE, StatusCode}};
use image::{Luma, png::PngEncoder, ColorType};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index)).route("/qr", get(get_qr_code));
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn index() -> &'static str {
    "Hello, world!"
}

async fn get_qr_code(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let link = match params.get("link") {
        Some(l) => l,
        None => {
            return (StatusCode::BAD_REQUEST, [(CONTENT_TYPE, "text/plain")], "Missing link").into_response();
        }
    };

    let qr_code = match qrcode::QrCode::new(link) {
        Ok(qr) => qr,
        Err(_) => return (StatusCode::BAD_REQUEST, [(CONTENT_TYPE, "text/plain")], "Invalid link").into_response(),
    };

    let qr_image = qr_code.render::<Luma<u8>>().build();
    let mut buffer: Vec<u8> = Vec::new();
    let width = qr_image.width();
    let height = qr_image.height();
    let encoder = PngEncoder::new(&mut buffer);
    match encoder.encode(&qr_image.into_raw(), width, height, ColorType::L8) {
        Ok(_) => (),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, [(CONTENT_TYPE, "text/plain")], "Failed to encode QR code").into_response(),
    }
    (StatusCode::OK, [(CONTENT_TYPE, "image/png")], buffer).into_response()
}