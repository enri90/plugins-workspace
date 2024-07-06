use tauri::{
    command,
    AppHandle,
    Runtime
};
use sentry::{add_breadcrumb, capture_event, protocol::Event, Breadcrumb};

#[command]
pub fn event<R: Runtime>(_app: AppHandle<R>, mut event: Event<'static>) {
    event.platform = "javascript".into();
    capture_event(event);
}

#[command]
pub fn breadcrumb<R: Runtime>(_app: AppHandle<R>, breadcrumb: Breadcrumb) {
    add_breadcrumb(breadcrumb);
}