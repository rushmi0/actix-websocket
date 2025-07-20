/*use actix_web::{get, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub(crate) async fn relay_info(req: HttpRequest) -> impl Responder {
    let accept_header = req
        .headers()
        .get("Accept")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    if accept_header != "application/nostr+json" {
        return HttpResponse::NotAcceptable().finish();
    }

    let data = json!({
        "name": "Fenrir-s",
        "description": "Test Fenrir-s by notoshi",
        "pubkey": "03742c205cb6c8d86031c93bc4a9b3d18484c32c86563fc0e218910a2df9aa5d",
        "contact": "admin@notoshi.win",
        "supported_nips": [1, 2, 4, 9, 11, 13, 15, 28, 45, 50],
        "icon": "https://i.imgur.com/dwLPgio.png",
        "software": "https://github.com/rushmi0/Fenrir-s",
        "version": "1.0.1",
        "limitation": {
            "max_filters": 18,
            "max_limit": 256,
            "max_message_length": 524288,
            "payment_required": false,
            "auth_required": false,
        }
    });

    HttpResponse::Ok().json(data)
}
*/