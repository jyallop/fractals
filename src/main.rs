#![allow(nonstandard_style)]

use bevy::prelude::*;
use bevy::sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::render::render_resource::*;
use bevy::math::primitives::Rectangle;
use bevy::shader::ShaderRef;
use bevy::window::Window;
use bevy::window::PrimaryWindow;
use rand::Rng;

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
            mu: Vec4::ZERO,
            col: Vec3::ZERO,
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
    static mut dt : f32 = 0.01;
    static mut T : f32 = 0.0;

    static mut mu_a : Vec4 = Vec4::new(-0.278, -0.479,  0.0,   0.0);
    static mut mu_b : Vec4 = Vec4::new( 0.278,  0.479,  0.0,   0.0);
    static mut mu_c : Vec4 = Vec4::new(-0.278, -0.479, -0.231, 0.235);

    static mut col_a : Vec3 = Vec3::new(0.24, 0.45, 1.0);
    static mut col_b : Vec3 = Vec3::new(0.24, 0.45, 1.0);
    static mut col_c : Vec3 = Vec3::new(0.24, 0.45, 1.0);

    let mut rng = rand::thread_rng();

    for (_, mat) in materials.iter_mut() {
        mat.time = time.elapsed_secs();

        unsafe {
            T += dt;

            if T >= 1.0
            {
                T = 0.0;

                mu_a[0] = mu_b[0];
                mu_a[1] = mu_b[1];
                mu_a[2] = mu_b[2];
                mu_a[3] = mu_b[3];

                mu_b[0] = rng.gen_range(0.0..1.0);
                mu_b[1] = rng.gen_range(0.0..1.0);
                mu_b[2] = rng.gen_range(0.0..1.0);
                mu_b[3] = rng.gen_range(0.0..1.0);

                println!("Next mu");

                col_a[0] = col_b[0];
                col_a[1] = col_b[1];
                col_a[2] = col_b[2];

                col_b[0] = rng.gen_range(0.0..1.0);
                col_b[1] = rng.gen_range(0.0..1.0);
                col_b[2] = rng.gen_range(0.0..1.0);
            }

            mu_c[0] = (1.0 - T) * mu_a[0] + T * mu_b[0];
            mu_c[1] = (1.0 - T) * mu_a[1] + T * mu_b[1];
            mu_c[2] = (1.0 - T) * mu_a[2] + T * mu_b[2];
            mu_c[3] = (1.0 - T) * mu_a[3] + T * mu_b[3];

            col_c[0] = (1.0 - T) * col_a[0] + T * col_b[0];
            col_c[1] = (1.0 - T) * col_a[1] + T * col_b[1];
            col_c[2] = (1.0 - T) * col_a[2] + T * col_b[2];

            mat.mu = mu_c;
            mat.col = col_c;
        }

    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct FullscreenMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
    #[uniform(2)] mu : Vec4,
    #[uniform(3)] col : Vec3,
}

impl Material2d for FullscreenMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/qjulia_shader.wgsl".into()
    }
}
