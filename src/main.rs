use bevy::{DefaultPlugins, app::App};
use crossequa_plugin::CrossequaPlugin;
use lighting::main_lighting;

mod atlas_test;
mod crossequa_plugin;
mod gameboard;
mod grid;
mod texture_manager;
mod lighting;

fn main() {
    println!("Hello, world!");

    //run_atlas_test();

    main_lighting();

    //App::new().add_plugins(CrossequaPlugin).run();
}
