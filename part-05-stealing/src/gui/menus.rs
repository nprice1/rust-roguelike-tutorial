use super::ItemMenuResult;
use crate::rltk;
use specs::prelude::*;

const ITEMS_PER_PAGE: usize = 20;
const MENU_X: i32 = 5;
const MENU_WIDTH: i32 = 38;
const MENU_PADDING: i32 = 2;
const HELP_WIDTH: usize = 25;

pub fn menu_box<T: ToString>(
    draw_batch: &mut rltk::DrawBatch,
    y: i32,
    width: i32,
    title: T,
    help_options: Vec<(&str, &str)>,
) {
    draw_batch.draw_box(
        rltk::Rect::with_size(MENU_X, y - MENU_PADDING, MENU_WIDTH, width),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
    );
    draw_batch.print_color(
        rltk::Point::new(MENU_X + MENU_PADDING, y - MENU_PADDING),
        &title.to_string(),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::MAGENTA),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
    help_menu(draw_batch, y + width - MENU_PADDING, help_options);
}

pub fn menu_option<T: ToString>(
    draw_batch: &mut rltk::DrawBatch,
    y: i32,
    hotkey: rltk::FontCharType,
    text: T,
    color: rltk::RGB,
) {
    draw_batch.set(
        rltk::Point::new(MENU_X + MENU_PADDING, y),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
        rltk::to_cp437('('),
    );
    draw_batch.set(
        rltk::Point::new(MENU_X + MENU_PADDING + 1, y),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
        hotkey,
    );
    draw_batch.set(
        rltk::Point::new(MENU_X + MENU_PADDING + 2, y),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
        rltk::to_cp437(')'),
    );
    draw_batch.print_color(
        rltk::Point::new(MENU_X + MENU_PADDING + 5, y),
        &text.to_string(),
        rltk::ColorPair::new(color, rltk::RGB::named(rltk::BLACK)),
    );
}

pub fn help_menu(draw_batch: &mut rltk::DrawBatch, y: i32, extra_options: Vec<(&str, &str)>) {
    print_help_text(draw_batch, y, ("ESC", "Cancel"));
    print_help_text(draw_batch, y + 1, (",", "Previous Page"));
    print_help_text(draw_batch, y + 2, (".", "Next Page"));
    for (j, option) in extra_options.iter().enumerate() {
        print_help_text(draw_batch, j as i32 + y + 3, *option);
    }
    draw_batch.print_color(
        rltk::Point::new(MENU_X + MENU_PADDING, extra_options.len() as i32 + y + 3),
        format!("└{:width$}┘", "", width = (MENU_WIDTH - 5) as usize),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
}

fn print_help_text(draw_batch: &mut rltk::DrawBatch, y: i32, option: (&str, &str)) {
    let command_text = format!("({}):", option.0);
    draw_batch.print_color(
        rltk::Point::new(MENU_X + MENU_PADDING, y),
        format!(
            "├ {:mid_width$}{:right_width$}┤",
            command_text,
            option.1,
            mid_width = 7,
            right_width = HELP_WIDTH
        ),
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
}

pub fn page_list<T>(items: &[T], page: usize) -> &[T] {
    let start_index = std::cmp::min(page * ITEMS_PER_PAGE, items.len() - 1);
    let end_index = std::cmp::min(start_index + ITEMS_PER_PAGE, items.len());
    return &items[start_index..end_index];
}

pub fn item_result_menu<S: ToString>(
    draw_batch: &mut rltk::DrawBatch,
    title: S,
    items: &[(Entity, String, rltk::RGB)],
    key: Option<rltk::VirtualKeyCode>,
    page: usize,
) -> (ItemMenuResult, Option<Entity>) {
    let paged_items = page_list(items, page);
    let count = paged_items.len();

    let mut y = (25 - (count / 2)) as i32;
    menu_box(draw_batch, y, (count + 3) as i32, title, Vec::new());

    let mut item_list: Vec<Entity> = Vec::new();
    let mut item_num = 0;
    for item in paged_items {
        menu_option(
            draw_batch,
            y,
            97 + item_num as rltk::FontCharType,
            &item.1,
            item.2,
        );
        item_list.push(item.0);
        y += 1;
        item_num += 1;
    }

    match key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            rltk::VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            rltk::VirtualKeyCode::Comma => {
                if page > 0 && items.len() > ITEMS_PER_PAGE {
                    (ItemMenuResult::PreviousPage, None)
                } else {
                    (ItemMenuResult::NoResponse, None)
                }
            }
            rltk::VirtualKeyCode::Period => {
                if item_num == ITEMS_PER_PAGE && items.len() > ITEMS_PER_PAGE {
                    (ItemMenuResult::NextPage, None)
                } else {
                    (ItemMenuResult::NoResponse, None)
                }
            }
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
