use bevy::{
    ecs::{
        entity::Entity,
        query::QueryState,
        resource::Resource,
        system::{Query, SystemState},
        world::{FromWorld, Mut, World},
    },
    math::Vec2,
};
use bevy_mod_scripting::{
    bindings::{FunctionCallContext, InteropError, V},
    script_bindings,
};
use bevy_rapier2d::{
    parry::shape::{Ball, Shape},
    plugin::ReadRapierContext,
    prelude::{Collider, CollisionGroups, Group, QueryFilter, ShapeCastOptions},
};

use crate::physics::CollisionGroup;

#[derive(Resource)]
pub struct CollisionQueryCachedState(SystemState<ReadRapierContext<'static, 'static>>);

impl FromWorld for CollisionQueryCachedState {
    fn from_world(world: &mut World) -> Self {
        Self(SystemState::new(world))
    }
}

#[script_bindings(remote, name = "group_functions")]
impl Group {
    pub fn PROJECTILE() -> V<Group> {
        Group::from(CollisionGroup::Projectile).into()
    }
    pub fn ENTITY() -> V<Group> {
        Group::from(CollisionGroup::ControlledEntity).into()
    }
    pub fn TERRAIN() -> V<Group> {
        Group::from(CollisionGroup::Terrain).into()
    }

    pub fn ALL() -> V<Group> {
        Group::all().into()
    }

    pub fn ENTITY_AND_TERRAIN() -> V<Group> {
        Group::from(CollisionGroup::Terrain)
            .union(Group::from(CollisionGroup::ControlledEntity))
            .into()
    }
}

#[script_bindings(remote, unregistered, name = "global_physics_functions")]
impl World {
    /// Casts a circle with specified diameter around the specified position.
    ///
    /// Returns all entities intersecting with the circle.
    pub fn circle_collision_query(
        ctxt: FunctionCallContext,
        center: V<Vec2>,
        radius: f32,
        groups: V<Group>,
    ) -> Result<Vec<V<Entity>>, InteropError> {
        let world = ctxt.world()?;

        let collisions = world.with_world_mut_access(|w| {
            w.resource_scope(|w, mut cache: Mut<CollisionQueryCachedState>| {
                let rapier = cache.0.get(w);
                let rapier_context = rapier.single().unwrap();
                let mut entities = Vec::default();
                rapier_context.intersect_shape(
                    *center,
                    Default::default(),
                    &Ball::new(radius),
                    QueryFilter::new().groups(CollisionGroups::new(groups.0, groups.0)),
                    |entity| {
                        entities.push(entity.into());
                        true
                    },
                );
                entities
            })
        })?;

        Ok(collisions)
    }
}
