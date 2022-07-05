use bevy::{input::system::exit_on_esc_system, prelude::*};
{% if bevy_inspector_egui %}use bevy_inspector_egui::WorldInspectorPlugin;{% endif %}

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        {%- if bevy_inspector_egui %}
        app.add_plugin(WorldInspectorPlugin::new())
            .add_system(exit_on_esc_system);
        {%- endif %}
    }
}
