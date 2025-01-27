mod bullet;
mod common;
mod enemy;
mod level;
mod player;
mod ui;
mod wall;

use bullet::*;
use common::*;
use enemy::*;
use level::*;
use player::*;
use ui::*;
use wall::*;

use bevy::{prelude::*, time::FixedTimestep};
use bevy_inspector_egui::{prelude::*, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_event::<ExplosionEvent>()
        .add_state(AppState::StartMenu)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GameMode::SinglePlayer)
        .insert_resource(LevelSelection::Index(0))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rapier)
        .add_startup_system(setup_wall)
        .add_startup_system(setup_explosion_assets)
        .add_startup_system(setup_ldtk_world)
        .add_system_set(SystemSet::on_enter(AppState::StartMenu).with_system(setup_start_menu))
        .add_system_set(
            SystemSet::on_update(AppState::StartMenu)
                .with_system(start_game)
                .with_system(switch_game_mode),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::StartMenu)
                .with_system(despawn_screen::<OnStartMenuScreen>),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(setup_player1)
                .with_system(setup_player2)
                .with_system(setup_enemies)
                // .with_system(setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(players_attack)
                .with_system(animate_players)
                .with_system(animate_shield)
                .with_system(remove_shield)
                .with_system(animate_water)
                .with_system(spawn_explosion)
                .with_system(animate_explosion)
                .with_system(check_bullet_collision),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player1_move)
                .with_system(player2_move)
                .with_system(move_bullet),
        )
        .add_system_to_stage(CoreStage::PostUpdate, display_events)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn setup_ldtk_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        transform: Transform::from_xyz(-13.5 * 32., -9. * 32., 0.0),
        ..Default::default()
    });
}

// setup系统 添加entities到世界
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 地图项
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 300.0, 0.0),
        LevelItem::Home,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 350.0, 0.0),
        LevelItem::Tree,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 400.0, 0.0),
        LevelItem::Water,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 450.0, 0.0),
        LevelItem::IronWall,
    );
    spawn_level_item(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec3::new(0.0, BOTTOM_WALL + 500.0, 0.0),
        LevelItem::StoneWall,
    );

    commands
        .spawn(TransformBundle::from(Transform::from_xyz(
            200.0, 100.0, 0.0,
        )))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(80.0, 30.0));
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for _collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
    }

    for _contact_force_event in contact_force_events.iter() {
        // println!("Received contact force event: {:?}", contact_force_event);
    }
}
