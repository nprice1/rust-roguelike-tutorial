use crate::rltk;

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult {
    NoSelection,
    QuitToMenu,
}

pub fn game_over(ctx: &mut rltk::BTerm) -> GameOverResult {
    let mut draw_batch = rltk::DrawBatch::new();
    draw_batch.print_color_centered(
        15,
        "Your journey has ended!",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
    draw_batch.print_color_centered(
        17,
        "One day, we'll tell you all about how you did.",
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color_centered(
        18,
        "That day, sadly, is not in this chapter..",
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );

    draw_batch.print_color_centered(
        19,
        &format!(
            "You lived for {} turns.",
            crate::gamelog::get_event_count("Turn")
        ),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color_centered(
        20,
        &format!(
            "You suffered {} points of damage.",
            crate::gamelog::get_event_count("Damage Taken")
        ),
        rltk::ColorPair::new(rltk::RGB::named(rltk::RED), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color_centered(
        21,
        &format!(
            "You inflicted {} points of damage.",
            crate::gamelog::get_event_count("Damage Inflicted")
        ),
        rltk::ColorPair::new(rltk::RGB::named(rltk::RED), rltk::RGB::named(rltk::BLACK)),
    );

    draw_batch.print_color_centered(
        23,
        "Press any key to return to the menu.",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::MAGENTA),
            rltk::RGB::named(rltk::BLACK),
        ),
    );

    draw_batch.submit(6000).expect("Failed to submit");

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
