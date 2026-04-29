use std::{collections::VecDeque, fmt, ops::Index, sync::atomic::AtomicUsize};

use bevy::{
    asset::{AssetPath, Assets, Handle},
    ecs::{
        entity::Entity,
        error::trace,
        message::{Message, MessageReader},
        reflect::ReflectResource,
        resource::Resource,
        system::{Command, Local, Query, Res, ResMut, SystemState},
        world::World,
    },
    log::{error, info, trace, warn},
    math::Vec2,
    platform::collections::HashMap,
    reflect::Reflect,
    time::{Time, Virtual},
    transform::components::{GlobalTransform, Transform},
};
use bevy_mod_scripting::{
    bindings::{AppReflectAllocator, InteropError, ReflectAllocator, ReflectReference},
    core::{error::ScriptError, event::CallbackLabel, script::ScriptContexts},
    display::{DisplayWithTypeInfo, ReflectDisplayWithTypeInfo, WorldAccessGuard},
    lua::LuaScriptingPlugin,
    prelude::{RunScriptCallback, ScriptValue},
    script::ScriptAttachment,
};
use bevy_rapier2d::prelude::{RigidBody, Velocity};
use petgraph::{
    Graph,
    dot::{Config, Dot},
    graph::NodeIndex,
    visit::{Dfs, EdgeRef, Walker},
};

use crate::spells::{
    dotgraph::{SpellGraphNode, SpellGraphTransition},
    spell::{LiveSpell, WithLifetime},
    spell_component_asset::SpellComponentAsset,
};

type NodeId = usize;

/// An optimized representation of an arrangement of spell components, which can be executed by the spell executor.
#[derive(Reflect, Debug)]
#[reflect(Debug, DisplayWithTypeInfo)]
pub struct Spell {
    pub debug_repr: String,
    pub nodes: Vec<SpellComponent>,
}

#[derive(Reflect, Debug)]
pub struct SpellComponent {
    pub descriptor: Handle<SpellComponentAsset>,
    pub transitions: Vec<(SpellTransitionTrigger, NodeId)>,
}

#[derive(Debug, Clone, Reflect)]
pub enum SpellTransitionTrigger {
    CastImmediately,
    HitCharacter,
    HitTerrain,
    Expired,
    Custom(String),
}

impl SpellTransitionTrigger {
    pub fn new(event: Option<&str>) -> Self {
        match event {
            Some(SpellEventPayload::ON_HIT_CHARACTER_LABEL_PREFIX) => Self::HitCharacter,
            Some(SpellEventPayload::ON_HIT_TERRAIN_LABEL_PREFIX) => Self::HitTerrain,
            Some(SpellEventPayload::ON_EXPIRED_LABEL_PREFIX) => Self::Expired,
            Some(event) => Self::Custom(event.to_string()),
            None => Self::CastImmediately,
        }
    }
    pub fn satisfied_by(&self, payload: &SpellEventPayload) -> bool {
        match payload {
            SpellEventPayload::HitCharacter { .. } => {
                matches!(self, Self::HitCharacter)
            }
            SpellEventPayload::HitTerrain { .. } => {
                matches!(self, Self::HitTerrain)
            }
            SpellEventPayload::Expired { .. } => matches!(self, Self::Expired),
            SpellEventPayload::Custom { event, .. } => {
                matches!(self, Self::Custom(subscribed_event) if subscribed_event == event)
            }
            _ => matches!(self, Self::CastImmediately),
        }
    }
}

impl Spell {
    /// builds and optimizes the spell graph, such that
    /// iterating the nodes yields the actual execution order
    pub fn new(start_node: NodeIndex, graph: Graph<SpellGraphNode, SpellGraphTransition>) -> Self {
        let mut dfs = Dfs::new(&graph, start_node);

        let mut dfs_index_of = vec![0usize; graph.node_count()];
        let mut idx = 0;
        while let Some(next) = dfs.next(&graph) {
            dfs_index_of[next.index()] = idx;
            idx += 1;
        }

        let mut nodes = Vec::with_capacity(dfs_index_of.len());
        for node_idx in dfs_index_of.iter() {
            let node_idx = NodeIndex::new(*node_idx);
            let transitions = graph
                .edges_directed(node_idx, petgraph::Direction::Outgoing)
                .map(|edge| {
                    let trigger_event = edge.weight().modifier_event.as_deref();
                    let trigger = SpellTransitionTrigger::new(trigger_event);
                    let target = edge.target();
                    let target_dfs_idx = dfs_index_of[target.index()];
                    (trigger, target_dfs_idx)
                })
                .collect();
            // ordered by dfs traversal
            nodes.push(SpellComponent {
                descriptor: graph
                    .node_weight(node_idx)
                    .expect("node just taken from graph")
                    .descriptor
                    .clone(),
                transitions,
            });
        }

        // we compute this here with the dfs order, even though we generate the original graph from dot files
        let debug_repr = format!(
            "{:?}",
            Dot::with_attr_getters(
                &graph,
                &[Config::EdgeNoLabel, Config::NodeNoLabel],
                &|_, node| {
                    node.weight()
                        .modifier_event
                        .as_ref()
                        .map(|event| format!("label=\"{}\"", event))
                        .unwrap_or_default()
                },
                &|_, (_, node)| format!(
                    "label=\"{}\"",
                    node.descriptor
                        .path()
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| String::from("missing asset path"))
                ),
            )
        );

        Self { debug_repr, nodes }
    }
}

impl DisplayWithTypeInfo for Spell {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        f.write_str(&self.debug_repr)
    }
}

#[derive(Resource, Default)]
pub struct AbilityExecutions(pub HashMap<AbilityExecutionId, AbilityExecution>);

impl AbilityExecutions {
    pub fn insert(&mut self, execution: AbilityExecution) {
        self.0.insert(execution.id, execution);
    }

    pub fn iter(&self) -> impl Iterator<Item = &AbilityExecution> {
        self.0.iter().map(|(_, a)| a)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut AbilityExecution> {
        self.0.iter_mut().map(|(_, a)| a)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AbilityExecutionState {
    Executing,
    Terminated,
}

#[derive(Message, Debug, Clone)]
pub struct SpellEvent {
    pub execution_id: usize,
    pub payload: SpellEventPayload,
}

#[derive(Debug, Clone)]
pub enum SpellEventPayload {
    Cast {
        position: Vec2,
        velocity: Vec2,
    },
    HitCharacter {
        spell_entity: Entity,
        other_entity: Entity,
    },
    HitTerrain {
        spell_entity: Entity,
        other_entity: Entity,
    },
    Expired {
        spell_entity: Entity,
    },
    /// mod driven
    Custom {
        spell_entity: Entity,
        event: String,
    },
}
impl SpellEventPayload {
    const ON_CAST_LABEL_PREFIX: &str = "on_cast";
    const ON_HIT_CHARACTER_LABEL_PREFIX: &str = "on_hit_character";
    const ON_HIT_TERRAIN_LABEL_PREFIX: &str = "on_hit_terrain";
    const ON_EXPIRED_LABEL_PREFIX: &str = "on_expired";

    pub fn to_callback_label(&self, postfix: &str) -> CallbackLabel {
        match self {
            SpellEventPayload::Cast { .. } => {
                CallbackLabel::new_lossy(&format!("{}_{postfix}", Self::ON_CAST_LABEL_PREFIX))
            }
            SpellEventPayload::HitCharacter { .. } => CallbackLabel::new_lossy(&format!(
                "{}_{postfix}",
                Self::ON_HIT_CHARACTER_LABEL_PREFIX
            )),
            SpellEventPayload::HitTerrain { .. } => CallbackLabel::new_lossy(&format!(
                "{}_{postfix}",
                Self::ON_HIT_TERRAIN_LABEL_PREFIX
            )),
            SpellEventPayload::Expired { .. } => {
                CallbackLabel::new_lossy(&format!("{}_{postfix}", Self::ON_EXPIRED_LABEL_PREFIX))
            }
            SpellEventPayload::Custom { event, .. } => {
                CallbackLabel::new_lossy(&format!("{event}_{postfix}"))
            }
        }
    }

    /// Returns the spell entity, or if not present, the position and velocity to spawn one with
    pub fn spell_entity(&self) -> Result<Entity, (Vec2, Vec2)> {
        match self {
            SpellEventPayload::Cast { position, velocity } => Err((*position, *velocity)),
            SpellEventPayload::HitCharacter { spell_entity, .. } => Ok(*spell_entity),
            SpellEventPayload::HitTerrain { spell_entity, .. } => Ok(*spell_entity),
            SpellEventPayload::Expired { spell_entity } => Ok(*spell_entity),
            SpellEventPayload::Custom { spell_entity, .. } => Ok(*spell_entity),
        }
    }

    pub fn add_args(&self, mut allocator: &mut ReflectAllocator, dest: &mut Vec<ScriptValue>) {
        match self {
            SpellEventPayload::Cast { .. } => {}
            SpellEventPayload::HitCharacter { other_entity, .. } => {
                dest.push(ReflectReference::new_allocated(*other_entity, &mut allocator).into())
            }
            SpellEventPayload::HitTerrain { other_entity, .. } => {
                dest.push(ReflectReference::new_allocated(*other_entity, &mut allocator).into())
            }
            SpellEventPayload::Expired { .. } => {}
            SpellEventPayload::Custom { .. } => {} // TODO: allow args ?
        }
    }
}

pub struct CastSpell {
    pub graph: Spell,
    pub position: Vec2,
    pub velocity: Vec2,
}

impl CastSpell {
    pub fn new(graph: impl Into<Spell>, position: Vec2, velocity: Vec2) -> Self {
        Self {
            graph: graph.into(),
            position,
            velocity,
        }
    }
}

impl Command for CastSpell {
    fn apply(self, world: &mut World) {
        let mut executions = world
            .remove_resource::<AbilityExecutions>()
            .expect("missing resource");

        let execution = AbilityExecution::new(self.graph);

        world.write_message(SpellEvent {
            execution_id: execution.id,
            payload: SpellEventPayload::Cast {
                position: self.position,
                velocity: self.velocity,
            },
        });
        executions.insert(execution);
        world.insert_resource(executions);
    }
}

pub struct AbilityExecution {
    pub id: AbilityExecutionId,
    pub unprocessed_events: VecDeque<SpellEvent>,
    pub graph: Spell,
    /// the DFS frontier
    pub next_node: NodeId,
    pub state: AbilityExecutionState,
    pub last_cast_entity: Entity,
}

impl AbilityExecution {
    pub fn new(graph: Spell) -> Self {
        static ID_HEAD: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: ID_HEAD.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            unprocessed_events: Default::default(),
            next_node: 0,
            graph,
            state: AbilityExecutionState::Executing,
            last_cast_entity: Entity::PLACEHOLDER,
        }
    }
}
pub type AbilityExecutionId = usize;

/// Reads all spell events and loads them into the right executions
pub fn read_spell_events_into_executor(
    mut executions: ResMut<AbilityExecutions>,
    mut events: MessageReader<SpellEvent>,
) {
    for event in events.read() {
        if let Some(exec) = executions.0.get_mut(&event.execution_id) {
            exec.unprocessed_events.push_back(event.clone());
        } else {
            error!("spell event for execution which doesn't exist: {:?}", event);
        }
    }
}

pub fn spell_executions_live(executions: Res<AbilityExecutions>) -> bool {
    !executions.0.is_empty()
}

// borrow checker friendly encapsulation
#[derive(Debug)]
struct StepPlan {
    spell_component_to_execute: Handle<SpellComponentAsset>,
    progress_to_next_spell_after_execution: Option<NodeId>,
    terminate_spell_after_execution: bool,
}

pub fn progress_spell_executions(
    world: &mut World,
    mut query_state: Local<SystemState<Query<(&Transform, &Velocity)>>>,
) {
    let mut executions = world
        .remove_resource::<AbilityExecutions>()
        .expect("missing resource");

    let allocator = world
        .get_resource::<AppReflectAllocator>()
        .cloned()
        .expect("missing allocator");

    let spell_descriptor_assets: Assets<SpellComponentAsset> =
        world.remove_resource().expect("missing resource");

    let mut any_finished = false;

    let mut mark_terminal_state = |execution: &mut AbilityExecution| {
        execution.state = AbilityExecutionState::Terminated;
        any_finished = true;
    };

    for (_, execution) in &mut executions.0 {
        while let Some(event) = execution.unprocessed_events.pop_front() {
            let plan = match plan_execution_for_event(execution, &event.payload) {
                Ok(plan) => plan,
                Err(err) => {
                    error!(
                        "Failed to plan spell execution for event: {:?}. {err}",
                        event
                    );
                    mark_terminal_state(execution);
                    continue;
                }
            };

            let spell_descriptor_asset =
                match spell_descriptor_assets.get(&plan.spell_component_to_execute) {
                    Some(asset) => asset,
                    None => {
                        error!(
                            "Spell execution failed, missing asset: {:?}",
                            plan.spell_component_to_execute.path()
                        );
                        mark_terminal_state(execution);
                        continue;
                    }
                };

            trace!("Executing spell event: {:?}", event.payload);

            if plan.terminate_spell_after_execution {
                mark_terminal_state(execution);
            }

            let (entity_ref, entity) = resolve_or_spawn_cast_entity(
                world,
                &allocator,
                execution,
                &event,
                spell_descriptor_asset,
                &plan.spell_component_to_execute,
            );

            // we have to do this here just before the callback
            // because the callback itself may despawn the entity, meaning we won't have access to the transform
            let cast_location = if plan.progress_to_next_spell_after_execution.is_some() {
                let (pos, vel) = match query_state.get(world).get(
                    if execution.last_cast_entity != Entity::PLACEHOLDER {
                        execution.last_cast_entity
                    } else {
                        entity
                    },
                ) {
                    Ok((t, v)) => (t.translation.truncate(), v.linvel),
                    Err(e) => {
                        error!(
                            "Error querying transform in spell cast, last_cast_entity: {:?} : {e}",
                            execution.last_cast_entity
                        );
                        (Default::default(), Default::default())
                    }
                };
                Some((pos, vel))
            } else {
                None
            };

            if let Err(err) = execute_spell_callback(
                world,
                &allocator,
                &event,
                spell_descriptor_asset,
                entity_ref,
            ) {
                error!("Spell execution failed: {err}");
                mark_terminal_state(execution);
                continue;
            }

            if let Some(move_to_node) = plan.progress_to_next_spell_after_execution {
                let cast_location = cast_location.unwrap_or_default();
                execution.unprocessed_events.push_front(SpellEvent {
                    execution_id: execution.id,
                    payload: SpellEventPayload::Cast {
                        position: cast_location.0,
                        velocity: cast_location.1,
                    },
                });
                execution.next_node = move_to_node;
            }
            execution.last_cast_entity = entity;
        }
    }

    // cleanup
    if any_finished {
        executions
            .0
            .retain(|_, v| !matches!(v.state, AbilityExecutionState::Terminated));
    }

    world.insert_resource(executions);
    world.insert_resource(spell_descriptor_assets);
}

pub fn plan_execution_for_event(
    execution: &AbilityExecution,
    payload: &SpellEventPayload,
) -> Result<StepPlan, String> {
    let current_node_id = execution.next_node;
    let current_node = match execution.graph.nodes.get(current_node_id) {
        Some(node) => node,
        None => {
            return Err(format!(
                "Missing node in spell execution {}",
                current_node_id
            ));
        }
    };
    let spell_to_execute = current_node.descriptor.clone();

    // TODO: this currently means ANY spell currently spawned can affect the transition
    // might not be what we want
    let progress_to_next_spell_after_execution = current_node
        .transitions
        .iter()
        .find_map(|(t, next)| t.satisfied_by(payload).then_some(*next));

    Ok(StepPlan {
        spell_component_to_execute: spell_to_execute,
        progress_to_next_spell_after_execution,
        terminate_spell_after_execution: matches!(payload, SpellEventPayload::Expired { .. }),
    })
}

fn execute_spell_callback(
    world: &mut World,
    allocator: &AppReflectAllocator,
    event: &SpellEvent,
    spell: &SpellComponentAsset,
    entity_ref: ReflectReference,
) -> Result<(), ScriptError> {
    let mut allocator = allocator.write();
    let mut args = vec![entity_ref.into()];
    event.payload.add_args(&mut allocator, &mut args);
    drop(allocator);

    let label = event
        .payload
        .to_callback_label(&spell.descriptor.handler_label);

    run_spell_callback(world, args, spell, label).map(|_| ())
}

fn resolve_or_spawn_cast_entity(
    world: &mut World,
    allocator: &AppReflectAllocator,
    execution: &AbilityExecution,
    event: &SpellEvent,
    spell: &SpellComponentAsset,
    handle: &Handle<SpellComponentAsset>,
) -> (ReflectReference, Entity) {
    let mut allocator = allocator.write();
    match event.payload.spell_entity() {
        Ok(entity) => (
            ReflectReference::new_allocated(entity, &mut allocator),
            entity,
        ),
        Err((pos, vel)) => {
            let time = world
                .get_resource::<Time<Virtual>>()
                .expect("missing time resource");
            let entity = world
                .spawn((LiveSpell::new(execution.id, pos, vel, time, handle.clone(), spell)))
                .id();

            let allocated = ReflectReference::new_allocated(entity, &mut allocator);

            (allocated, entity)
        }
    }
}

fn run_spell_callback(
    world: &mut World,
    args: Vec<ScriptValue>,
    parent: &SpellComponentAsset,
    label: CallbackLabel,
) -> Result<ScriptValue, ScriptError> {
    let handle = parent.script.clone();

    let cmd = RunScriptCallback::<LuaScriptingPlugin>::new(
        ScriptAttachment::StaticScript(handle),
        label,
        args,
        false,
    )
    .with_send_errors(true)
    .with_error_context(format!("running spell: {}", parent.descriptor.identifier));
    cmd.run(world)
}
