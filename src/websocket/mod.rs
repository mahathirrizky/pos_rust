pub mod broadcaster;
pub mod session;

use actix::Addr;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::websocket::broadcaster::Broadcaster;
use crate::websocket::session::WsConn;

#[get("/ws")]
pub async fn start_ws_connection(
    req: HttpRequest,
    stream: web::Payload,
    broadcaster: web::Data<Addr<Broadcaster>>,
) -> Result<HttpResponse, Error> {
    let broadcaster_addr = broadcaster.get_ref().clone();
    ws::start(WsConn::new(broadcaster_addr), &req, stream)
}
