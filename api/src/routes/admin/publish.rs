use std::sync::Arc;

use afire::{extensions::RouteShorthands, route::RouteContext, Content, HeaderName, Server};
use serde::Deserialize;
use serde_json::json;
use tokio_scoped::ScopeBuilder;
use tracing::warn;
use web_push::{ContentEncoding, Urgency, WebPushClient, WebPushError, WebPushMessageBuilder};

use crate::app::App;

#[derive(Deserialize)]
struct PublishRequest {
    message: String,
}

pub fn attach(server: &mut Server<App>) {
    server.post("/api/admin/publish", |ctx| {
        let auth = ctx
            .req
            .headers
            .get(HeaderName::Authorization)
            .context("Missing Authorization header")?;

        let auth = auth
            .strip_prefix("Bearer ")
            .context("Invalid Authorization header")?;

        if auth.trim() != ctx.app().config.publish_token {
            return Err("Invalid token".into());
        }

        let request = serde_json::from_slice::<PublishRequest>(&ctx.req.body)?;
        let message = Arc::new(request.message.as_bytes());
        let subscribers = ctx
            .app()
            .database
            .get_subscribers()
            .context("Getting subscribers")?;

        let _ = ctx
            .text(json!({
                "status": "ok",
            }))
            .content(Content::JSON)
            .send();

        ScopeBuilder::from_runtime(&ctx.app().runtime).scope(move |scope| {
            for subscriber in subscribers {
                let subscription_info = subscriber.into();
                let signature = ctx
                    .app()
                    .signature
                    .clone()
                    .add_sub_info(&subscription_info)
                    .build()
                    .unwrap();

                let mut builder = WebPushMessageBuilder::new(&subscription_info);
                builder.set_urgency(Urgency::High);
                builder.set_vapid_signature(signature);
                builder.set_payload(ContentEncoding::Aes128Gcm, &*message);

                let message = builder.build().unwrap();
                let app = ctx.app();
                scope.spawn(async move {
                    match app.web_push_client.send(message).await {
                        Err(WebPushError::EndpointNotFound | WebPushError::EndpointNotValid) => {
                            if let Err(e) =
                                app.database.remove_subscriber(&subscription_info.endpoint)
                            {
                                warn!("Failed to remove subscriber: {}", e);
                            }
                        }
                        Err(e) => warn!("Failed to send push notification: {}", e),
                        _ => {}
                    }
                });
            }
        });

        Ok(())
    });
}
