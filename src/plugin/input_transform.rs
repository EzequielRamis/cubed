use crate::system::keyboard_input;
use bevy::prelude::*;

#[derive(Default)]
pub struct InputTransform;

impl Plugin for InputTransform {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_input.system());
    }
}
