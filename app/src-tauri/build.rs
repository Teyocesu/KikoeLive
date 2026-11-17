fn main() {
    // Declaring the app's commands makes Tauri require an explicit capability
    // grant per window (see capabilities/), instead of exposing every command
    // to every webview.
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(&[
        "get_snapshot",
        "list_sources",
        "set_language",
        "update_settings",
        "reset_overlay_position",
        "subtitle_rendered",
        "download_model",
        "cancel_download",
        "use_existing_model",
        "start",
        "stop",
    ])))
    .expect("failed to run tauri-build");
}
