mod components;
mod entities;
mod frame_cnt;
mod frame_limiter;
mod game_events;
mod inventory;
mod keyboard;
mod levels;
mod systems;

use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy::window;
// use bevy::render::pass::ClearColor;
use frame_cnt::FrameCntPlugin;
use frame_limiter::FrameLimiterPlugin;
use game_events::GameEvents;
use inventory::Inventory;
use keyboard::KeyboardPlugin;
use levels::{LevelInfo, LevelSet, LevelSetLoader};
use systems::{
    activate_capsule_system, asset_events, create_sprites, damage_system, force_field_system,
    game_event_system, level_setup, magnetic_field_system, move_robbo, move_system, prepare_render,
    render_setup, shot_system, tick_system,
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
        .add_resource(bevy::render::pass::ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_resource(TextureAtlasHandle(None))
        .add_resource(Inventory::default())
        .add_resource(GameEvents::default())
        .add_resource(LevelInfo::default())
        .add_default_plugins()
        .add_asset::<LevelSet>()
        .add_asset_loader::<LevelSet, LevelSetLoader>()
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
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
        .add_stage_after("keyboard", "magnetic_field")
        .add_stage_after("frame_cnt", "tick")
        .add_system_to_stage("magnetic_field", magnetic_field_system.system())
        .add_system_to_stage(stage::EVENT_UPDATE, asset_events.system())
        .add_system_to_stage("pre_update1", game_event_system.system())
        .add_system_to_stage("pre_update2", shot_system.system())
        .add_system_to_stage(stage::UPDATE, move_system.system())
        .add_system_to_stage("move_robbo", move_robbo.system()) // it must be after move_system
        .add_system_to_stage(stage::POST_UPDATE, create_sprites.system())
        .add_system_to_stage("post_update2", prepare_render.system())
        .add_system_to_stage("events", activate_capsule_system.system())
        .add_system_to_stage("tick", tick_system.system())
        .add_system_to_stage("tick", damage_system.system())
        .add_system_to_stage("tick", force_field_system.system())
        .run();
}
