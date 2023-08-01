//! Displays a single [`Sprite`], created from an image.

use bevy::prelude::*;
use bevy_visual_novel::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VisualNovelPlugin)
        .run();
}
