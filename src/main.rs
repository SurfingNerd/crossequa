use atlas_test::run_atlas_test;
use bevy::{app::App, DefaultPlugins};
use crossequa_plugin::CrossequaPlugin;

mod texture_manager;
mod crossequa_plugin;
mod gameboard;
mod atlas_test;


fn main() {
    println!("Hello, world!");

    //run_atlas_test();

     App::new()
         .add_plugins(CrossequaPlugin)
         .run();
}
