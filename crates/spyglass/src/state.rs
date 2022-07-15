use std::sync::Arc;

use dashmap::DashMap;
use entities::models::create_connection;
use entities::sea_orm::DatabaseConnection;
use tokio::sync::mpsc::Sender;

use crate::{
    plugins::PluginCommand,
    search::{IndexPath, Searcher},
};
use shared::config::{Config, Lens, UserSettings};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub app_state: Arc<DashMap<String, String>>,
    pub lenses: Arc<DashMap<String, Lens>>,
    pub user_settings: UserSettings,
    pub index: Searcher,
    // Plugin command/control
    pub plugin_cmd_tx: Option<Sender<PluginCommand>>,
}

impl AppState {
    pub async fn new(config: &Config) -> Self {
        let db = create_connection(config, false)
            .await
            .expect("Unable to connect to database");

        let index = Searcher::with_index(&IndexPath::LocalPath(config.index_dir()));

        // TODO: Load from saved preferences
        let app_state = DashMap::new();
        app_state.insert("paused".to_string(), "false".to_string());

        // Convert into dashmap
        let lenses = DashMap::new();
        for (key, value) in config.lenses.iter() {
            lenses.insert(key.clone(), value.clone());
        }

        AppState {
            db,
            app_state: Arc::new(app_state),
            user_settings: config.user_settings.clone(),
            lenses: Arc::new(lenses),
            index,
            plugin_cmd_tx: None,
        }
    }
}
