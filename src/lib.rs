use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy::render::{RenderPlugin, settings::{RenderCreation, WgpuSettings, Backends}};

// https://github.com/bevyengine/bevy/issues/9975
pub struct DefaultPluginsPatch;

impl PluginGroup for DefaultPluginsPatch {
    fn build(self) -> PluginGroupBuilder {
        DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        })
    }
}
