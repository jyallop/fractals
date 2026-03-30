use bevy::prelude::*;
use bevy::sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::render::render_resource::*;
use bevy::math::primitives::Rectangle;
use bevy::shader::ShaderRef;
use bevy::window::Window;
use bevy::window::PrimaryWindow;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<FullscreenMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_resolution)
        .add_systems(Update, update_time)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FullscreenMaterial>>,
)
{
    commands.spawn(Camera2d);

    let mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(FullscreenMaterial {
            resolution: Vec2::ZERO, // will be set next frame
            time : 0.0,
        })),
        Transform::from_scale(Vec3::new(2000.0, 2000.0, 1.0)), // big enough
    ));
}

fn update_resolution(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<FullscreenMaterial>>,
) 
{
    let window = windows.single().unwrap();

    let res = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    for (_, mat) in materials.iter_mut() {
        mat.resolution = res;
    }
}

fn update_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<FullscreenMaterial>>,
)
{
    for (_, mat) in materials.iter_mut() {
        mat.time = time.elapsed_secs();
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct FullscreenMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
}

impl Material2d for FullscreenMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/qjulia_shader.wgsl".into()
    }
}
