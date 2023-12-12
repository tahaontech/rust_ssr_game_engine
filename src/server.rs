use crate::engine::{Engine, EngineState, image_to_base64};

// use crate::messages::{ClientactorMessage, Connect, Disconnect, WsMessage};

use std::time::Duration;

use actix::prelude::*;
use actix_web_actors::ws;
use image::{ImageBuffer, Rgba, DynamicImage};



// it should be tuned for the game engine its 10 fps now change it to 17 to get 60 fps
const FRAME_INTERVAL: Duration = Duration::from_millis(100); 

struct FrameMessage {
    frame: String
}

impl FrameMessage {
    fn to_string(&self) -> String {
        self.frame.clone()
    }

    fn from_image_buffer(img_buff: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        let base_img: DynamicImage = img_buff.into();
        let frame: String = image_to_base64(&base_img);
        FrameMessage { frame: frame }
    }
}


/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    engine: Engine
}

impl MyWebSocket {
    pub fn new(img: &DynamicImage) -> Self {
        Self { engine: Engine::new(img.clone()) }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {

        ctx.run_interval(FRAME_INTERVAL, |my_self, ctx| {
            let buffered_image = my_self.engine.next_frame();
            let base64_data = FrameMessage::from_image_buffer(buffered_image).to_string();
            let new_data: &str = &base64_data;
            
            ctx.text(new_data);
        });
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Text(text)) => {
                if text == "left" {
                    self.engine.change_state(EngineState::LEFT);
                    // ctx.text("change state to left")
                } else if text == "right" {
                    self.engine.change_state(EngineState::RIGHT);
                    // ctx.text("change state to right")
                } else if text == "up" {
                    self.engine.change_state(EngineState::UP);
                    // ctx.text("change state to up")
                } else if text == "down" {
                    self.engine.change_state(EngineState::DOWN);
                    // ctx.text("change state to down")
                } else {
                    ctx.text("wrong game key")
                }
            },
            Ok(ws::Message::Ping(msg)) => {
                
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}