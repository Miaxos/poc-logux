#[macro_use]
extern crate log;

mod domain;
mod infrastructure;
mod middleware;

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use domain::messages::lib::LoguxEvent;
use domain::messages::connect::{decode_connect_message};
use domain::messages::ping::decode_ping_message;
use domain::messages::pong::{PongMessage, decode_pong_message};
use domain::messages::connected::{ConnectedMessage, decode_connected_message, OptionnalConnectedMessage};
use domain::messages::error::{UnkownMessageErrorMessage, WrongFormatErrorMessage};
use domain::messages::sync::decode_sync_message;
use domain::messages::synced::decode_synced_message;
use infrastructure::logger::ConfigLogger;
use log::LevelFilter;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use middleware::{middleware_sync, middleware_connect, middleware_connected, middleware_pong, middleware_ping};

#[allow(clippy::cognitive_complexity)]
fn process_action(
    vec: std::vec::Vec<Value>,
    ctx: &mut ws::WebsocketContext<MyWs>,
) -> Option<serde_json::Result<String>> {
    match vec.get(0) {
        Some(Value::String(action_type)) => match action_type.as_ref() {
            "error" => {
                error!("{:?}", vec);
                Some(serde_json::to_string(&vec!["error", "wrong-format"]))
            }

            // After a connected is received, check if client got right to connect
            "connect" => {
                info!("Connect message received: {:?}", vec);

                let receive_date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                match decode_connect_message(&vec) {
                    Ok(val) => {
                        debug!("Connect message successfully decoded.");
                        middleware_connect(ctx, &val);

                        // Create connected message
                        let start = SystemTime::now();
                        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
                        Some(Ok(ConnectedMessage {
                        protocol: val.protocol,
                        time_sync: [receive_date-1, since_the_epoch.as_millis() as u64],
                        node_id: "server:sb7VjwpO".to_string(),
                        options: match val.options {
                            Some(options) => Some(OptionnalConnectedMessage {
                                credentials: options.credentials,
                                subprotocol: options.subprotocol,
                            }),
                            None => None,
                        },
                        }.encode()))
                    },
                    Err(e) => {
                        error!("Connected message malformed: {}", e);
                        Some(Ok(e.to_string()))
                    },
                }
                
            }

            "connected" => {
                info!("Connected message received: {:?}", vec);

                match decode_connected_message(&vec) {
                    Ok(val) => {
                        debug!("Connected message successfully decoded.");
                        middleware_connected(ctx, &val);
                        None
                    },
                    Err(e) => {
                        error!("Connected message malformed: {}", e);
                        Some(Ok(e.to_string()))
                    },
                }
            }

            // After a ping message is received by the server, send back a pong message.
            "ping" => {
                info!("Ping message received: {:?}", vec);

                match decode_ping_message(&vec) {
                    Ok(val) => {
                        debug!("Ping message successfully decoded.");
                        middleware_ping(ctx, &val);
                        Some(Ok(PongMessage {
                            synced: val.synced,
                        }.encode()))
                    },
                    Err(e) => {
                        error!("Pong message malformed: {}", e);
                        Some(Ok(e.to_string()))
                    },
                }
            }
            // After a pong message is received by the client, keep the connection on.
            "pong" => {
                info!("Pong message received: {:?}", vec);
                // TODO: After decoding, do protocol
                match decode_pong_message(&vec) {
                    Ok(val) => {
                        debug!("Pong message successfully decoded.");
                        middleware_pong(ctx, &val);
                        None
                    },
                    Err(e) => {
                        error!("Pong message malformed: {}", e);
                        Some(Ok(e.to_string()))
                    },
                }
            }
            "sync" => {
                info!("Sync message received: {:?}", vec);
                match decode_sync_message(&vec) {
                    Ok(val) => {
                        debug!("Sync message successfully decoded.");
                        middleware_sync(ctx, &val);
                        None
                    },
                    Err(e) => {
                        error!("Sync message malformed: {}", e);
                        Some(Ok(e.to_string()))
                    },
                }
            }
            "synced" => {
                info!("Synced message received: {:?}", vec);
                match decode_synced_message(&vec) {
                    Ok(_) => {
                        debug!("Synced message successfully decoded.");
                        // middleware_pong(ctx, &val);
                        None
                    },
                    Err(e) => {
                        error!("Synced message malformed: {}", e);
                        Some(Ok(e.to_string()))
                    },
                }
            }
            "debug" => {
                info!("Debug message received: {:?}", vec);
                None
            }
            _ => {
                warn!("Weird, a bad type: {} has been sent", action_type);
                Some(Ok(UnkownMessageErrorMessage {
                    message_type: action_type.to_string(),
                }
                .to_string()))
            }
        },
        None => Some(Ok(
            WrongFormatErrorMessage {
                message: String::from("array is empty"),
            }.to_string())),
        _ => Some(Ok(
            WrongFormatErrorMessage {
                message: String::from("incorrect format, please refer to: https://github.com/logux/logux/blob/master/protocol/spec.md"),
            }.to_string())),
    }
}

/// Define http actor
pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWs {
    // We should just provide this handle function with the same arguments so
    // users can get their own actix_server running
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            // Websocket ? To check if we delete it or not ?
            ws::Message::Ping(msg) => {
                info!("{}", &msg);
                ctx.pong(&msg);
            }
            ws::Message::Text(text) => {
                info!(" {}", &text);
                match serde_json::from_str::<Value>(&text) {
                    Ok(Value::Array(val)) => match process_action(val, ctx) {
                        Some(Ok(message)) => ctx.text(message),
                        Some(Err(e)) => error!("Shit happened: {}", e),
                        _ => (),
                    },
                    Err(_) => {
                        ctx.text(
                            WrongFormatErrorMessage {
                                message: String::from("incorrect format, please refer to: https://github.com/logux/logux/blob/master/protocol/spec.md"),
                            }.to_string()
                        );
                    }
                    _ => (),
                }
            }
            _ => {
                ctx.text(
                    serde_json::to_string(&vec!["error", "wrong-format", "not an array"]).unwrap(),
                );
            }
        }
    }
}

fn index(req: HttpRequest, stream: web::Payload) -> std::result::Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    info!("{:?}", resp);
    resp
}

fn main() {
    // Start logger
    let _logger = ConfigLogger::init(LevelFilter::Debug);

    info!("Starting logtux-rust");
    info!("Listening to 127.0.0.1:8088");
    HttpServer::new(move || App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
