//! Handle internal errors with Tauri.

use tauri::App;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

use std::path::PathBuf;

fn exit_app(app: &mut App) -> ! {
    app.cleanup_before_exit();
    std::process::exit(0x0);
}

/// Fatal error occurs.
pub fn internal_error(app: &mut App) -> ! {
    app.dialog()
        .message(
            "An internal error has occurred. Please wait for the next update.",
        )
        .kind(MessageDialogKind::Error)
        .title("Internal error")
        .blocking_show();
    exit_app(app);
}

/// Alert user to authorize keyring to access secure key.
pub fn unauthorized_key(app: &mut App) -> ! {
    app.dialog()
    .message("Secure key is used to decrypt your messages. Denying access may delete all previous messages.")
    .kind(MessageDialogKind::Warning)
    .title("Secure key warning")
    .blocking_show();
    exit_app(app);
}

/// If database is not readable, ask user to delete or conserve it.
pub fn corrupted_db(app: &mut App, db_path: PathBuf) -> ! {
    let result = app
        .dialog()
        .message(
            "Do you want to delete it to launch the application? This will delete all messages.",
        )
        .kind(MessageDialogKind::Error)
        .title("Messages database is corrupted")
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Delete database".into(),
            "No".into(),
        ))
        .blocking_show();

    if result {
        if std::fs::remove_file(db_path.clone()).is_err() {
            failed_delete_corrupted_db(app, db_path);
        }

        app.handle().restart();
    } else {
        exit_app(app);
    }
}

fn failed_delete_corrupted_db(app: &mut App, db_path: PathBuf) -> ! {
    app.dialog()
        .message(format!(
            "Cannot delete file automatically. Delete yourself at: {db_path:?}"
        ))
        .kind(MessageDialogKind::Error)
        .title("Critical error")
        .blocking_show();
    exit_app(app);
}
