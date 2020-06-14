use amethyst::{
    core::Time,
    ecs::prelude::{Entity, Read, ReadExpect, System, WriteStorage},
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};

use crate::bunnymark::BunnyMarkResource;

/// This system is responsible for updating UiText
pub struct UiUpdateSystem {
    fps_ui: Option<Entity>,
    bunny_count_ui: Option<Entity>,
}

impl UiUpdateSystem {
    pub fn new() -> UiUpdateSystem {
        UiUpdateSystem {
            fps_ui: None,
            bunny_count_ui: None,
        }
    }
}

impl<'s> System<'s> for UiUpdateSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, FpsCounter>,
        ReadExpect<'s, BunnyMarkResource>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    /// Update FPS and Bunny Counts every 20 frames
    fn run(&mut self, (time, fps_counter, resource, mut ui_text, finder): Self::SystemData) {
        if time.frame_number() % 20 == 0 {
            match self.fps_ui {
                Some(fps_ui) => {
                    if let Some(text) = ui_text.get_mut(fps_ui) {
                        text.text = format!("FPS: {:.2}", fps_counter.sampled_fps());
                    }
                }
                None => {
                    self.fps_ui = finder.find("fps");
                }
            }

            match self.bunny_count_ui {
                Some(bunny_count_ui) => {
                    if let Some(text) = ui_text.get_mut(bunny_count_ui) {
                        text.text = format!("Bunnies: {}", resource.render_count);
                    }
                }
                None => {
                    self.bunny_count_ui = finder.find("bunny_count");
                }
            }
        }
    }
}
