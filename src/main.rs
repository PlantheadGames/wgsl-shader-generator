use bevy::{
    asset::RenderAssetUsages,
    camera::RenderTarget,
    prelude::*,
    render::render_resource::{TextureDimension, TextureFormat, TextureUsages},
    ui::{RelativeCursorPosition, widget::ViewportNode},
};
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, cursor_show_position)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a UI camera
    commands.spawn(Camera3d::default());

    // Set up an texture for the 3D camera to render to.
    // The size of the texture will be based on the viewport's ui size.
    let mut image = Image::new_uninit(
        default(),
        TextureDimension::D2,
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    // Spawn the 3D camera
    let camera = commands
        .spawn((
            Camera3d::default(),
            Camera {
                // Render this camera before our UI camera
                order: -1,
                ..default()
            },
            RenderTarget::Image(image_handle.clone().into()),
        ))
        .id();

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(50),
                left: px(50),
                width: px(200),
                height: px(200),
                border: UiRect::all(px(5)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            ViewportNode::new(camera),
        ))
        .observe(on_drag_viewport);
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(50),
                left: px(50),
                width: px(200),
                height: px(200),
                border: UiRect::all(px(5)),
                ..default()
            },
            BackgroundColor(LinearRgba::BLUE.into()),
            RelativeCursorPosition::default(),
            BorderColor::all(Color::BLACK),
        ))
        .observe(on_drag_viewport);
}

fn on_drag_viewport(drag: On<Pointer<Drag>>, mut node_query: Query<&mut Node>) {
    if matches!(drag.button, PointerButton::Secondary) {
        let mut node = node_query.get_mut(drag.entity).unwrap();

        if let (Val::Px(top), Val::Px(left)) = (node.top, node.left) {
            node.left = px(left + drag.delta.x);
            node.top = px(top + drag.delta.y);
        };
    }
}

fn cursor_show_position(position: Query<&RelativeCursorPosition>) {
    for position in position {
        if position.normalized.is_some() {
            println!("position: {:#?}", position.normalized);
        }
    }
}
