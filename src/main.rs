#![allow(nonstandard_style)]

use bevy::prelude::{*, ops::cos};
use bevy::sprite_render::{Material2d, AlphaMode2d, Material2dPlugin, MeshMaterial2d};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::render::render_resource::*;
use bevy::math::primitives::Rectangle;
use bevy::shader::ShaderRef;
use bevy::window::{Window, PrimaryWindow};
use bevy::ecs::entity_disabling::Disabled;
use rand::{distr::Uniform, prelude::*};
use std::f32::consts::PI;
use bevy::window::WindowMode;
use std::cmp::min;
use bevy::text::{FontSmoothing, LineBreak, TextBounds};

#[derive(Component)]
struct Mat;

#[derive(Component)]
struct Fader;

#[derive(Component)]
struct Data;

static CYCLE_TIME : f32 = 10.0;
static TEXT_ROWS : f32 = 10.0;

fn main()
{
    App::new()
//        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
//                mode: WindowMode::Fullscreen(MonitorSelection::Index(0), VideoModeSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Material2dPlugin::<MandelbrotMaterial>::default())
        .add_plugins(Material2dPlugin::<QuatJuliaMaterial>::default())
        .add_plugins(Material2dPlugin::<MandelbulbMaterial>::default())
        .add_plugins(Material2dPlugin::<MengerSpongeMaterial>::default())
        .add_plugins(Material2dPlugin::<MandelboxMaterial>::default())
        .add_plugins(Material2dPlugin::<IfsMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (update_time, update_state, fader, update_text))
        //.add_systems(Update, (update_time, update_state))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut material_mandelbrot: ResMut<Assets<MandelbrotMaterial>>,
    mut material_mandelbulb: ResMut<Assets<MandelbulbMaterial>>,
    mut material_qjulia: ResMut<Assets<QuatJuliaMaterial>>,
    mut material_ifs: ResMut<Assets<IfsMaterial>>,
    mut material_sponge: ResMut<Assets<MengerSpongeMaterial>>,
    mut material_mandelbox: ResMut<Assets<MandelboxMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>
)
{
    commands.spawn(Camera2d);
    let window = windows.single().unwrap();
    let height = window.physical_height() as f32;
    let width = window.physical_width() as f32;

    let res = Vec2::new(
        width,
        height
    );
    let mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    commands.insert_resource(State {
        mu_a : Vec4::new(-0.278, -0.479,  0.0,   0.0),
        mu_b : Vec4::new( 0.278,  0.479,  0.0,   0.0),
        col_a : Vec3::new(0.24, 0.45, 1.0),
        col_b : Vec3::new(0.24, 0.45, 1.0),
        s_a : 1.0,
        s_b : 1.0,
        h_a : 10.0,
        h_b : 10.0,
        base_color : Vec3::new(0.24, 0.45, 1.0),
        t : 0.0,
        time : 0.0,
        data : String::new(),
    });

    let text_box =
        (Text2d::new("Hello!"),
        TextFont {
            font_size: height / TEXT_ROWS,
            ..default()
        },
        Data,
        Fader,
        TextColor(Color::WHITE),
         TextLayout::new(Justify::Left, LineBreak::AnyCharacter),
         // Wrap text in the rectangle
         TextBounds::from(Vec2::new(width, height)),
         Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        );

    commands.spawn(text_box);
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(
            materials.add(ColorMaterial {
                color: Color::srgba(0.0, 0.0, 0.0, 1.0),
                alpha_mode: AlphaMode2d::Blend, // Required for transparency!
                ..default()
            })
        ),
        Fader,
        Transform {
            scale: Vec3::new(window.physical_width() as f32, window.physical_height() as f32, 0.0),
            translation: Vec3::new(0.0, 0.0, 2.0),
            ..default()
        },
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_qjulia.add(QuatJuliaMaterial {
            resolution: res.clone(), // will be set next frame
            time: 0.0,
            mu: Vec4::ZERO,
            col: Vec3::ZERO,
        })),
        Mat,
        Transform::from_scale(Vec3::new(width, height, 1.0)),
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_mandelbrot.add(MandelbrotMaterial {
            resolution: res.clone(), // will be set next frame
            time: 0.0,
            base_color: Vec3::ZERO,
            h_factor: 10.0,
        })),
        Mat,
        Transform::from_scale(Vec3::new(width, height, 1.0)),
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_ifs.add(IfsMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
        })),
        Mat,
        Transform::from_scale(Vec3::new(width, height, 1.0)),
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_mandelbulb.add(MandelbulbMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
            base_color : Vec3::new(0.0, 0.0, 0.0),
        })),
        Mat,
        Transform::from_scale(Vec3::new(width, height, 1.0)),
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_sponge.add(MengerSpongeMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
            base_color : Vec3::new(0.0, 0.0, 0.0),
            sponge_s : 1.0,
        })),
        Mat,
        Transform::from_scale(Vec3::new(width, height, 1.0)),
    ));

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_mandelbox.add(MandelboxMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
            base_color : Vec3::new(0.0, 0.0, 0.0),
        })),
        Mat,
        Transform::from_scale(Vec3::new(width, height, 1.0)),
    ));
}

fn update_state(
    time: Res<Time>,
    mut commands: Commands,
    mut state: ResMut<State>) {

    let dt : f32 = time.delta_secs();
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

        state.mu_b[0] = 2.0 * rng.next().unwrap() - 1.0; // [-1, 1)
        state.mu_b[1] = 2.0 * rng.next().unwrap() - 1.0; // [-1, 1)
        state.mu_b[2] = 2.0 * rng.next().unwrap() - 1.0; // [-1, 1)
        state.mu_b[3] = 2.0 * rng.next().unwrap() - 1.0; // [-1, 1)

        state.col_a[0] = state.col_b[0];
        state.col_a[1] = state.col_b[1];
        state.col_a[2] = state.col_b[2];

        state.col_b[0] = 0.8 * rng.next().unwrap() + 0.2; // [0.2, 1.0)
        state.col_b[1] = 0.8 * rng.next().unwrap() + 0.2; // [0.2, 1.0)
        state.col_b[2] = 0.8 * rng.next().unwrap() + 0.2; // [0.2, 1.0)

        state.s_a = state.s_b;

        state.s_b = 2.0 * rng.next().unwrap() + 1.0; // [1, 3)

        state.h_a = state.h_b;
        state.h_b = 90.0 * rng.next().unwrap() + 10.0; // [10, 100)

        for i in 0..2 {
            let addition = &(state.col_b[i].to_string());
            state.data += &("|".to_owned() + addition);
            let addition_two = &(state.col_a[i].to_string());
            state.data += &("|".to_owned() + addition_two);
        }
    }

    state.base_color[0] = (1.0 - state.t) * state.col_a[0] + state.t * state.col_b[0];
    state.base_color[1] = (1.0 - state.t) * state.col_a[1] + state.t * state.col_b[1];
    state.base_color[2] = (1.0 - state.t) * state.col_a[2] + state.t * state.col_b[2];
}

fn update_text(
    mut state: ResMut<State>,
    mut query: Query<&mut Text2d, With<Data>>,
) {
    for mut span in &mut query {
        **span = state.data.clone();
    }
    if state.data.len() > 0 {
        state.data = state.data[1..].to_string();
    }
}
fn fader(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&MeshMaterial2d<ColorMaterial>, With<Fader>>,
    mut text_query: Query<&mut TextColor, With<Fader>>,
    mut state: ResMut<State>,
) {
    let alpha = 0.5 * (cos(2.0 * PI * time.elapsed_secs() / CYCLE_TIME) + 1.0);
    for handle in &query {
        if let Some(material) = materials.get_mut(&handle.0) {
            material.color.set_alpha(alpha);
        }
    }
    for mut text_color in &mut text_query {
        text_color.0.set_alpha(alpha);
    }
}

fn update_time(
    time: Res<Time>,
    mut commands: Commands,
    dmats: Query<(Entity, &Mat), With<Disabled>>,
    emats: Query<(Entity, &Mat), Without<Disabled>>,
    mut state: ResMut<State>,
    mut material_mandelbrot: ResMut<Assets<MandelbrotMaterial>>,
    mut material_qjulia: ResMut<Assets<QuatJuliaMaterial>>,
    mut material_mandelbulb: ResMut<Assets<MandelbulbMaterial>>,
    mut material_ifs: ResMut<Assets<IfsMaterial>>,
    mut material_sponge: ResMut<Assets<MengerSpongeMaterial>>,
    mut material_mandelbox: ResMut<Assets<MandelboxMaterial>>,
)
{
    state.time += time.delta_secs();
    for (_, mat) in material_qjulia.iter_mut() {
        mat.time += time.delta_secs();

        let mut mu_c : Vec4 = Vec4::new(-0.278, -0.479, -0.231, 0.235);

        let mut col_c : Vec3 = Vec3::new(0.24, 0.45, 1.0);

        mu_c[0] = (1.0 - state.t) * state.mu_a[0] + state.t * state.mu_b[0];
        mu_c[1] = (1.0 - state.t) * state.mu_a[1] + state.t * state.mu_b[1];
        mu_c[2] = (1.0 - state.t) * state.mu_a[2] + state.t * state.mu_b[2];
        mu_c[3] = (1.0 - state.t) * state.mu_a[3] + state.t * state.mu_b[3];

        mat.mu = mu_c;
        mat.col = state.base_color;
        mat.resolution.x -= 10.0;
        mat.resolution.y -= 10.0;
    }

    for (_, mat) in material_mandelbrot.iter_mut() {
        mat.time += time.delta_secs();
        mat.base_color = state.base_color;
        mat.h_factor = (1.0 - state.t) * state.h_a + state.t * state.h_b;
    }

    for (_, mat) in material_mandelbulb.iter_mut() {
        mat.time += time.delta_secs();
        mat.base_color = state.base_color;
    }

    for (_, mat) in material_sponge.iter_mut() {
        mat.time += time.delta_secs();
        mat.base_color = state.base_color;
        mat.sponge_s = (1.0 - state.t) * state.s_a + state.t * state.s_b;
    }

    for (_, mat) in material_mandelbox.iter_mut() {
        mat.time += time.delta_secs();
        mat.base_color = state.base_color;
    }

    for (_, mat) in material_ifs.iter_mut() {
        mat.time += time.delta_secs();
    }

    if state.time > CYCLE_TIME {
        let mut rng = rand::rng();
        let next_ind = if dmats.is_empty() { Some(0) } else { (0..dmats.count() - 1).choose(&mut rng) };
        println!("next: {:#?}", next_ind);
        let mut next = dmats.iter().next();
        for (i, val) in dmats.iter().enumerate() {
            if i == next_ind.unwrap() {
                next = Some(val);
            }
        }
        let enab = if dmats.is_empty() { emats.iter().skip(1) } else { emats.iter().skip(0) };
        for (enabled, _) in enab {
            println!("Enabled: {:#?}", enabled);
            commands.entity(enabled).insert(Disabled);
        }
        next.map(|(n, _)| { println!("Disabled: {:#?}", n); commands.entity(n).remove::<Disabled>(); });
        state.time = 0.0;
        /*
        for (_, mat) in material_mandelbrot.iter_mut() {
            mat.time = 0.0;
        }

        for (_, mat) in material_qjulia.iter_mut() {
            mat.time = 0.0;
        }

        for (_, mat) in material_ifs.iter_mut() {
            mat.time = 0.0;
        }

        for (_, mat) in material_mandelbulb.iter_mut() {
            mat.time = 0.0;
        }

        for (_, mat) in material_sponge.iter_mut() {
            mat.time = 0.0;
    }
        */
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct MandelbrotMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
    #[uniform(2)] base_color : Vec3,
    #[uniform(3)] h_factor : f32,
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

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct MandelbulbMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
    #[uniform(2)] base_color : Vec3,
}

impl Material2d for MandelbulbMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbulb_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct MengerSpongeMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
    #[uniform(2)] base_color : Vec3,
    #[uniform(3)] sponge_s : f32,
}

impl Material2d for MengerSpongeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sponge_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct MandelboxMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
    #[uniform(2)] base_color : Vec3,
}

impl Material2d for MandelboxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbox_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Clone, Debug)]
struct IfsMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
}

impl Material2d for IfsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ifs_shader.wgsl".into()
    }
}

#[derive(Resource)]
struct State {
    mu_a : Vec4,
    mu_b : Vec4,
    col_a : Vec3,
    col_b : Vec3,
    s_a : f32,
    s_b : f32,
    h_a : f32,
    h_b : f32,
    base_color : Vec3,
    t : f32,
    time : f32,
    data : String,
}
