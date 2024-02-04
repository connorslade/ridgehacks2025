use afire::{extensions::RouteShorthands, Content, Server};
use serde::Deserialize;
use serde_json::json;
use tracing::info;

use crate::app::App;

#[derive(Deserialize)]
struct Request {
    endpoint: String,
}

pub fn attach(server: &mut Server<App>) {
    server.post("/api/subscribe", |ctx| {
        let request = serde_json::from_slice::<Request>(&ctx.req.body)?;
        info!("New subscriber: {}", request.endpoint);
        ctx.app().database.add_subscriber(&request.endpoint)?;
        ctx.text(json!({"status": "ok"}))
            .content(Content::JSON)
            .send()?;
        Ok(())
    });
}
