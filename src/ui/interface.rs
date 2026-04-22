use bevy::{
    app::Plugin,
    asset::{AssetServer, Assets, Handle},
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        hierarchy::ChildSpawnerCommands,
        name::Name,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut, Single},
    },
    math::Vec2,
    sprite::{Sprite, Text2d},
    text::{Font, TextColor, TextFont},
    transform::components::Transform,
    utils::default,
};
use bevy_lunex::prelude::*;

use crate::{
    camera::{MAIN_CAMERA_ORDER, UI_CAMERA_ORDER},
    character::{controllable_character::Player, health::Health},
    scripts::{
        loaded_script_descriptors::{self, LoadedScriptDescriptors},
        script_descriptor::{ModPathBuf, ScriptDescriptor},
    },
    spells::mana::Mana,
};

#[derive(Resource)]
pub struct GameUiConfig {
    pub status_bar_width: f32,
    pub status_bar_height: f32,
    pub status_bar_spacing: f32,
    pub font_size: f32,
}

impl Default for GameUiConfig {
    fn default() -> Self {
        Self {
            status_bar_width: 200.0,
            status_bar_height: 30.0,
            status_bar_spacing: 40.0,
            font_size: 16.0,
        }
    }
}

#[derive(Resource)]
pub struct GameUiColors {
    pub health: Color,
    pub mana: Color,
    pub background: Color,
    pub text: Color,
}

impl Default for GameUiColors {
    fn default() -> Self {
        Self {
            health: Color::srgb(1.0, 0.0, 0.0),
            mana: Color::srgb(0.0, 0.4, 1.0),
            background: Color::srgb(0.2, 0.2, 0.2),
            text: Color::WHITE,
        }
    }
}

#[derive(Component)]
pub struct PlayerHealthBarUI;

#[derive(Component)]
pub struct PlayerManaBarUI;

#[derive(Component)]
pub struct HealthFill;

#[derive(Component)]
pub struct ManaFill;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ManaText;

pub fn spawn_player_interface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameUiConfig>,
    colors: Res<GameUiColors>,
    loaded_script_descriptors: Res<LoadedScriptDescriptors>,
    descriptor_assets: Res<Assets<ScriptDescriptor>>,
) {
    let font_path = ModPathBuf::new_mod_assets_path("Main", "fonts/edit_undo/editundo.ttf");

    let font = asset_server.load(
        font_path
            .asset_path(&loaded_script_descriptors, &descriptor_assets)
            .expect("missing core font"),
    );

    // so this would be sick to script actually
    // since it's retained mode it would only need to run lua once + on modification
    commands
        .spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<UI_CAMERA_ORDER>,
            Name::new("Player Interface"),
        ))
        .with_children(|root| {
            root.spawn(
                UiLayout::window()
                    .anchor(Anchor::TOP_LEFT)
                    .pos((0., -500.)) // this is weird, it should start in top left
                    .size((
                        config.status_bar_width,
                        config.status_bar_height * 2.0 + config.status_bar_spacing,
                    ))
                    .pack(),
            )
            .with_children(|ui| {
                spawn_bar(
                    ui,
                    &config,
                    &colors,
                    font.clone(),
                    0.0,
                    PlayerHealthBarUI,
                    HealthFill,
                    HealthText,
                    colors.health,
                    "100 / 100",
                );

                spawn_bar(
                    ui,
                    &config,
                    &colors,
                    font.clone(),
                    -config.status_bar_spacing,
                    PlayerManaBarUI,
                    ManaFill,
                    ManaText,
                    colors.mana,
                    "75 / 100",
                );
            });
        });
}

fn spawn_bar<BarMarker: Component, FillMarker: Component, TextMarker: Component>(
    parent: &mut ChildSpawnerCommands,
    config: &GameUiConfig,
    colors: &GameUiColors,
    font: Handle<Font>,
    y_offset: f32,
    bar_marker: BarMarker,
    fill_marker: FillMarker,
    text_marker: TextMarker,
    fill_color: Color,
    initial_text: &str,
) {
    parent
        .spawn((
            UiLayout::window()
                .size((config.status_bar_width, config.status_bar_height))
                .pos((0.0, y_offset))
                .pack(),
            bar_marker,
        ))
        .with_children(|bar| {
            // Background
            bar.spawn(Sprite {
                color: colors.background,
                custom_size: Some(Vec2::new(config.status_bar_width, config.status_bar_height)),
                ..default()
            });

            // Fill
            bar.spawn((
                Sprite {
                    color: fill_color,
                    custom_size: Some(Vec2::new(config.status_bar_width, config.status_bar_height)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                fill_marker,
            ));

            // Text
            bar.spawn((
                Text2d::new(initial_text),
                TextFont {
                    font,
                    font_size: config.font_size,
                    ..default()
                },
                TextColor(colors.text),
                Transform::from_xyz(0.0, 0.0, 1.0),
                text_marker,
            ));
        });
}

pub fn set_health_ui(
    current: f32,
    max: f32,
    config: &GameUiConfig,
    mut fill_query: Single<(&mut Sprite, &mut Transform), With<HealthFill>>,
    mut text_query: Single<&mut Text2d, With<HealthText>>,
) {
    update_bar(current, max, config, fill_query, text_query);
}

pub fn set_mana_ui(
    current: f32,
    max: f32,
    config: &GameUiConfig,
    mut fill_query: Single<(&mut Sprite, &mut Transform), With<ManaFill>>,
    mut text_query: Single<&mut Text2d, With<ManaText>>,
) {
    update_bar(current, max, config, fill_query, text_query);
}

fn update_bar<F1, F2>(
    current: f32,
    max: f32,
    config: &GameUiConfig,
    fill_query: Single<(&mut Sprite, &mut Transform), F1>,
    text_query: Single<&mut Text2d, F2>,
) where
    F1: bevy::ecs::query::QueryFilter,
    F2: bevy::ecs::query::QueryFilter,
{
    let percent = (current / max).clamp(0.0, 1.0);

    let full_width = config.status_bar_width;
    let new_width = full_width * percent;

    // Fill
    let (mut sprite, mut transform) = fill_query.into_inner();
    sprite.custom_size = Some(Vec2::new(new_width, config.status_bar_height));
    transform.translation.x = -(full_width - new_width) / 2.0;

    // Text
    let text: &mut Text2d = &mut text_query.into_inner();
    *text = Text2d::new(format!("{:.0} / {:.0}", current, max));
}

pub fn sync_mana_value_to_ui(
    config: Res<GameUiConfig>,
    player_mana: Single<&Mana, (With<Mana>, With<Player>)>,
    mut fill_query: Single<(&mut Sprite, &mut Transform), With<ManaFill>>,
    mut text_query: Single<&mut Text2d, With<ManaText>>,
) {
    set_mana_ui(
        player_mana.current,
        player_mana.maximum,
        &config,
        fill_query,
        text_query,
    );
}

pub fn sync_health_value_to_ui(
    config: Res<GameUiConfig>,
    player_health: Single<&Health, (With<Health>, With<Player>)>,
    mut fill_query: Single<(&mut Sprite, &mut Transform), With<HealthFill>>,
    mut text_query: Single<&mut Text2d, With<HealthText>>,
) {
    set_health_ui(
        player_health.current,
        player_health.maximum,
        &config,
        fill_query,
        text_query,
    );
}
