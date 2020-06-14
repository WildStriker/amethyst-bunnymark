use amethyst::{
    core::{Time, Transform},
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

use rand::random;

use crate::bunnymark::{Bunny, BunnyMarkResource, SPRITE_HEIGHT, SPRITE_WIDTH};

pub struct BounceSystem;

const GRAVITY: f32 = 750.0;

/// This system is responsible for setting the velocity state
impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Bunny>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, BunnyMarkResource>,
        Read<'s, Time>,
    );

    /// set the velocity state
    ///
    /// * for `x` coordinate simply flip once edge is reached
    /// * for `y`:
    ///     * once max height is reach the "roof" is hit and velocity is lossed.
    ///     * once min height is reached:
    ///         * continue with reduced momentum
    ///         * there is a chance that the bunny can regain additional momentum
    fn run(&mut self, (mut bunnies, transforms, resource, time): Self::SystemData) {
        for (bunny, transform) in (&mut bunnies, &transforms).join() {
            let bunny_x = transform.translation().x;
            let bunny_y = transform.translation().y;

            // apply gravity
            bunny.velocity[1] -= GRAVITY * time.delta_seconds();

            if bunny_y <= SPRITE_HEIGHT {
                // flip and reduce falling momentum
                bunny.velocity[1] *= -0.85;
                // possibly gain more momentum
                if random::<f32>() > 0.5 {
                    bunny.velocity[1] += random::<f32>() * 360.0;
                }
            } else if bunny_y >= resource.top_right[1] {
                // "roof" is hit, reset velocity by applying only gravity
                bunny.velocity[1] = -GRAVITY * time.delta_seconds();
            }

            // edge reach flip direction
            if bunny_x <= SPRITE_WIDTH || bunny_x >= resource.top_right[0] {
                bunny.velocity[0] *= -1.0;
            }
        }
    }
}
