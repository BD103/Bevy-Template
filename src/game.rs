use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup() {
    info!("Game initialized.");
}
