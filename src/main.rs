use atlas_test::run_atlas_test;
use bevy::app::App;
use crossequa_plugin::CrossequaPlugin;

mod atlas_test;
mod crossequa_plugin;
mod gameboard;
mod grid;
mod texture_manager;

fn main() {
    println!("Hello, world!");

    //run_atlas_test();

    App::new().add_plugins(CrossequaPlugin).run();
}
