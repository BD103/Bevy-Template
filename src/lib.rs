mod game;

#[cfg(feature = "dev")]
mod dev;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        DefaultPlugins.build(group);

        group.add(game::MainPlugin);

        #[cfg(feature = "dev")]
        group.add(dev::DevPlugin);
    }
}
