use anyhow::Result;
use tokio::runtime::{self, Runtime};

use crate::{config::Config, database::Db};

pub struct App {
    pub database: Db,
    pub config: Config,
    pub runtime: Runtime,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let connection = rusqlite::Connection::open(&config.database)?;
        let database = Db::new(connection);
        database.init()?;

        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(config.async_threads)
            .enable_all()
            .build()?;

        Ok(Self {
            database,
            config,
            runtime,
        })
    }
}
