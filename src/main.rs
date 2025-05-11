use bevy::app::App;
use bevy::input;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crossequa_plugin::CrossequaPlugin;

mod atlas_test;
mod board;
mod crossequa_plugin;
mod equation;
mod player_input;
mod texture_manager;

fn main() {
    //run_atlas_test();

    App::new()
        .add_plugins(CrossequaPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::default().run_if(
            input::common_conditions::input_toggle_active(false, input::prelude::KeyCode::Tab),
        ))
        .run();
}
