use afire::{
    extensions::Logger,
    trace::{self, Level},
    Middleware, Server,
};
use anyhow::Result;
use dotenvy::dotenv_override;

use app::App;
use config::Config;
mod app;
mod config;
mod routes;

fn main() -> Result<()> {
    trace::set_log_level(Level::Trace);
    dotenv_override().unwrap();

    let config = Config::from_env()?;

    let mut server = Server::<App>::new(config.host, config.port);
    server.thread_pool.resize(config.threads);

    Logger::new().attach(&mut server);
    routes::attach(&mut server);

    server.run()?;
    Ok(())
}
