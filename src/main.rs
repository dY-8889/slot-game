use bevy::prelude::*;

use rand::{seq::SliceRandom, thread_rng};

use ItemKind::*;

const ITEM_SPEED: f32 = 20.0;
// アイテムのサイズ
const ITEM_SCALE: Vec2 = Vec2::new(200.0, 200.0);
// アイテムのx座標
const ITEM_LOCATIONS: [f32; 3] = [-230., 0., 230.];

const ITEM_LIST: [ItemKind; 5] = [Apple, Mikan, Grape, Banana, Pine];

// アイテム同士の余白
const ITEM_PADDING: f32 = 17.0;
// アイテムがスポーンする座標
const ITEM_SPAWN_POINT: f32 = (ITEM_SCALE.y + ITEM_PADDING) * 2.0;
// アイテムがデスポーンする場所
const ITEM_DESPAWN_POINT: f32 =
    -((ITEM_SCALE.y + ITEM_PADDING) * ITEM_LIST.len() as f32) + ITEM_SPAWN_POINT;

const Y_RANGE: f32 = (ITEM_SCALE.y + ITEM_PADDING) / 1.5;

// ボタンのサイズ
const BUTTON_WIDTH: Val = Val::Px(140.0);
const BUTTON_HEIGHT: Val = Val::Px(70.0);

const FRAME_LEFT_WIDTH: Val = Val::Percent(15.0);
const FRAME_RIGHT_WIDHT: Val = Val::Percent(15.0);
const FRAME_TOP_HEIGHT: Val = Val::Percent(20.0);
const FRAME_BOTTOM_HEIGHT: Val = Val::Percent(20.0);

const FRAME_COLOR: Color = Color::WHITE;
const BUTTON_COLOR: Color = Color::BLUE;
const BUTTON_PRESSED_COLOR: Color = Color::RED;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1100., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_event::<ItemStopEvent>()
        .add_event::<SoundEvent>()
        .insert_resource(ItemMoveFlag {
            item1: true,
            item2: true,
            item3: true,
        })
        .init_resource::<ItemEq>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, keyboard_input, sound_event))
        .add_systems(Update, (item_move, item_stop))
        .run();
}

#[derive(Resource, Default, Debug)]
struct ItemEq {
    item1: Option<ItemKind>,
    item2: Option<ItemKind>,
    item3: Option<ItemKind>,
}

#[derive(Resource, Default)]
struct ItemMoveFlag {
    item1: bool,
    item2: bool,
    item3: bool,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
enum ItemKind {
    Apple,
    Mikan,
    Grape,
    Banana,
    Pine,
}

enum FrameLocation {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Component)]
struct ItemLocation(usize);

#[derive(Component)]
struct StartButton(usize);

#[derive(Component)]
struct StopButton(usize);

#[derive(Event, Default)]
struct ItemStopEvent(usize);

type Sound = Handle<AudioSource>;

#[derive(Resource)]
struct Sounds {
    eq: Sound,
    stop: Sound,
}

impl Sounds {
    fn get(&self, audio: &Audio) -> Handle<AudioSource> {
        let sound = match audio {
            Audio::Arrange => &self.eq,
            Audio::Stop => &self.stop,
        };
        sound.clone()
    }
}

#[derive(Event)]
struct SoundEvent(Audio);

enum Audio {
    Arrange,
    Stop,
}

#[derive(Bundle)]
struct StopButtonBundle {
    button_bundle: ButtonBundle,
    id: StopButton,
}

#[derive(Bundle)]
struct FrameBunde {
    node_bundle: NodeBundle,
}

#[derive(Bundle)]
struct ItemBundle {
    sprite_bundle: SpriteBundle,
    id: ItemLocation,
    kind: ItemKind,
}

impl ItemEq {
    fn eq(&self) -> bool {
        if self.item1 == self.item2 && self.item2 == self.item3 {
            return true;
        }
        false
    }
    fn change(&mut self, location: usize, kind: ItemKind) {
        let item = match location {
            1 => &mut self.item1,
            2 => &mut self.item2,
            3 => &mut self.item3,
            _ => panic!(),
        };

        *item = Some(kind);
    }
    fn reset(&mut self) {
        self.item1 = None;
        self.item2 = None;
        self.item3 = None;
    }
}

impl ItemMoveFlag {
    fn change(&mut self, item: usize) {
        let flag = match item {
            1 => &mut self.item1,
            2 => &mut self.item2,
            3 => &mut self.item3,
            _ => panic!(),
        };
        *flag = false;
    }
    fn get_location(&self, location: usize) -> bool {
        match location {
            1 => self.item1,
            2 => self.item2,
            3 => self.item3,
            _ => panic!(),
        }
    }
    fn all_true(&mut self) {
        self.item1 = true;
        self.item2 = true;
        self.item3 = true;
    }
}

impl ItemKind {
    fn texture(&self) -> String {
        let str = match self {
            Apple => "apple.png",
            Mikan => "mikankun.png",
            Grape => "budou.png",
            Banana => "banana.png",
            Pine => "pine.png",
        };
        "images/".to_string() + str
    }
    const fn scale(&self) -> Vec2 {
        match self {
            Apple => Vec2::new(1.5, 1.5),
            Mikan => Vec2::new(1., 1.),
            Grape => Vec2::new(1., 1.),
            Banana => Vec2::new(1., 1.),
            Pine => Vec2::new(1., 1.),
        }
    }
}

impl ItemBundle {
    fn new(kind: ItemKind, texture: Handle<Image>, location: usize, number: usize) -> ItemBundle {
        let y =
            ITEM_DESPAWN_POINT + (ITEM_PADDING + ITEM_SCALE.y) * number as f32 + ITEM_SPAWN_POINT;
        let translation = Vec2::new(ITEM_LOCATIONS[location - 1], y);

        ItemBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: translation.extend(0.0),
                    scale: ITEM_SCALE.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    // color: Color::BLUE,
                    custom_size: Some(kind.scale()),
                    ..default()
                },
                texture,
                ..default()
            },
            id: ItemLocation(location),
            kind,
        }
    }
}

impl FrameBunde {
    fn new(location: FrameLocation) -> NodeBundle {
        NodeBundle {
            style: location.style(),
            background_color: FRAME_COLOR.into(),
            ..default()
        }
    }
}

impl StopButtonBundle {
    fn new(id: usize) -> StopButtonBundle {
        StopButtonBundle {
            button_bundle: ButtonBundle {
                style: Style {
                    width: BUTTON_WIDTH,
                    height: BUTTON_HEIGHT,
                    margin: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: BUTTON_COLOR.into(),
                ..default()
            },
            id: StopButton(id),
        }
    }
}

impl FrameLocation {
    fn style(&self) -> Style {
        match self {
            FrameLocation::Left => Style {
                width: FRAME_LEFT_WIDTH,
                height: Val::Percent(100.),
                justify_self: JustifySelf::Start,
                ..default()
            },
            FrameLocation::Right => Style {
                width: FRAME_RIGHT_WIDHT,
                height: Val::Percent(100.),
                justify_self: JustifySelf::End,
                ..default()
            },
            FrameLocation::Top => Style {
                width: Val::Percent(100.),
                height: FRAME_TOP_HEIGHT,
                align_self: AlignSelf::Start,
                ..default()
            },
            FrameLocation::Bottom => Style {
                width: Val::Percent(100.),
                height: FRAME_BOTTOM_HEIGHT,
                align_self: AlignSelf::End,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(Sounds {
        eq: asset_server.load("sound/success.ogg"),
        stop: asset_server.load("sound/stop.ogg"),
    });

    commands.spawn(FrameBunde::new(FrameLocation::Left));
    commands.spawn(FrameBunde::new(FrameLocation::Right));
    commands.spawn(FrameBunde::new(FrameLocation::Top));
    commands
        .spawn(FrameBunde::new(FrameLocation::Bottom))
        .with_children(|parent| {
            parent.spawn(StopButtonBundle::new(1));
            parent.spawn(StopButtonBundle::new(2));
            parent.spawn(StopButtonBundle::new(3));
        });

    let mut item_list = ITEM_LIST;
    for location in 1..=3 {
        item_list.shuffle(&mut thread_rng());

        for (padding, item) in item_list.iter().enumerate() {
            commands.spawn(ItemBundle::new(
                *item,
                asset_server.load(item.texture()),
                location,
                padding,
            ));
        }
    }
}

fn item_move(mut item_query: Query<(&mut Transform, &ItemLocation)>, move_flag: Res<ItemMoveFlag>) {
    for (mut transform, item) in &mut item_query {
        if move_flag.get_location(item.0) {
            if transform.translation.y <= ITEM_DESPAWN_POINT {
                transform.translation.y = ITEM_SPAWN_POINT
            }
            transform.translation.y -= ITEM_SPEED;
        }
    }
}

fn keyboard_input(
    key: Res<Input<KeyCode>>,
    mut move_flag: ResMut<ItemMoveFlag>,
    mut item_eq: ResMut<ItemEq>,
    mut item_stop_event: EventWriter<ItemStopEvent>,
) {
    if key.just_pressed(KeyCode::J) {
        item_stop_event.send(ItemStopEvent(1));
        move_flag.change(1)
    }
    if key.just_pressed(KeyCode::K) {
        item_stop_event.send(ItemStopEvent(2));
        move_flag.change(2)
    }
    if key.just_pressed(KeyCode::L) {
        item_stop_event.send(ItemStopEvent(3));
        move_flag.change(3)
    }
    if key.just_pressed(KeyCode::Space) {
        move_flag.all_true();
        item_eq.reset();
    }
}

fn button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut StopButton),
        (Changed<Interaction>, With<StopButton>),
    >,
    mut move_flag: ResMut<ItemMoveFlag>,
    mut item_stop_event: EventWriter<ItemStopEvent>,
) {
    for (interaction, mut background, button_id) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                // アイテムを止める時時の処理
                if move_flag.get_location(button_id.0) {
                    item_stop_event.send(ItemStopEvent(button_id.0));
                }

                // 押したボタンの位置のmoveflagをfalseにする
                move_flag.change(button_id.0);

                // ボタンの色を変更する
                background.0 = BUTTON_PRESSED_COLOR;
            }
            // ボタンを押していないときの色
            _ => background.0 = BUTTON_COLOR,
        }
    }
}

fn item_stop(
    mut item_query: Query<(&mut Transform, &ItemLocation, &ItemKind)>,
    mut item_stop_event: EventReader<ItemStopEvent>,
    mut sound_event: EventWriter<SoundEvent>,
    mut item_eq: ResMut<ItemEq>,
) {
    for stop_location in item_stop_event.read() {
        let stop = stop_location.0;
        // どれくらいずらすか
        let mut difference: f32 = 0.0;

        for (transform, location, kind) in &item_query {
            // 押した行といま取得しているアイテムが同じ行にあるか
            if stop == location.0 {
                // アイテムのy座標
                let y = transform.translation.y;

                // あいてむが指定した範囲にあったら
                if -Y_RANGE <= y && y <= Y_RANGE {
                    item_eq.change(stop, *kind);

                    if item_eq.eq() {
                        sound_event.send(SoundEvent(Audio::Arrange));
                    } else {
                        sound_event.send(SoundEvent(Audio::Stop));
                    }

                    difference = y;

                    break;
                }
            }
        }
        for (mut transform, location, _) in &mut item_query {
            if stop == location.0 {
                // ずらす
                transform.translation.y -= difference;
            }
        }
    }
}

fn sound_event(
    mut commands: Commands,
    mut sound_event: EventReader<SoundEvent>,
    sound: Res<Sounds>,
) {
    for audio in sound_event.read() {
        commands.spawn(AudioBundle {
            source: sound.get(&audio.0),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
