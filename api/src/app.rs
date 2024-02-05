use anyhow::Result;
use tokio::runtime::{self, Runtime};
use web_push::{IsahcWebPushClient, PartialVapidSignatureBuilder, VapidSignatureBuilder};

use crate::{config::Config, database::Db};

pub struct App {
    pub database: Db,
    pub config: Config,
    pub runtime: Runtime,

    pub web_push_client: IsahcWebPushClient,
    pub signature: PartialVapidSignatureBuilder,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let connection = rusqlite::Connection::open(&config.database)?;
        let database = Db::new(connection);
        database.init()?;

        let signature =
            VapidSignatureBuilder::from_base64_no_sub(&config.push_private_key, base64::URL_SAFE)?;

        let web_push_client = IsahcWebPushClient::new()?;

        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(config.async_threads)
            .enable_all()
            .build()?;

        Ok(Self {
            database,
            config,
            runtime,
            web_push_client,
            signature,
        })
    }
}
