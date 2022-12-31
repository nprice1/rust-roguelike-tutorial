use crate::rltk;
use crate::State;

use super::menu_box;
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
    AllItems,
}

pub fn show_cheat_mode(gs: &mut State, ctx: &mut rltk::BTerm) -> CheatMenuResult {
    if !gs.game_options.show_cheat_menu {
        return CheatMenuResult::Cancel;
    }
    let mut draw_batch = rltk::DrawBatch::new();
    let count = 6;
    let mut y = (25 - (count / 2)) as i32;
    menu_box(
        &mut draw_batch,
        y,
        (count + 3) as i32,
        "Cheating!",
        Vec::new(),
    );

    menu_option(
        &mut draw_batch,
        y,
        rltk::to_cp437('T'),
        "Teleport to next level",
        rltk::RGB::named(rltk::YELLOW),
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        y,
        rltk::to_cp437('H'),
        "Heal all wounds",
        rltk::RGB::named(rltk::YELLOW),
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        y,
        rltk::to_cp437('R'),
        "Reveal the map",
        rltk::RGB::named(rltk::YELLOW),
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        y,
        rltk::to_cp437('G'),
        "God Mode (No Death)",
        rltk::RGB::named(rltk::YELLOW),
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        y,
        rltk::to_cp437('S'),
        "Learn all spells",
        rltk::RGB::named(rltk::YELLOW),
    );

    y += 1;
    menu_option(
        &mut draw_batch,
        y,
        rltk::to_cp437('I'),
        "Give all items",
        rltk::RGB::named(rltk::YELLOW),
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
            rltk::VirtualKeyCode::I => CheatMenuResult::AllItems,
            rltk::VirtualKeyCode::Escape => CheatMenuResult::Cancel,
            _ => CheatMenuResult::NoResponse,
        },
    }
}
