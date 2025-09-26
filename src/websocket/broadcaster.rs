use actix::prelude::*;
use std::collections::HashMap;

/// Pesan broadcast yang akan dikirim ke semua klien.
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

/// Pesan untuk terhubung ke broadcaster.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<BroadcastMessage>,
    pub id: usize,
}

/// Pesan untuk memutus koneksi dari broadcaster.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Aktor Broadcaster
#[derive(Debug, Default)]
pub struct Broadcaster {
    sessions: HashMap<usize, Recipient<BroadcastMessage>>,
}

impl Actor for Broadcaster {
    type Context = Context<Self>;
}

impl Handler<Connect> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        println!("Broadcaster: New client connected with id {}", msg.id);
        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        println!("Broadcaster: Client {} disconnected", msg.id);
        self.sessions.remove(&msg.id);
    }
}

impl Handler<BroadcastMessage> for Broadcaster {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Self::Context) {
        println!("Broadcaster: Broadcasting message to all clients");
        for addr in self.sessions.values() {
            let _ = addr.do_send(BroadcastMessage(msg.0.clone()));
        }
    }
}
