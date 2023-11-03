use bevy::{prelude::{App, PluginGroup, Entity, Component}, DefaultPlugins};
use bevy_replicon::{ReplicationPlugins, server::{ServerPlugin, TickPolicy}, prelude::{MapNetworkEntities, Mapper}};
use bevy_xpbd_2d::prelude::PhysicsPlugins;
use serde::{Deserialize, Serialize};


fn main()
{
    App::new()
    .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
    .add_plugins(    
    ReplicationPlugins
        .build()
        .disable::<ServerPlugin>()
        .set(ServerPlugin::new(TickPolicy::MaxTickRate(60))))
        .run();

} 






#[derive(Component, Deserialize, Serialize)]
struct MappedComponent(Entity);

impl MapNetworkEntities for MappedComponent {
    fn map_entities<T: Mapper>(&mut self, mapper: &mut T) {
        self.0 = mapper.map(self.0);
    }
}