use actix_web_actors::ws;

use crate::domain::messages::connect::ConnectMessage;
use crate::domain::messages::connected::ConnectedMessage;
use crate::domain::messages::ping::PingMessage;
use crate::domain::messages::pong::PongMessage;
use crate::domain::messages::sync::SyncMessage;
use crate::MyWs;
/*
use domain::messages::error::{UnkownMessageErrorMessage, WrongFormatErrorMessage};
use domain::messages::sync::SyncMessage;
use domain::messages::synced::SyncedMessage;
use serde_json::Value;
*/

pub fn middleware_connect(_ctx: &mut ws::WebsocketContext<MyWs>, _msg: &ConnectMessage) {
    // _ctx.text(serde_json::to_string(&vec!["test"]).unwrap());
}

pub fn middleware_connected(_ctx: &mut ws::WebsocketContext<MyWs>, _msg: &ConnectedMessage) {
    // _ctx.text(serde_json::to_string(&vec!["test"]).unwrap());
}

pub fn middleware_pong(_ctx: &mut ws::WebsocketContext<MyWs>, _msg: &PongMessage) {
    // _ctx.text(serde_json::to_string(&vec!["test"]).unwrap());
}

pub fn middleware_ping(_ctx: &mut ws::WebsocketContext<MyWs>, _msg: &PingMessage) {
    // ctx.text(serde_json::to_string(&vec!["test"]).unwrap());
}

pub fn middleware_sync(_ctx: &mut ws::WebsocketContext<MyWs>, _msg: &SyncMessage) {
    info!("Sync middleware on");
    let actions_iter = _msg.actions.chunks_exact(2);
    for x in actions_iter {
        info!("{:?}", x[0]);
        info!("{:?}", x[1]);
    }
    // ctx.text(serde_json::to_string(&vec!["test"]).unwrap());
}
