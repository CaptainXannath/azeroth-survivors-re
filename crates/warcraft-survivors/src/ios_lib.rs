use benilla_app::BuildId;

/// Bevy's mobile entry point. On iOS this is called from the native application startup path,
/// which initializes UIKit before winit creates the render window and event loop.
#[bevy::prelude::bevy_main]
pub fn main() {
    let _ = benilla_app::run_survivors(BuildId {
        version: env!("CARGO_PKG_VERSION"),
        describe: env!("BENILLA_GIT_DESCRIBE"),
        sha: env!("BENILLA_GIT_SHA"),
        short: env!("BENILLA_GIT_SHORT"),
        date: env!("BENILLA_GIT_DATE"),
        profile: env!("BENILLA_PROFILE"),
        project_dir: env!("BENILLA_PROJECT_DIR"),
        ..Default::default()
    });
}
