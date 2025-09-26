use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::websocket::broadcaster::{BroadcastMessage, Broadcaster, Connect, Disconnect};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsConn {
    id: usize,
    hb: Instant,
    broadcaster_addr: Addr<Broadcaster>,
}

impl WsConn {
    pub fn new(broadcaster_addr: Addr<Broadcaster>) -> Self {
        Self {
            id: rand::random::<usize>(),
            hb: Instant::now(),
            broadcaster_addr,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("WsConn: Client heartbeat failed, disconnecting!");
                act.broadcaster_addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.broadcaster_addr
            .send(Connect {
                addr: addr.recipient(),
                id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                if res.is_err() {
                    ctx.stop();
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.broadcaster_addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<BroadcastMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.broadcaster_addr.do_send(BroadcastMessage(text.to_string()));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}