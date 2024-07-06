use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};
use serde::Serialize;

pub use sentry;
pub use sentry_rust_minidump as minidump;

#[derive(Debug, Clone, Serialize)]
pub struct JavaScriptOptions {
    pub inject: bool,
    pub debug: bool,
    pub dsn: String,
}

impl Default for JavaScriptOptions {
    fn default() -> Self {
        Self {
            dsn: "".to_string(),
            inject: true,
            #[cfg(not(debug_assertions))]
            debug: false,
            #[cfg(debug_assertions)]
            debug: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Options {
    pub javascript: JavaScriptOptions,
}

mod commands;

pub fn init<R: Runtime>() -> TauriPlugin<R> {

    let  builder =  PluginBuilder::new("sentry")
    .invoke_handler(tauri::generate_handler![commands::event, commands::breadcrumb]);

    /*if options.javascript.inject {
        builder = builder.js_init_script(
            include_str!("../dist-js/inject.min.js")
            .replace("__DEBUG__", &format!("{}", options.javascript.debug))
            .replace("__SENTRY_DSN__", &format!("{}", options.javascript.dsn)),
        );
    }*/

    builder.build()
}