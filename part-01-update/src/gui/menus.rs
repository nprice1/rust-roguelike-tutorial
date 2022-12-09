use super::ItemMenuResult;
use crate::rltk;
use specs::prelude::*;

pub fn menu_box<T: ToString>(
    draw_batch: &mut rltk::DrawBatch,
    x: i32,
    y: i32,
    width: i32,
    title: T,
) {
    draw_batch.draw_box(
        rltk::Rect::with_size(x, y - 2, 31, width),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color(
        rltk::Point::new(18, y - 2),
        &title.to_string(),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::MAGENTA),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
}

pub fn menu_option<T: ToString>(
    draw_batch: &mut rltk::DrawBatch,
    x: i32,
    y: i32,
    hotkey: rltk::FontCharType,
    text: T,
) {
    draw_batch.set(
        rltk::Point::new(x, y),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
        rltk::to_cp437('('),
    );
    draw_batch.set(
        rltk::Point::new(x + 1, y),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
        hotkey,
    );
    draw_batch.set(
        rltk::Point::new(x + 2, y),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
        rltk::to_cp437(')'),
    );
    draw_batch.print_color(
        rltk::Point::new(x + 5, y),
        &text.to_string(),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
}

pub fn item_result_menu<S: ToString>(
    draw_batch: &mut rltk::DrawBatch,
    title: S,
    count: usize,
    items: &[(Entity, String)],
    key: Option<rltk::VirtualKeyCode>,
) -> (ItemMenuResult, Option<Entity>) {
    let mut y = (25 - (count / 2)) as i32;
    draw_batch.draw_box(
        rltk::Rect::with_size(15, y - 2, 31, (count + 3) as i32),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color(
        rltk::Point::new(18, y - 2),
        &title.to_string(),
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

    let mut item_list: Vec<Entity> = Vec::new();
    let mut j = 0;
    for item in items {
        menu_option(draw_batch, 17, y, 97 + j as rltk::FontCharType, &item.1);
        item_list.push(item.0);
        y += 1;
        j += 1;
    }

    match key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            rltk::VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(item_list[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}
