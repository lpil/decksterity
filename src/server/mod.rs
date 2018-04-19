use env_logger;
use actix::prelude::*;
use actix_web::{fs, http, middleware, server, ws, App, Error, HttpRequest, HttpResponse};

struct WebSocketActor;

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WebSocketActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => ctx.stop(),
            _ => (),
        }
    }
}

pub fn start() {
    let sys = actix::System::new("ws-example");

    let handler = || {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/ws/", |r| {
                r.method(http::Method::GET)
                    .f(|r| ws::start(r, WebSocketActor))
            })
            .handler("/", fs::StaticFiles::new("dist/").index_file("index.html"))
    };

    server::new(handler).bind("127.0.0.1:1972").unwrap().start();

    let _ = sys.run();
}
