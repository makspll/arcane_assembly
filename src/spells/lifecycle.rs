use bevy::{
    ecs::{
        entity::Entity,
        message::{MessageReader, MessageWriter},
        query::With,
        system::{Commands, Query, Res, Single},
        world::{Mut, Ref},
    },
    time::{Time, Virtual},
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    character::controllable_character::{Character, Player},
    spells::{
        executor::{SpellEvent, SpellEventPayload},
        spell::{ExecutingSpellComponent, WithLifetime},
    },
};

pub fn trigger_spell_expirations(
    mut spells: Query<(Entity, &ExecutingSpellComponent, &WithLifetime)>,
    mut spell_events: MessageWriter<SpellEvent>,
) {
    for (entity, spell, _) in spells.iter_mut().filter(|(_, _, l)| l.expired) {
        spell_events.write(SpellEvent {
            payload: SpellEventPayload::Expired {
                spell_entity: entity,
            },
            execution_id: spell.execution_id,
        });
    }
}

// rapier also does Contact events, but those require keeping track of entities we care about in map or something.
// here because we know these are short lived, it might be better to go from the projectiles themselvevs, and check.
// the collision graph is gonna be hella optiimzed for this.
pub fn trigger_spell_hits(
    collidable_spells: Query<(Entity, Ref<ExecutingSpellComponent>)>,
    characters: Query<Entity, With<Character>>,
    mut collisions: MessageReader<CollisionEvent>,
    mut spell_events: MessageWriter<SpellEvent>,
    // mut physics_context: Single<Ref<RapierContextSimulation>>,
) {
    // there is also CollidingEntities as a component, which can be inserted and rapier will do this for us
    // but we'd need to manually workout which collisions are new each frame
    for collision in collisions.read() {
        // do something on stopped too ?
        if let CollisionEvent::Started(e1, e2, _) = collision {
            let (spell_entity, other_entity, spell) =
                if let Ok((entity, spell)) = collidable_spells.get(*e1) {
                    (entity, *e2, spell)
                } else if let Ok((entity, spell)) = collidable_spells.get(*e2) {
                    (entity, *e1, spell)
                } else {
                    continue;
                };
            spell_events.write(SpellEvent {
                execution_id: spell.execution_id,
                payload: if characters.contains(other_entity) {
                    SpellEventPayload::HitCharacter {
                        spell_entity,
                        other_entity,
                    }
                } else {
                    SpellEventPayload::HitTerrain {
                        spell_entity,
                        other_entity,
                    }
                },
            });
        }
    }
}

pub fn mark_dead_lifetimes(time: Res<Time<Virtual>>, mut with_lifetime: Query<&mut WithLifetime>) {
    let current_time = time.elapsed_secs_wrapped_f64();

    for (mut lifetime) in with_lifetime.iter_mut() {
        // abs due to possible wrapping
        let diff = (lifetime.start_at - current_time).abs();
        if diff <= lifetime.lifetime_seconds {
            continue;
        }
        lifetime.expired = true;
    }
}

pub fn despawn_expired_entities(
    with_lifetime: Query<(Entity, &WithLifetime)>,
    mut commands: Commands,
) {
    for (e, _) in with_lifetime.iter().filter(|(_, l)| l.expired) {
        commands.entity(e).despawn();
    }
}
