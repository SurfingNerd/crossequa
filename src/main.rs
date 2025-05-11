use bevy::app::App;
use bevy::input;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crossequa_plugin::CrossequaPlugin;
use lighting::main_lighting;

mod atlas_test;
mod crossequa_plugin;
mod grid;
mod texture_manager;
mod lighting;

fn main() {
    //run_atlas_test();

    App::new()
        .add_plugins(CrossequaPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::default().run_if(
            input::common_conditions::input_toggle_active(true, input::prelude::KeyCode::Tab),
        ))
        .run();
}
