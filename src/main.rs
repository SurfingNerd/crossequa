use bevy::{app::App, DefaultPlugins};
use crossequa_plugin::CrossequaPlugin;

mod crossequa_plugin;
mod gameboard;


fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(CrossequaPlugin)
        .run();
}
