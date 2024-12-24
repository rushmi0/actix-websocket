use actix_web::{HttpRequest, Responder, web};
use actix_ws::Message;
use futures_util::StreamExt;
use log::{error, info};
use miniserde::{json, Deserialize, Serialize};

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

pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    match json::from_str::<WSPayload>(&text) {
                        Ok(payload) => {
                            info!("Received payload: {:?}", &payload);

                            // สร้าง EventRes ด้วย miniserde
                            let json_response = EventRes {
                                status: "Ok",
                                payload: &payload,
                            };

                            // แปลง EventRes เป็น JSON string
                            let data = json::to_string(&json_response);

                            if let Err(e) = session.text(data).await {
                                error!("Failed to send response: {:?}", e);
                                return;
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse payload: {:?}", e);
                        }
                    }
                }
                Message::Close(reason) => {
                    info!("WebSocket closed: {:?}", reason);
                    break;
                }
                _ => {}
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}
