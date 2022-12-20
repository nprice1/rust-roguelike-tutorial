use super::{get_item_color, get_item_display_name, menu_box};
use crate::rltk;
use crate::{InBackpack, Item, Name, State, Vendor, VendorMode};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum VendorResult {
    NoResponse,
    Cancel,
    Sell,
    BuyMode,
    SellMode,
    Buy,
}

fn vendor_sell_menu(
    gs: &mut State,
    ctx: &mut rltk::BTerm,
    _vendor: Entity,
    _mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    let mut draw_batch = rltk::DrawBatch::new();
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let items = gs.ecs.read_storage::<Item>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    menu_box(
        &mut draw_batch,
        15,
        y,
        (count + 3) as i32,
        "Sell Which Item? (space to switch to buy mode)",
    );
    draw_batch.print_color(
        rltk::Point::new(18, y + count as i32 + 1),
        "ESCAPE to cancel",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, item) in (&entities, &backpack, &items)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        draw_batch.set(
            rltk::Point::new(17, y),
            rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
            rltk::to_cp437('('),
        );
        draw_batch.set(
            rltk::Point::new(18, y),
            rltk::ColorPair::new(
                rltk::RGB::named(rltk::YELLOW),
                rltk::RGB::named(rltk::BLACK),
            ),
            97 + j as rltk::FontCharType,
        );
        draw_batch.set(
            rltk::Point::new(19, y),
            rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
            rltk::to_cp437(')'),
        );

        draw_batch.print_color(
            rltk::Point::new(21, y),
            &get_item_display_name(&gs.ecs, entity),
            rltk::ColorPair::new(
                get_item_color(&gs.ecs, entity),
                rltk::RGB::from_f32(0.0, 0.0, 0.0),
            ),
        );
        draw_batch.print(
            rltk::Point::new(50, y),
            &format!("{:.1} gp", item.base_value * 0.8),
        );
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    draw_batch.submit(6000).expect("Failed to submit");

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            rltk::VirtualKeyCode::Space => (VendorResult::BuyMode, None, None, None),
            rltk::VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Sell,
                        Some(equippable[selection as usize]),
                        None,
                        None,
                    );
                }
                (VendorResult::NoResponse, None, None, None)
            }
        },
    }
}

fn vendor_buy_menu(
    gs: &mut State,
    ctx: &mut rltk::BTerm,
    vendor: Entity,
    _mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    use crate::raws::*;
    let mut draw_batch = rltk::DrawBatch::new();

    let vendors = gs.ecs.read_storage::<Vendor>();

    let inventory = crate::raws::get_vendor_items(
        &vendors.get(vendor).unwrap().categories,
        &RAWS.lock().unwrap(),
    );
    let count = inventory.len();

    let mut y = (25 - (count / 2)) as i32;
    menu_box(
        &mut draw_batch,
        15,
        y,
        (count + 3) as i32,
        "Buy Which Item? (space to switch to sell mode)",
    );
    draw_batch.print_color(
        rltk::Point::new(18, y + count as i32 + 1),
        "ESCAPE to cancel",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );

    for (j, sale) in inventory.iter().enumerate() {
        draw_batch.set(
            rltk::Point::new(17, y),
            rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
            rltk::to_cp437('('),
        );
        draw_batch.set(
            rltk::Point::new(18, y),
            rltk::ColorPair::new(
                rltk::RGB::named(rltk::YELLOW),
                rltk::RGB::named(rltk::BLACK),
            ),
            97 + j as rltk::FontCharType,
        );
        draw_batch.set(
            rltk::Point::new(19, y),
            rltk::ColorPair::new(rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK)),
            rltk::to_cp437(')'),
        );

        draw_batch.print(rltk::Point::new(21, y), &sale.0);
        draw_batch.print(rltk::Point::new(50, y), &format!("{:.1} gp", sale.1 * 1.2));
        y += 1;
    }

    draw_batch.submit(6000).expect("Failed to submit");

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            rltk::VirtualKeyCode::Space => (VendorResult::SellMode, None, None, None),
            rltk::VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Buy,
                        None,
                        Some(inventory[selection as usize].0.clone()),
                        Some(inventory[selection as usize].1),
                    );
                }
                (VendorResult::NoResponse, None, None, None)
            }
        },
    }
}

pub fn show_vendor_menu(
    gs: &mut State,
    ctx: &mut rltk::BTerm,
    vendor: Entity,
    mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    match mode {
        VendorMode::Buy => vendor_buy_menu(gs, ctx, vendor, mode),
        VendorMode::Sell => vendor_sell_menu(gs, ctx, vendor, mode),
    }
}
