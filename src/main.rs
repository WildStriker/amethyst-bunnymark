use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};

mod bindings;
mod bunnymark;
mod systems;

use bindings::BunnyMarkBindings;
use bunnymark::BunnyMarkState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<BunnyMarkBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(UiBundle::<BunnyMarkBindings>::new())?
        .with(systems::BounceSystem, "bounce_system", &[])
        .with(
            systems::MoveBunniesSystem,
            "move_system",
            &["bounce_system"],
        )
        .with(systems::AddBunniesSystem, "add_bunnies", &[])
        .with(systems::UiUpdateSystem::new(), "ui_update", &[]);

    let mut game = Application::new(assets_dir, BunnyMarkState::default(), game_data)?;
    game.run();

    Ok(())
}
