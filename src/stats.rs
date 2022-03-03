use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBodyVelocityComponent;

use crate::{cheat_codes::CheatCodeResource, runner::Player, states::GameStates};

pub struct GameStatsPlugin;

impl Plugin for GameStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyKilledEvent>();
        app.insert_resource(GameStatsResource::new());
        app.add_system(enemy_killed_handler);
        app.add_system(update_max_distance);
        app.add_system(update_cheats_activated);
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(update_run_time));
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(update_avg_speed));
    }
}

pub struct GameStatsResource {
    pub distance: f32,
    pub enemy_killed: usize,
    pub enemy_score: usize,
    pub cheats_activated: usize,
    pub run_time: f64,
    pub avg_speed: f32,
}

impl GameStatsResource {
    fn new() -> Self {
        Self {
            distance: 0.,
            enemy_killed: 0,
            enemy_score: 0,
            cheats_activated: 0,
            run_time: 0.,
            avg_speed: 0.,
        }
    }

    pub fn get_score(&self) -> usize {
        let distance_coef = 2.0;
        let enemy_coef = 1.0;
        let time_coef = 1.0;
        let cheats_coef = 10.0;

        (self.distance * distance_coef
            + self.enemy_score as f32 * enemy_coef
            + self.run_time as f32 * time_coef
            + self.cheats_activated as f32 * cheats_coef) as usize
    }
}

pub struct EnemyKilledEvent(usize);

pub fn enemy_killed_handler(
    mut enemy_event_reader: EventReader<EnemyKilledEvent>,
    mut stats_res: ResMut<GameStatsResource>,
) {
    for EnemyKilledEvent(score) in enemy_event_reader.iter() {
        stats_res.enemy_killed += 1;
        stats_res.enemy_score += score;
    }
}

pub fn update_max_distance(
    player_query: Query<&Transform, With<Player>>,
    mut stats_res: ResMut<GameStatsResource>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        stats_res.distance = stats_res
            .distance
            .max(player_transform.translation.x / 100.);
    }
}

pub fn update_cheats_activated(
    cheat_codes_res: Res<CheatCodeResource>,
    mut stats_res: ResMut<GameStatsResource>,
) {
    // if nothing changed, skip
    if !cheat_codes_res.is_changed() {
        return;
    }

    stats_res.cheats_activated = cheat_codes_res
        .codes
        .iter()
        .filter(|(kind, _)| cheat_codes_res.is_code_activated(kind))
        .count()
}

pub fn update_run_time(time: Res<Time>, mut stats_res: ResMut<GameStatsResource>) {
    stats_res.run_time += time.delta_seconds_f64();
}

pub fn update_avg_speed(
    time: Res<Time>,
    mut query: Query<&mut RigidBodyVelocityComponent, With<Player>>,
    mut stats_res: ResMut<GameStatsResource>,
) {
    let velocity: Vec2 = query.get_single_mut().unwrap().linvel.into();
    let avg_dist = stats_res.run_time as f32 * stats_res.avg_speed;
    stats_res.avg_speed = (avg_dist + velocity.x * (time.delta_seconds_f64() as f32))
        / (stats_res.run_time as f32 + time.delta_seconds_f64() as f32);
}
