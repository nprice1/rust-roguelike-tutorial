use crate::rltk;
use crate::State;

use super::menu_option;

#[derive(PartialEq, Copy, Clone)]
pub enum CheatMenuResult {
    NoResponse,
    Cancel,
    TeleportToExit,
    Heal,
    Reveal,
    GodMode,
    LearnSpells,
}

pub fn show_cheat_mode(_gs: &mut State, ctx: &mut rltk::BTerm) -> CheatMenuResult {
    let mut draw_batch = rltk::DrawBatch::new();
    let count = 5;
    let mut y = (25 - (count / 2)) as i32;
    draw_batch.draw_box(
        rltk::Rect::with_size(15, y - 2, 31, (count + 3) as i32),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color(
        rltk::Point::new(18, y - 2),
        "Cheating!",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
    draw_batch.print_color(
        rltk::Point::new(18, y + count as i32 + 1),
        "ESCAPE to cancel",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );

    menu_option(
        &mut draw_batch,
        17,
        y,
        rltk::to_cp437('T'),
        "Teleport to next level",
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        17,
        y,
        rltk::to_cp437('H'),
        "Heal all wounds",
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        17,
        y,
        rltk::to_cp437('R'),
        "Reveal the map",
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        17,
        y,
        rltk::to_cp437('G'),
        "God Mode (No Death)",
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        17,
        y,
        rltk::to_cp437('S'),
        "Learn all spells",
    );

    draw_batch
        .submit(6000)
        .expect("Failed to submit cheat menu draw batch");

    match ctx.key {
        None => CheatMenuResult::NoResponse,
        Some(key) => match key {
            rltk::VirtualKeyCode::T => CheatMenuResult::TeleportToExit,
            rltk::VirtualKeyCode::H => CheatMenuResult::Heal,
            rltk::VirtualKeyCode::R => CheatMenuResult::Reveal,
            rltk::VirtualKeyCode::G => CheatMenuResult::GodMode,
            rltk::VirtualKeyCode::S => CheatMenuResult::LearnSpells,
            rltk::VirtualKeyCode::Escape => CheatMenuResult::Cancel,
            _ => CheatMenuResult::NoResponse,
        },
    }
}
