#![allow(nonstandard_style)]

use bevy::prelude::*;
use bevy::sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::render::render_resource::*;
use bevy::math::primitives::Rectangle;
use bevy::shader::ShaderRef;
use bevy::window::Window;
use bevy::window::PrimaryWindow;
use bevy::ecs::entity_disabling::Disabled;
use rand::{distr::Uniform, prelude::*};

#[derive(Component)]
enum Mat {
     Mandelbrot,
     QuatJulia,
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<MandelbrotMaterial>::default())
        .add_plugins(Material2dPlugin::<QuatJuliaMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_time)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut material_mandelbrot: ResMut<Assets<MandelbrotMaterial>>,
    mut material_qjulia: ResMut<Assets<QuatJuliaMaterial>>,
)
{
    commands.spawn(Camera2d);
    let window = windows.single().unwrap();

    let res = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );
    let mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    commands.insert_resource(State {
        mu_a : Vec4::new(-0.278, -0.479,  0.0,   0.0),
        mu_b : Vec4::new( 0.278,  0.479,  0.0,   0.0),
        col_a : Vec3::new(0.24, 0.45, 1.0),
        col_b : Vec3::new(0.24, 0.45, 1.0),
        t : 0.0,
    });
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_qjulia.add(QuatJuliaMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
            mu: Vec4::ZERO,
            col: Vec3::ZERO,
        })),
        Mat::QuatJulia,
        Transform::from_scale(Vec3::new(2000.0, 2000.0, 1.0)), // big enough
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_mandelbrot.add(MandelbrotMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
        })),
        Mat::Mandelbrot,
        Transform::from_scale(Vec3::new(2000.0, 2000.0, 1.0)), // big enough
    ));
}

fn update_time(
    time: Res<Time>,
    mut commands: Commands,
    dmats: Query<(Entity, &Mat), With<Disabled>>,
    emats: Query<(Entity, &Mat), Without<Disabled>>,
    mut state: ResMut<State>,
    mut material_mandelbrot: ResMut<Assets<MandelbrotMaterial>>,
    mut material_qjulia: ResMut<Assets<QuatJuliaMaterial>>,
)
{
    let mut max_time = 0.0;
    for (_, mat) in material_qjulia.iter_mut() {
        mat.time += time.delta_secs();
        max_time = mat.time;
        let dt : f32 = time.delta_secs();

        let mut mu_c : Vec4 = Vec4::new(-0.278, -0.479, -0.231, 0.235);

        let mut col_c : Vec3 = Vec3::new(0.24, 0.45, 1.0);

        state.t += dt;

        if state.t >= 1.0
        {
            state.t = 0.0;
            let mut g = rand::rng();
            let mut rng = Uniform::new_inclusive(0.0, 1.0).unwrap().sample_iter(&mut g);

            state.mu_a[0] = state.mu_b[0];
            state.mu_a[1] = state.mu_b[1];
            state.mu_a[2] = state.mu_b[2];
            state.mu_a[3] = state.mu_b[3];

            state.mu_b[0] = rng.next().unwrap();
            state.mu_b[1] = rng.next().unwrap();
            state.mu_b[2] = rng.next().unwrap();
            state.mu_b[3] = rng.next().unwrap();

            state.col_a[0] = state.col_b[0];
            state.col_a[1] = state.col_b[1];
            state.col_a[2] = state.col_b[2];

            state.col_b[0] = rng.next().unwrap();
            state.col_b[1] = rng.next().unwrap();
            state.col_b[2] = rng.next().unwrap();
        }

        mu_c[0] = (1.0 - state.t) * state.mu_a[0] + state.t * state.mu_b[0];
        mu_c[1] = (1.0 - state.t) * state.mu_a[1] + state.t * state.mu_b[1];
        mu_c[2] = (1.0 - state.t) * state.mu_a[2] + state.t * state.mu_b[2];
        mu_c[3] = (1.0 - state.t) * state.mu_a[3] + state.t * state.mu_b[3];

        col_c[0] = (1.0 - state.t) * state.col_a[0] + state.t * state.col_b[0];
        col_c[1] = (1.0 - state.t) * state.col_a[1] + state.t * state.col_b[1];
        col_c[2] = (1.0 - state.t) * state.col_a[2] + state.t * state.col_b[2];

        mat.mu = mu_c;
        mat.col = col_c;
    }

    for (_, mat) in material_mandelbrot.iter_mut() {
        mat.time += time.delta_secs();
        if mat.time > max_time { max_time = mat.time; }
    }

    if max_time > 2.0 {
        let mut rng = rand::rng();
        let next_ind = (0..dmats.count()).choose(&mut rng);
        let next = dmats.iter().next();

        for (enabled, _) in emats {
            println!("Enabled: {:#?}", enabled);
            commands.entity(enabled).insert(Disabled);
        }
        next.map(|(n, _)| { println!("Disabled: {:#?}", n); commands.entity(n).remove::<Disabled>(); });
        for (_, mat) in material_mandelbrot.iter_mut() {
            mat.time = 0.0;
        }

        for (_, mat) in material_qjulia.iter_mut() {
            mat.time = 0.0;
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct MandelbrotMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
}

impl Material2d for MandelbrotMaterial {
    fn fragment_shader() -> ShaderRef {
       "shaders/fractal_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct QuatJuliaMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
    #[uniform(2)] mu : Vec4,
    #[uniform(3)] col : Vec3,
}

impl Material2d for QuatJuliaMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/qjulia_shader.wgsl".into()
    }
}

#[derive(Resource)]
struct State {
    mu_a : Vec4,
    mu_b : Vec4,
    col_a : Vec3,
    col_b : Vec3,
    t : f32,
}
