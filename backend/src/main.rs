use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::Query,
    http::{header::CONTENT_TYPE, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use image::{png::PngEncoder, ColorType, Luma};
use maud::{html, Markup};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/qr", get(get_qr_code))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET]),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn index() -> Markup {
    html! {
        html {
            head {
                title { "QR Code Generator" }
            }
            body {
                h1 { "QR Code Generator" }
                form action="/qr" method="get" {
                    label for="link" { "Value: " }
                    input type="text" name="link" id="link" required;
                    input type="submit" value="Generate QR Code";
                }
            }
        }
    }
}

async fn get_qr_code(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let link = match params.get("link") {
        Some(l) => l,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                [(CONTENT_TYPE, "text/plain")],
                "Missing link",
            )
                .into_response();
        }
    };

    let qr_code = match qrcode::QrCode::new(link) {
        Ok(qr) => qr,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                [(CONTENT_TYPE, "text/plain")],
                "Invalid link",
            )
                .into_response()
        }
    };

    let qr_image = qr_code.render::<Luma<u8>>().build();
    let mut buffer: Vec<u8> = Vec::new();
    let width = qr_image.width();
    let height = qr_image.height();
    let encoder = PngEncoder::new(&mut buffer);
    match encoder.encode(&qr_image.into_raw(), width, height, ColorType::L8) {
        Ok(_) => (),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(CONTENT_TYPE, "text/plain")],
                "Failed to encode QR code",
            )
                .into_response()
        }
    }
    (StatusCode::OK, [(CONTENT_TYPE, "image/png")], buffer).into_response()
}
