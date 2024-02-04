use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;
use tracing::info;
use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, Urgency, WebPushClient,
    WebPushMessageBuilder,
};

use crate::{app::App, database::PushSubscribe};

pub fn attach(server: &mut Server<App>) {
    server.post("/api/subscribe", |ctx| {
        let app = ctx.app();

        let request = serde_json::from_slice::<PushSubscribe>(&ctx.req.body)?;
        info!("New subscriber: {}", request.endpoint);
        app.database.add_subscriber(&request)?;

        ctx.text(json!({"status": "ok"}))
            .content(Content::JSON)
            .send()?;

        let subscription_info =
            SubscriptionInfo::new(&request.endpoint, &request.p256dh, &request.auth);

        let signature = app
            .signature
            .clone()
            .add_sub_info(&subscription_info)
            .build()?;

        let mut builder = WebPushMessageBuilder::new(&subscription_info);
        builder.set_urgency(Urgency::High);
        builder.set_vapid_signature(signature);
        builder.set_payload(
            ContentEncoding::Aes128Gcm,
            "You are now subscribed to Ridgehacks push notifications!".as_bytes(),
        );

        let message = builder.build()?;
        let client = IsahcWebPushClient::new()?;

        let response = app.runtime.block_on(client.send(message));
        let _ = dbg!("{:?}", response);

        Ok(())
    });
}
