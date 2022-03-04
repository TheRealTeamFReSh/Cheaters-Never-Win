use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use bevy_loading::{prelude::AssetsLoading, LoadingPlugin};

mod first_page;
mod second_page;

use crate::{
    cheat_codes::{CheatCodeKind, CheatCodeResource},
    states::GameStates,
    stats::GameStatsResource,
    toast::ShowToast,
};

pub struct TabMenuPlugin;
impl Plugin for TabMenuPlugin {
    fn build(&self, app: &mut App) {
        // assets loading
        app.add_plugin(LoadingPlugin {
            loading_state: GameStates::TabMenuLoading,
            next_state: GameStates::TabMenu,
        });

        app.add_system_set(
            SystemSet::on_enter(GameStates::TabMenuLoading)
                .with_system(load_assets)
                .with_system(build_base_ui),
        );

        // open menu trigger
        app.add_system_set(SystemSet::on_update(GameStates::Main).with_system(open_menu_trigger));
        app.add_system_set(
            SystemSet::on_update(GameStates::TabMenu)
                .with_system(close_menu_trigger)
                .with_system(switch_page),
        );

        // on enter
        app.add_system_set(
            SystemSet::on_enter(GameStates::TabMenu)
                .with_system(remind_second_page)
                .label("build_base_ui"),
        );
        app.add_system_set(
            SystemSet::on_enter(GameStates::TabMenu)
                .with_system(first_page::build_ui)
                .after("build_base_ui"),
        );

        // on exit
        app.add_system_set(SystemSet::on_exit(GameStates::TabMenu).with_system(destroy_menu));
    }
}

#[derive(Component)]
pub struct TabMenuComponent;

#[derive(Component)]
pub struct TabMenuContent;

fn switch_page(
    commands: Commands,
    mut assets: ResMut<TabMenuAssets>,
    mut keyboard: ResMut<Input<KeyCode>>,
    query: Query<Entity, With<TabMenuContent>>,
    window: Res<Windows>,
    cheat_codes_res: Res<CheatCodeResource>,
    stats_res: Res<GameStatsResource>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        // on the first page
        if assets.current_page == "first".to_string() {
            assets.current_page = "second".to_string();
            second_page::build_ui(commands, assets, query, cheat_codes_res, window);
        } else {
            assets.current_page = "first".to_string();
            first_page::build_ui(commands, assets, query, cheat_codes_res, stats_res, window);
        }

        keyboard.reset(KeyCode::Tab);
    }
}

fn destroy_menu(mut commands: Commands, query: Query<Entity, With<TabMenuComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn open_menu_trigger(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        game_state.push(GameStates::TabMenuLoading).unwrap();
        keyboard.reset(KeyCode::Tab);
    }
}

fn close_menu_trigger(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameStates>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.pop().unwrap();
        keyboard.reset(KeyCode::Tab);
        keyboard.reset(KeyCode::Escape);
    }
}

fn remind_second_page(mut ev_writer: EventWriter<ShowToast>) {
    ev_writer.send(ShowToast {
        value: "Press TAB to show the second page".to_string(),
        duration: Duration::from_secs(5),
    });
}

#[allow(dead_code)]
pub struct TabMenuAssets {
    first_page: Handle<Image>,
    second_page: Handle<Image>,
    font: Handle<Font>,
    font_2: Handle<Font>,
    icons: HashMap<CheatCodeKind, Handle<Image>>,
    current_page: String,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    cheat_codes_res: Res<CheatCodeResource>,
) {
    let first_page = asset_server.load("open_book.png");
    loading.add(&first_page);

    let second_page = asset_server.load("open_book_2.png");
    loading.add(&second_page);

    let font = asset_server.load("fonts/SpaceMadness.ttf");
    loading.add(&font);

    let font_2 = asset_server.load("fonts/VT323-Regular.ttf");
    loading.add(&font_2);

    let mut icons = HashMap::new();

    for (kind, code) in cheat_codes_res.codes.iter() {
        let icon = asset_server.load(&format!("cheat_codes/{}", code.image));
        loading.add(&icon);
        icons.insert(*kind, icon);
    }

    commands.insert_resource(TabMenuAssets {
        first_page,
        second_page,
        font,
        font_2,
        icons,
        current_page: "first".to_string(),
    })
}

fn build_base_ui(mut commands: Commands, window: Res<Windows>) {
    let current_window = window.get_primary().unwrap();

    // UI comps
    let parent_component = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()),
                Val::Px(current_window.height()),
            ),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 150).into(),
        ..Default::default()
    };

    // UI tree
    commands
        .spawn_bundle(parent_component)
        .insert(TabMenuComponent)
        .insert(TabMenuContent);
}
