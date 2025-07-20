// gateway.rs

use actix_web::{HttpRequest, HttpResponse, web, Error};
use actix_files::NamedFile;
use actix_web::http::header;
use actix_ws::{Message, ProtocolError};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json;
use tracing::{error, info};

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct WSPayload {
    id: String,
    created_at: u32,
    kind: u8,
    tags: Vec<Vec<String>>,
}

#[derive(Serialize)]
struct EventRes<'a> {
    status: &'a str,
    payload: &'a WSPayload,
}


pub async fn gateway(
    req: HttpRequest,
    payload: web::Payload,
    static_path: web::Data<PathBuf>,
) -> Result<HttpResponse, Error> {
    let is_ws = req
        .headers()
        .get(header::UPGRADE)
        .and_then(|h| h.to_str().ok())
        .map_or(false, |v| v.eq_ignore_ascii_case("websocket"));

    if is_ws {
        handle_websocket(req, payload).await
    } else {
        handle_http(req, static_path).await
    }
}

// ====== WebSocket handler ======
async fn handle_websocket(
    req: HttpRequest,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    let Ok(payload) = serde_json::from_str::<WSPayload>(&text) else {
                        error!("Failed to parse payload");
                        continue;
                    };

                    info!("Received payload: {:?}", &payload);

                    let json_response = EventRes {
                        status: "Ok",
                        payload: &payload,
                    };

                    let Ok(data) = serde_json::to_string(&json_response) else {
                        error!("Failed to serialize response");
                        continue;
                    };

                    if let Err(e) = session.text(data).await {
                        error!("Failed to send response: {:?}", e);
                        return;
                    }
                }
                Message::Close(reason) => {
                    info!("WebSocket closed: {:?}", reason);
                    break;
                }
                Message::Nop => {
                    info!("Received NOP message");
                }
                _ => {} // รองรับ future types ของ message
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

// ====== HTTP handler ======
async fn handle_http(
    req: HttpRequest,
    static_path: web::Data<PathBuf>,
) -> Result<HttpResponse, Error> {
    let file_path = static_path.join("index.html");

    let file = NamedFile::open(file_path)?
        .use_last_modified(false)
        .disable_content_disposition();

    Ok(file.into_response(&req))
}


/*async fn handle_http(
    req: HttpRequest,
    static_path: web::Data<PathBuf>,
) -> Result<HttpResponse, Error> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap_or_default();

    let full_path = static_path.join(&path);

    if full_path.is_file() {
        Ok(NamedFile::open(full_path)?
            .use_last_modified(true)
            .disable_content_disposition()
            .into_response(&req))
    } else {
        // fallback to index.html for SPA routes
        let index_file = static_path.join("index.html");
        Ok(NamedFile::open(index_file)?
            .use_last_modified(true)
            .disable_content_disposition()
            .into_response(&req))
    }
}
*/