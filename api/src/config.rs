use std::{env, path::PathBuf};

use anyhow::Result;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub threads: usize,
    pub async_threads: usize,
    pub database: PathBuf,

    pub push_public_key: String,
    pub push_private_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: env::var("HOST")?,
            port: env::var("PORT")?.parse()?,
            threads: env::var("THREADS")?.parse()?,
            async_threads: env::var("ASYNC_THREADS")?.parse()?,
            database: env::var("DATABASE")?.into(),
            push_public_key: env::var("PUSH_PUBLIC_KEY")?,
            push_private_key: env::var("PUSH_PRIVATE_KEY")?,
        })
    }
}
