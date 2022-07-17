use std::path::Path;
use wasmer::{Exports, Function, Store};
use wasmer_wasi::WasiEnv;

use super::{wasi_read, wasi_read_string, PluginConfig, PluginEnv};
use crate::state::AppState;
use entities::models::crawl_queue::enqueue_all;
use spyglass_plugin::{PluginEnqueueRequest, PluginMountRequest};

pub fn register_exports(
    state: &AppState,
    plugin: &PluginConfig,
    store: &Store,
    env: &WasiEnv,
) -> Exports {
    let mut exports = Exports::new();
    let env = PluginEnv {
        name: plugin.name.clone(),
        app_state: state.clone(),
        data_dir: plugin.data_folder(),
        wasi_env: env.clone(),
    };

    exports.insert(
        "plugin_enqueue",
        Function::new_native_with_env(store, env.clone(), plugin_enqueue),
    );
    exports.insert(
        "plugin_log",
        Function::new_native_with_env(store, env.clone(), plugin_log),
    );
    exports.insert(
        "plugin_sync_file",
        Function::new_native_with_env(store, env, plugin_sync_file),
    );
    exports
}

pub(crate) fn plugin_log(env: &PluginEnv) {
    if let Ok(msg) = wasi_read_string(&env.wasi_env) {
        log::info!("{}: {}", env.name, msg);
    }
}

/// Adds a file into the plugin data directory. Use this to copy files from elsewhere
/// in the filesystem so that it can be processed by the plugin.
pub(crate) fn plugin_sync_file(env: &PluginEnv) {
    if let Ok(mount_request) = wasi_read::<PluginMountRequest>(&env.wasi_env) {
        log::info!(
            "requesting access to folder: {}: {}",
            mount_request.dst,
            mount_request.src
        );

        let src = Path::new(&mount_request.src);
        if let Some(file_name) = src.file_name() {
            let dst = &env.data_dir.join(file_name);
            // Attempt to mount directory
            if let Err(e) = std::fs::copy(mount_request.src, &dst) {
                log::error!("Unable to copy into plugin data dir: {}", e);
            }
        } else {
            log::error!("Source must be a file: {}", src.display());
        }
    }
}

pub(crate) fn plugin_enqueue(env: &PluginEnv) {
    if let Ok(request) = wasi_read::<PluginEnqueueRequest>(&env.wasi_env) {
        log::info!("{} enqueuing {} urls", env.name, request.urls.len());
        let state = env.app_state.clone();
        // Grab a handle to the plugin manager runtime
        let rt = tokio::runtime::Handle::current();
        rt.spawn(async move {
            let state = state.clone();
            match enqueue_all(
                &state.db.clone(),
                &request.urls,
                &[],
                &state.user_settings,
                &Default::default(),
            )
            .await
            {
                Ok(_) => log::info!("enqueue successful"),
                Err(e) => log::error!("error adding to queue: {}", e),
            }
        });
    }
}
