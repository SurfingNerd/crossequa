use bevy::app::App;
use crossequa_plugin::CrossequaPlugin;

mod crossequa_plugin;
mod gameboard;
mod grid;

fn main() {
    App::new().add_plugins(CrossequaPlugin).run();
}
