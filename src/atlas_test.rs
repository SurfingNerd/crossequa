//! This example illustrates how to use `TextureAtlases` within ui

use bevy::{color::palettes::css::*, prelude::*, /* winit::WinitSettings */};

pub fn run_atlas_test() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, increment_atlas_index)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera
    commands.spawn(Camera2d);

    let text_font = TextFont::default();

    let texture_handle = asset_server.load("textures/Scifi_Panels_03_basecolor.png");

    //let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(128), 32, 32, None, None);

    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(1024, 820),
        4,
        4,
        None,
        Some(UVec2 { x: 0, y: 410 }),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // offset left: 1024
    // offsset top: 412

    // height 820

    // width: 1024

    // root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(text_font.font_size * 2.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageNode::from_atlas_image(
                    texture_handle,
                    TextureAtlas::from(texture_atlas_handle),
                ),
                Node {
                    width: Val::Px(256.),
                    height: Val::Px(256.),
                    ..default()
                },
                BackgroundColor(ANTIQUE_WHITE.into()),
                Outline::new(Val::Px(8.0), Val::ZERO, CRIMSON.into()),
            ));
            parent
                .spawn((Text::new("press "), text_font.clone()))
                .with_child((
                    TextSpan::new("space"),
                    TextColor(YELLOW.into()),
                    text_font.clone(),
                ))
                .with_child((TextSpan::new(" to advance frames"), text_font));
        });
}

fn increment_atlas_index(
    mut image_nodes: Query<&mut ImageNode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut inc: i64 = 0;

    if keyboard.just_pressed(KeyCode::ArrowDown) {
        inc = 32;
    }

    if keyboard.just_pressed(KeyCode::ArrowUp) {
        inc = -32;
    }

    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        inc = -1;
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        inc = 1;
    }

    for mut image_node in &mut image_nodes {
        if let Some(atlas) = &mut image_node.texture_atlas {
            let mut new_index = atlas.index as i64 + inc;

            if new_index < 0 {
                atlas.index = 0;
            } else {
                atlas.index = (new_index % (32 * 32)) as usize;
            }
             
            // bevy::log::debug!("atlas index: {:?}", atlas.index );
        }
    }
}
