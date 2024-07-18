use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}, window::{PrimaryWindow, WindowResized},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct MandelbrotMaterial {
    #[uniform(0)]
    position: Vec2,
    #[uniform(1)]
    units_per_pixel: f32,
    #[uniform(2)]
    window_size: Vec2,
    #[uniform(3)]
    iteration: u32,
    iteration_f: f32,
}

impl Material2d for MandelbrotMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}

#[derive(Component)]
struct BackgroundRect;

fn main() {
    App::new()
        .add_plugins((
            bevy_mandelbrot::DefaultPluginsPatch,
            Material2dPlugin::<MandelbrotMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (input_system, on_window_resized))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();

    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_scale(window.size().extend(1.0)),
            material: materials.add(MandelbrotMaterial {
                position: Vec2::new(0.0, 0.0),
                units_per_pixel: 4.0 / window.size().y,
                window_size: window.size(),
                iteration: 40,
                iteration_f: 40.0,
            }),
            ..default()
        },
        BackgroundRect,
    ));
}

fn input_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
){
    for (_, m) in &mut materials.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            m.position.x -= m.units_per_pixel * 300. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            m.position.x += m.units_per_pixel * 300. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            m.position.y -= m.units_per_pixel * 300. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            m.position.y += m.units_per_pixel * 300. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            m.units_per_pixel *= 0.98;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            m.units_per_pixel /= 0.98;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            if 0.0 < m.iteration_f {
                m.iteration_f = (m.iteration_f - 20.0 * time.delta_seconds()).max(0.0);
                m.iteration = m.iteration_f as u32;
            }
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            if m.iteration_f < 100.0 {
                m.iteration_f = (m.iteration_f + 20.0 * time.delta_seconds()).min(100.0);
                m.iteration = m.iteration_f as u32;
            }
        }
    }
}

fn on_window_resized(
    mut resize_reader: EventReader<WindowResized>,
    mut materials: ResMut<Assets<MandelbrotMaterial>>,
    mut q_rect: Query<&mut Transform, With<BackgroundRect>>,
) {
    let mut rect = q_rect.single_mut();
    for ev in resize_reader.read() {
        for (_, m) in materials.iter_mut() {
            m.window_size = Vec2::new(ev.width, ev.height);
        }
        rect.scale = Vec3::new(ev.width, ev.height, 1.0);
    }
}
