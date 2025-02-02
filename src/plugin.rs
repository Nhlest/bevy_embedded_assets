use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

/// Bevy plugin to add to your application that will insert a custom [`AssetServer`] embedding
/// your assets instead of the default added by the [`AssetPlugin`](bevy::asset::AssetPlugin).
/// If you are using the [`DefaultPlugins`] group from Bevy, it can be added this way:
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_embedded_assets::EmbeddedAssetPlugin;
/// # fn main() {
///     App::new().add_plugins_with(DefaultPlugins, |group| {
///         group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
///     });
/// # }
/// ```
#[allow(
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::module_name_repetitions
)]
#[derive(Default)]
pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetServer::new(
            crate::EmbeddedAssetIo::preloaded(),
        ));
    }
}
