use std::process;

use anyhow::Result;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

// Increment every time schema changes in a non backwards compatible way, even in dev
const DATABASE_VERSION: u64 = 3;

pub struct Db {
    inner: Mutex<Option<Connection>>,
}

impl Db {
    pub fn new(connection: Connection) -> Self {
        Self {
            inner: Mutex::new(Some(connection)),
        }
    }

    fn take(&self) -> Connection {
        let val = self.inner.lock().take();
        val.expect("No value to take")
    }

    fn lock(&self) -> MappedMutexGuard<'_, Connection> {
        MutexGuard::map(self.inner.lock(), |x: &mut Option<Connection>| {
            x.as_mut().expect("No value to take")
        })
    }
}

impl Db {
    pub fn init(&self) -> Result<()> {
        let mut this = self.lock();
        this.pragma_update(None, "journal_mode", "WAL")?;
        this.pragma_update(None, "synchronous", "NORMAL")?;

        let db_version =
            this.pragma_query_value(None, "user_version", |row| row.get::<_, u64>(0))?;

        match db_version {
            DATABASE_VERSION => info!("Loaded database at `{}`", this.path().unwrap()),
            0 => {
                info!("Creating database at `{}`", this.path().unwrap());
                this.pragma_update(None, "user_version", DATABASE_VERSION)?;
            }
            i => {
                error!(
                    "Database version mismatch. Expected {}, got {}. Please run migrations, or \
                     just like delete the database and start over.",
                    DATABASE_VERSION, i
                );
                drop(this);
                self.cleanup()?;
                process::exit(1);
            }
        }

        let trans = this.transaction()?;
        for i in [include_str!("./sql/create_subscribers.sql")] {
            trans.execute(i, [])?;
        }
        trans.commit()?;

        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        let this = self.take();
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        this.pragma_update(None, "optimize", "")?;
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        drop(this);

        Ok(())
    }

    pub fn add_subscriber(&self, subscription: &PushSubscribe) -> Result<()> {
        self.lock().execute(
            include_str!("sql/insert_subscribers.sql"),
            [
                &subscription.endpoint,
                &subscription.p256dh,
                &subscription.auth,
            ],
        )?;
        Ok(())
    }

    pub fn remove_subscriber(&self, endpoint: &str) -> Result<()> {
        self.lock()
            .execute("DELETE FROM subscribers WHERE endpoint = ?", [endpoint])?;
        Ok(())
    }

    pub fn get_stats(&self) -> Result<Stats> {
        let count = self
            .lock()
            .query_row("SELECT COUNT(*) FROM subscribers", [], |row| row.get(0))?;
        Ok(Stats { count })
    }
}

#[derive(Deserialize)]
pub struct PushSubscribe {
    pub endpoint: String,
    pub auth: String,
    pub p256dh: String,
}

#[derive(Serialize)]
pub struct Stats {
    pub count: u64,
}
