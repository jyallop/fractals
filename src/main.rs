use bevy::prelude::*;
use bevy::sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::render::render_resource::*;
use bevy::math::primitives::Rectangle;
use bevy::shader::ShaderRef;
use bevy::window::Window;
use bevy::window::PrimaryWindow;
use bevy::ecs::entity_disabling::Disabled;
use rand::prelude::*;

#[derive(Component)]
enum Mat {
     Mandel,
     Animate,
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<MandelbrotMaterial>::default())
        .add_plugins(Material2dPlugin::<AnimateMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_time)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut material_man: ResMut<Assets<MandelbrotMaterial>>,
    mut material_an: ResMut<Assets<AnimateMaterial>>,
)
{
    commands.spawn(Camera2d);

    let mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let window = windows.single().unwrap();

    let res = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    let transform = Transform {
        scale: Vec3::new(2000.0, 2000.0, 1.0),
        translation: Vec3::new(0.0, 0.0, 0.0),
        ..default()
    };

    let transform_two = Transform {
        scale: Vec3::new(2000.0, 2000.0, 1.0),
        translation: Vec3::new(0.0, 0.0, 0.0),
        ..default()
    };

    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_man.add(MandelbrotMaterial {
            resolution: res.clone(), // will be set next frame
            time : 0.0,
        })),
        Mat::Mandel,
        transform.clone()
    ));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material_an.add(AnimateMaterial {
            resolution: res, // will be set next frame
            time : 0.0,
        })),
        transform_two,
        Mat::Animate,
    ));

}

fn update_time(
    time: Res<Time>,
    mut commands: Commands,
    dmats: Query<(Entity, &Mat), With<Disabled>>,
    emats: Query<(Entity, &Mat), Without<Disabled>>,
    mut material_man: ResMut<Assets<MandelbrotMaterial>>,
    mut material_an: ResMut<Assets<AnimateMaterial>>,
)
{
    let mut max_time = 0.0;
    for (_, mat) in material_man.iter_mut() {
        mat.time += time.delta_secs();
        max_time = mat.time;
    }

    for (_, mat) in material_an.iter_mut() {
        mat.time += time.delta_secs();
        if mat.time > max_time { max_time = mat.time; }
    }

    if max_time > 10.0 {
        let mut rng = rand::rng();
        let next_ind = (0..dmats.count()).choose(&mut rng);
        let next = dmats.iter().next();

        for (enabled, _) in emats {
            println!("Enabled: {:#?}", enabled);
            commands.entity(enabled).insert(Disabled);
        }
        next.map(|(n, _)| { println!("Disabled: {:#?}", n); commands.entity(n).remove::<Disabled>(); });
        for (_, mat) in material_man.iter_mut() {
            mat.time = 0.0;
        }

        for (_, mat) in material_an.iter_mut() {
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
struct AnimateMaterial {
    #[uniform(0)] resolution : Vec2,
    #[uniform(1)] time : f32,
}

impl Material2d for AnimateMaterial {
    fn fragment_shader() -> ShaderRef {
       "shaders/fract_two_shader.wgsl".into()
    }
}
