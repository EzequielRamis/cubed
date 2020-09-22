mod component;
mod entity;
mod plugin;
mod system;
mod utils;

use bevy::prelude::*;
use plugin::InputTransform;
use system::setup;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_plugin(InputTransform)
        .run();
}
