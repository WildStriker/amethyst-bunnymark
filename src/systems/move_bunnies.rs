use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

use crate::bunnymark::{Bunny, BunnyMarkResource, SPRITE_HEIGHT, SPRITE_WIDTH};

/// This System is responsible for moving all bunnies
#[derive(SystemDesc)]
pub struct MoveBunniesSystem;

impl<'s> System<'s> for MoveBunniesSystem {
    type SystemData = (
        ReadStorage<'s, Bunny>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, BunnyMarkResource>,
        Read<'s, Time>,
    );

    /// Move bunnies based on their current velocity
    ///
    /// Set a min and max range in case bunny's velocity pushes them outside the visible area
    fn run(&mut self, (bunnies, mut transforms, resource, time): Self::SystemData) {
        for (bunny, transform) in (&bunnies, &mut transforms).join() {
            let translation = *transform.translation();

            transform.set_translation_x(
                (translation.x + bunny.velocity[0] * time.delta_seconds())
                    .min(resource.top_right[0])
                    .max(SPRITE_WIDTH),
            );

            transform.set_translation_y(
                (translation.y + bunny.velocity[1] * time.delta_seconds())
                    .min(resource.top_right[1])
                    .max(SPRITE_HEIGHT),
            );
        }
    }
}
