use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Read, System, SystemData, WriteExpect, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};

use rand::random;

use crate::bindings::{ActionBinding, BunnyMarkBindings};
use crate::bunnymark::{Bunny, BunnyMarkResource};

const BUNNY_RATE: u32 = 100;

const VELOCITY_X: f32 = 600.0;
const VELOCITY_Y: f32 = -600.0;

/// This system is responsible for adding more bunnies
#[derive(SystemDesc)]
pub struct AddBunniesSystem;

impl AddBunniesSystem {
    /// Helper function to create a new bunny entity
    fn add_bunny<'s>(
        &mut self,
        entities: &Entities<'s>,
        bunnies: &mut WriteStorage<'s, Bunny>,
        transforms: &mut WriteStorage<'s, Transform>,
        sprite_renders: &mut WriteStorage<'s, SpriteRender>,
        resource: &WriteExpect<'s, BunnyMarkResource>,
    ) {
        let mut local_transform = Transform::default();

        local_transform.set_translation_xyz(0.0, resource.top_right[1], 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet: resource.sprite_sheet.clone(),
            sprite_number: 0,
        };

        entities
            .build_entity()
            .with(sprite_render, sprite_renders)
            .with(
                Bunny {
                    velocity: [
                        random::<f32>() * VELOCITY_X,
                        (random::<f32>() * VELOCITY_Y) - VELOCITY_Y / 2.0,
                    ],
                },
                bunnies,
            )
            .with(local_transform, transforms)
            .build();
    }
}

impl<'s> System<'s> for AddBunniesSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Bunny>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteExpect<'s, BunnyMarkResource>,
        Read<'s, InputHandler<BunnyMarkBindings>>,
    );

    /// If AddBunnies action is detected, start adding more bunnies.
    fn run(
        &mut self,
        (
            entities,
            mut bunnies,
            mut transforms,
            mut sprite_renders,
            mut resource,
            input,
        ): Self::SystemData,
    ) {
        if input
            .action_is_down(&ActionBinding::AddBunnies)
            .unwrap_or(false)
        {
            for _ in 0..BUNNY_RATE {
                self.add_bunny(
                    &entities,
                    &mut bunnies,
                    &mut transforms,
                    &mut sprite_renders,
                    &resource,
                );
            }

            // update render count to reflect new bunnies
            resource.render_count += BUNNY_RATE;
        }
    }
}
