use afire::{extensions::RouteShorthands, Server};
use serde::Deserialize;
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
        Ok(())
    });
}
