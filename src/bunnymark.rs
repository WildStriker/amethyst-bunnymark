use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::{
        Builder, GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans, World, WorldExt,
    },
    renderer::{camera::Projection, Camera, ImageFormat, Sprite, SpriteSheet, Texture},
    ui::UiCreator,
    winit::{dpi::LogicalSize, Event, WindowEvent},
};

pub const SPRITE_WIDTH: f32 = 26.0;
pub const SPRITE_HEIGHT: f32 = 37.0;

const SPRITE_MIDDLE_X: f32 = 13.0;
const SPRITE_MIDDLE_Y: f32 = 18.5;

#[derive(Default)]
pub struct BunnyMarkState {
    camera: Option<Entity>,
}

impl SimpleState for BunnyMarkState {
    /// Initialization starts once BunnyMarkState has started
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // initial width and height of the window
        // should match display.ron but handle_event may override this right away anyways
        let width = 800.0;
        let height = 600.0;

        let world = data.world;

        // initialize shared resource
        let resource = BunnyMarkResource {
            sprite_sheet: load_sprite_sheet(world),
            top_right: [width, height],
            render_count: 0,
        };

        world.insert(resource);

        // initialize UI from config file
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/ui.ron", ());
        });

        // initialize camera
        let mut transform = Transform::default();
        transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

        self.camera = Some(
            world
                .create_entity()
                .with(Camera::from(config_projection(width, height)))
                .with(transform)
                .build(),
        );
    }

    /// Update camera / resource when game window is resized
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => match event {
                Event::WindowEvent {
                    window_id: _,
                    event: window_event,
                } => match window_event {
                    WindowEvent::Resized(logical_size) => {
                        // get current window size
                        let LogicalSize { width, height } = logical_size;

                        // do nothing if window is to small \ minimized
                        if width <= 0.0 || height <= 0.0 {
                            return Trans::None;
                        }
                        let width = width as f32;
                        let height = height as f32;

                        let mut cameras = data.world.write_storage::<Camera>();
                        let mut transforms = data.world.write_storage::<Transform>();
                        // set new camera projection
                        let camera = cameras.get_mut(self.camera.unwrap()).unwrap();
                        camera.set_projection(config_projection(width, height));

                        // update new top right location
                        let mut resource = data.world.write_resource::<BunnyMarkResource>();

                        resource.top_right[0] = width;
                        resource.top_right[1] = height;
                        // update camera location
                        let transform = transforms.get_mut(self.camera.unwrap()).unwrap();
                        transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        Trans::None
    }
}

/// Initializes bunny sprite sheet
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/wabbit_alpha.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let mut sprites = Vec::with_capacity(1);

    let sprite_width = SPRITE_WIDTH as u32;
    let sprite_height = SPRITE_HEIGHT as u32;

    let sprite = Sprite::from_pixel_values(
        sprite_width,
        sprite_height,
        sprite_width,
        sprite_height,
        0,
        0,
        [SPRITE_MIDDLE_X, SPRITE_MIDDLE_Y],
        false,
        false,
    );

    sprites.push(sprite);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load_from_data(SpriteSheet { texture, sprites }, (), &sprite_sheet_store)
}

/// Bunny Component to hold individual tracked states
pub struct Bunny {
    pub velocity: [f32; 2],
}

impl Component for Bunny {
    type Storage = DenseVecStorage<Self>;
}

/// Shared Resource that does not belong to a single bunny entity
pub struct BunnyMarkResource {
    /// Handle for sprite_sheet containing Bunny Sprite
    pub sprite_sheet: Handle<SpriteSheet>,
    /// Bottom Left is always 0, 0.
    /// This variable keeps track of the current Top Right coordinates location (x, y)
    /// Or in other words top_right will represent the total (width, height)
    pub top_right: [f32; 2],
    /// Total count of Bunny Entities being rendered
    pub render_count: u32,
}

/// Helper function to create a new Projection for camera
///
/// used for initial set up and when window is resized
fn config_projection(width: f32, height: f32) -> Projection {
    Projection::orthographic(
        -width / 2.0,
        width / 2.0,
        -height / 2.0,
        height / 2.0,
        0.1,
        2000.0,
    )
}
