use std::sync::Arc;

use anyhow::Error;
use sea_orm::{ConnectOptions, Database, DbConn};
use tokio::fs;

use super::settings::AppSettings;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub db: Arc<DbConn>,
    pub private_key: Vec<u8>,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let opt = ConnectOptions::new(&settings.db.endpoint);
        // opt.sqlx_logging(true)
        //     .sqlx_logging_level(log::LevelFilter::Info);
        let db = Arc::new(Database::connect(opt).await?);

        let private_key = fs::read_to_string(&settings.auth.private_key_file)
            .await?
            .into_bytes();

        Ok(Self {
            settings,
            db,
            private_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sea_orm::DatabaseConnection;

    use crate::app::{
        auth::settings::{AuthSettings, RPSettings},
        settings::{AppSettings, DBSettings, HttpSettings},
    };

    use super::AppState;

    impl AppState {
        pub fn default() -> Self {
            Self {
                settings: AppSettings {
                    _name: String::default(),
                    http: HttpSettings {
                        endpoint: String::default(),
                    },
                    db: DBSettings {
                        endpoint: String::default(),
                    },
                    auth: AuthSettings {
                        rp: RPSettings {
                            id: String::default(),
                            name: String::default(),
                        },
                        private_key_file: String::default(),
                    },
                },
                db: Arc::new(DatabaseConnection::default()),
                private_key: vec![],
            }
        }
    }
}
