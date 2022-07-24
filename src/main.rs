use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct DiceMesh(Handle<Mesh>);
pub struct DiceMaterial(Handle<StandardMaterial>);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(spawn_new_dice)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(
            Mat4::look_at_rh(
                Vec3::new(-30.0, 30.0, 100.0),
                Vec3::new(0.0, 10.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )
            .inverse(),
        ),
        ..Default::default()
    });
}

pub fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height, ground_size));

    // Light

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 10.0),
        ..default()
    });

    // load meshes
    commands.insert_resource(DiceMesh(meshes.add(Mesh::from(shape::Cube { size: 2.0 }))));
    commands.insert_resource(DiceMaterial(
        materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
    ));
}

fn spawn_new_dice(
    mut commands: Commands,
    mut dice_mesh: ResMut<DiceMesh>,
    mut dice_material: ResMut<DiceMaterial>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        /*
         * Create the cubes
         */
        let num = 2;
        let rad = 1.0;

        let centerx = (num / 2) as f32;
        let centery = 0f32;
        let centerz = (num / 2) as f32;

        let mut color = 0;
        let colors = [
            Color::hsl(220.0, 1.0, 0.3),
            Color::hsl(180.0, 1.0, 0.3),
            Color::hsl(260.0, 1.0, 0.7),
        ];

        let x = -centerx;
        let y = centery + 3.0;
        let z = centerz;
        color += 1;

        commands
            .spawn_bundle(TransformBundle::from(Transform::from_rotation(
                Quat::from_rotation_x(0.2),
            )))
            .with_children(|child| {
                child
                    .spawn_bundle(PbrBundle {
                        mesh: dice_mesh.0.clone(),
                        material: dice_material.0.clone(),
                        ..default()
                    })
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(x, y, z)))
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::cuboid(rad, rad, rad))
                    .insert(ColliderDebugColor(colors[color % 3]));
            });
    }
}
