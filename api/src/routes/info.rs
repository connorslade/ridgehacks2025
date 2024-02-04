use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/info", |ctx| {
        let config = &ctx.app().config;
        ctx.text(json!({
            "push_key": config.push_public_key,
        }))
        .content(Content::JSON)
        .send()?;
        Ok(())
    });
}
