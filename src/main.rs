mod components;
mod entities;
mod frame_cnt;
mod frame_limiter;
mod game_events;
mod inventory;
mod levels;
mod systems;

use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy::window;
use components::{Capsule, Position, Usable};
use frame_cnt::FrameCntPlugin;
use frame_limiter::FrameLimiterPlugin;
use game_events::GameEvents;
use inventory::Inventory;
use levels::{create_level, Level, LevelLoader};
use systems::{
    create_sprites, game_event_system, move_robbo, move_system, prepare_render, render_setup,
    shot_system, KeyboardPlugin,
};

mod consts {
    pub const WIDTH: i32 = 31;
    pub const HEIGHT: i32 = 16;
    pub const SCALE: f32 = 1.5;
    pub const FPS: f32 = 30.0;
    pub const BOX_SIZE: f32 = 32.0 * SCALE;
}

use consts::*;

use bevy::asset::AddAsset;

fn level_setup(asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    let _level_handle: Handle<Level> = asset_server.load("assets/01.txt").unwrap();
    println!("handle: {:?}", _level_handle);
}

pub struct TextureAtlasHandle(pub Option<Handle<TextureAtlas>>);

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Robbo".to_string(),
            width: ((32 * consts::WIDTH) as f32 * SCALE) as u32,
            height: ((32 * HEIGHT) as f32 * SCALE) as u32,
            vsync: true,
            resizable: false,
            mode: window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(TextureAtlasHandle(None))
        .add_resource(Inventory::default())
        .add_resource(GameEvents::default())
        .add_default_plugins()
        .add_asset::<Level>()
        .add_asset_loader::<Level, LevelLoader>()
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .add_plugin(FrameLimiterPlugin { fps: FPS })
        .add_plugin(FrameCntPlugin)
        .add_plugin(KeyboardPlugin)
        .add_startup_system(render_setup.system())
        .add_startup_system(level_setup.system())
        .add_stage_after(stage::UPDATE, "move_robbo")
        .add_stage_after("move_robbo", "events")
        .add_stage_after(stage::PRE_UPDATE, "pre_update1")
        .add_stage_after("pre_update1", "pre_update2")
        .add_stage_after(stage::POST_UPDATE, "post_update2")
        .add_system_to_stage(stage::EVENT_UPDATE, asset_events.system())
        .add_system_to_stage("pre_update1", game_event_system.system())
        .add_system_to_stage("pre_update2", shot_system.system())
        .add_system_to_stage(stage::UPDATE, move_system.system())
        .add_system_to_stage("move_robbo", move_robbo.system()) // it must be after move_system
        .add_system_to_stage(stage::POST_UPDATE, create_sprites.system())
        .add_system_to_stage("post_update2", prepare_render.system())
        .add_system_to_stage("events", activate_capsule_system.system())
        .run();
}

#[derive(Default)]
pub struct AssetEventsState {
    reader: EventReader<AssetEvent<Level>>,
}

pub fn asset_events(
    mut commands: Commands,
    mut game_events: ResMut<GameEvents>,
    mut state: Local<AssetEventsState>,
    levels: ResMut<Assets<Level>>,
    events: Res<Events<AssetEvent<Level>>>,
    mut items: Query<With<Position, Entity>>,
) {
    for event in state.reader.iter(&events) {
        let handle = match event {
            AssetEvent::Created { handle } => handle,
            AssetEvent::Modified { handle } => handle,
            _ => continue,
        };
        if let Some(level) = levels.get(handle) {
            game_events.flush();
            create_level(&mut commands, &mut items, level);
        }
    }
}

pub fn activate_capsule_system(
    mut commands: Commands,
    inventory: Res<Inventory>,
    mut query: Query<With<Capsule, Without<Usable, Entity>>>,
) {
    for capsule in &mut query.iter() {
        if inventory.screws > 0 {
            println!("activating capsule");
            entities::repair_capsule(&mut commands, capsule);
        }
    }
}
