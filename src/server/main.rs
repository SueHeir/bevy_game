use bevy::{prelude::{App, PluginGroup}, DefaultPlugins};
use bevy_game::SharedGamePlugin;
use bevy_replicon::{ReplicationPlugins, prelude::ClientPlugin, server::{ServerPlugin, TickPolicy}};
use bevy_xpbd_2d::prelude::PhysicsPlugins;

fn main()
{
    App::new() 
    .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
    .add_plugins(
        ReplicationPlugins
        .build()
        .disable::<ClientPlugin>()
        .set(ServerPlugin::new(TickPolicy::MaxTickRate(60))))
    .add_plugins(SharedGamePlugin)
        .run();

}