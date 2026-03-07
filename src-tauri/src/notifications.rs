// Notifications module - Native system notifications
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn show_notification(app: &AppHandle, title: &str, body: &str) -> Result<(), String> {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| e.to_string())
}

pub fn show_error_notification(app: &AppHandle, message: &str) -> Result<(), String> {
    show_notification(app, "Error - Favoritos Desktop", message)
}

pub fn show_success_notification(app: &AppHandle, message: &str) -> Result<(), String> {
    show_notification(app, "Favoritos Desktop", message)
}
