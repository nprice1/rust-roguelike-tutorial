use crate::rltk;
use crate::State;

#[derive(PartialEq, Copy, Clone)]
pub enum CheatMenuResult {
    NoResponse,
    Cancel,
    TeleportToExit,
    Heal,
    Reveal,
    GodMode,
}

pub fn show_cheat_mode(_gs: &mut State, ctx: &mut rltk::BTerm) -> CheatMenuResult {
    let count = 4;
    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        rltk::RGB::named(rltk::YELLOW),
        rltk::RGB::named(rltk::BLACK),
        "Cheating!",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        rltk::RGB::named(rltk::YELLOW),
        rltk::RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    ctx.set(
        17,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('('),
    );
    ctx.set(
        18,
        y,
        rltk::RGB::named(rltk::YELLOW),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('T'),
    );
    ctx.set(
        19,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437(')'),
    );
    ctx.print(21, y, "Teleport to next level");

    y += 1;
    ctx.set(
        17,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('('),
    );
    ctx.set(
        18,
        y,
        rltk::RGB::named(rltk::YELLOW),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('H'),
    );
    ctx.set(
        19,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437(')'),
    );
    ctx.print(21, y, "Heal all wounds");

    y += 1;
    ctx.set(
        17,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('('),
    );
    ctx.set(
        18,
        y,
        rltk::RGB::named(rltk::YELLOW),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('R'),
    );
    ctx.set(
        19,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437(')'),
    );
    ctx.print(21, y, "Reveal the map");

    y += 1;
    ctx.set(
        17,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('('),
    );
    ctx.set(
        18,
        y,
        rltk::RGB::named(rltk::YELLOW),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437('G'),
    );
    ctx.set(
        19,
        y,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        rltk::to_cp437(')'),
    );
    ctx.print(21, y, "God Mode (No Death)");

    match ctx.key {
        None => CheatMenuResult::NoResponse,
        Some(key) => match key {
            rltk::VirtualKeyCode::T => CheatMenuResult::TeleportToExit,
            rltk::VirtualKeyCode::H => CheatMenuResult::Heal,
            rltk::VirtualKeyCode::R => CheatMenuResult::Reveal,
            rltk::VirtualKeyCode::G => CheatMenuResult::GodMode,
            rltk::VirtualKeyCode::Escape => CheatMenuResult::Cancel,
            _ => CheatMenuResult::NoResponse,
        },
    }
}
