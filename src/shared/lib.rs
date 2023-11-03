use bevy::prelude::*;
use bevy_replicon::prelude::*;


mod camera;


pub struct SharedGamePlugin;

impl Plugin for SharedGamePlugin {
    fn build(&self, _app: &mut App) {
        //If this is even needed
    }
}