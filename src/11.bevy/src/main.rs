use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_ratatui::crossterm::event::KeyCode;
use bevy_ratatui::event::KeyMessage;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use bevy_ratatui_camera::{
    RatatuiCamera, RatatuiCameraPlugin, RatatuiCameraStrategy, RatatuiCameraWidget,
};
use ratatui::layout::Alignment;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Widget};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1. / 60.)),
            RatatuiPlugins::default(),
            RatatuiCameraPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_scene)
        .add_systems(PreUpdate, handle_input)
        .add_systems(Update, (rotate_cube, draw_scene))
        .run();
}

#[derive(Component)]
struct Cube;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Cube,
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.54, 0.7),
            ..Default::default()
        })),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(3., 4., 6.),
    ));

    commands.spawn((
        RatatuiCamera::default(),
        RatatuiCameraStrategy::halfblocks(),
        Camera3d::default(),
        Transform::from_xyz(2.5, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}

fn rotate_cube(time: Res<Time>, mut cube: Single<&mut Transform, With<Cube>>) {
    cube.rotate_z(time.delta_secs());
    cube.rotate_x(time.delta_secs() * 0.5);
}

fn handle_input(mut messages: MessageReader<KeyMessage>, mut exit: MessageWriter<AppExit>) {
    for message in messages.read() {
        if let KeyCode::Char('q') | KeyCode::Esc = message.code {
            exit.write_default();
        }
    }
}

fn draw_scene(
    mut ratatui: ResMut<RatatuiContext>,
    mut camera_widget: Single<&mut RatatuiCameraWidget>,
) -> Result {
    ratatui.draw(|frame| {
        let block = Block::bordered()
            .bg(ratatui::style::Color::Black)
            .title_bottom("[q to quit]")
            .title_alignment(Alignment::Center);

        let inner = block.inner(frame.area());
        frame.render_widget(block, frame.area());

        camera_widget.render(inner, frame.buffer_mut());
    })?;

    Ok(())
}
