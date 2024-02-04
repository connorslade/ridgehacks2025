use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/stats", |ctx| {
        let stats = ctx.app().database.get_stats()?;
        ctx.text(json!(stats)).content(Content::JSON).send()?;
        Ok(())
    });
}
