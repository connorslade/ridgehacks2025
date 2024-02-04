use afire::{extensions::RouteShorthands, Content, Server};
use serde::Deserialize;
use serde_json::json;
use tracing::info;

use crate::app::App;

#[derive(Deserialize)]
struct UnsubscribeRequest {
    endpoint: String,
}

pub fn attach(server: &mut Server<App>) {
    server.post("/api/unsubscribe", |ctx| {
        let body = serde_json::from_slice::<UnsubscribeRequest>(&ctx.req.body)?;
        info!("Unsubscribing: {}", body.endpoint);

        ctx.app().database.remove_subscriber(&body.endpoint)?;

        ctx.text(json!({"status": "ok"}))
            .content(Content::JSON)
            .send()?;
        Ok(())
    });
}
