use super::{get_item_color, get_item_display_name, menu_box};
use crate::gui::{menu_option, page_list};
use crate::rltk;
use crate::{InBackpack, Item, State, Vendor, VendorMode};
use specs::prelude::*;

const ITEMS_PER_PAGE: usize = 20;
const PRICE_X: i32 = 34;

#[derive(PartialEq, Copy, Clone)]
pub enum VendorResult {
    NoResponse,
    Cancel,
    Sell,
    BuyMode,
    SellMode,
    Buy,
    NextPage,
    PreviousPage,
}

fn vendor_sell_menu(
    gs: &mut State,
    ctx: &mut rltk::BTerm,
    _vendor: Entity,
    page: usize,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    let mut draw_batch = rltk::DrawBatch::new();
    let player_entity = gs.ecs.fetch::<Entity>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let items = gs.ecs.read_storage::<Item>();
    let entities = gs.ecs.entities();

    let mut inventory: Vec<(Entity, Item)> = Vec::new();
    (&entities, &backpack, &items)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .for_each(|item| inventory.push((item.0, item.2.clone())));
    let paged_inventory = page_list(&inventory, page);
    let count = paged_inventory.len();

    let mut y = (25 - (count / 2)) as i32;
    menu_box(
        &mut draw_batch,
        y,
        (count + 3) as i32,
        "Sell Which Item?",
        vec![("SPC", "Buy Menu")],
    );

    let mut equippable: Vec<Entity> = Vec::new();
    for (j, item) in paged_inventory.iter().enumerate() {
        menu_option(
            &mut draw_batch,
            y,
            97 + j as rltk::FontCharType,
            get_item_display_name(&gs.ecs, item.0),
            get_item_color(&gs.ecs, item.0),
        );
        draw_batch.print(
            rltk::Point::new(PRICE_X, y),
            &format!("{:.1} gp", item.1.base_value * 0.8),
        );
        equippable.push(item.0);
        y += 1;
    }

    draw_batch.submit(6000).expect("Failed to submit");

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            rltk::VirtualKeyCode::Space => (VendorResult::BuyMode, None, None, None),
            rltk::VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            rltk::VirtualKeyCode::Comma => {
                if page > 0 && inventory.len() > ITEMS_PER_PAGE {
                    (VendorResult::PreviousPage, None, None, None)
                } else {
                    (VendorResult::NoResponse, None, None, None)
                }
            }
            rltk::VirtualKeyCode::Period => {
                if count == ITEMS_PER_PAGE && inventory.len() > ITEMS_PER_PAGE {
                    (VendorResult::NextPage, None, None, None)
                } else {
                    (VendorResult::NoResponse, None, None, None)
                }
            }
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
    page: usize,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    use crate::raws::*;
    let mut draw_batch = rltk::DrawBatch::new();

    let vendors = gs.ecs.read_storage::<Vendor>();

    let inventory = crate::raws::get_vendor_items(
        &vendors.get(vendor).unwrap().categories,
        &RAWS.lock().unwrap(),
    );
    let paged_inventory = page_list(&inventory, page);
    let count = paged_inventory.len();

    let mut y = (25 - (count / 2)) as i32;
    menu_box(
        &mut draw_batch,
        y,
        (count + 3) as i32,
        "Buy Which Item?",
        vec![("SPC", "Sell Menu")],
    );

    for (j, sale) in paged_inventory.iter().enumerate() {
        menu_option(
            &mut draw_batch,
            y,
            97 + j as rltk::FontCharType,
            &sale.0,
            rltk::RGB::named(rltk::WHITE),
        );

        draw_batch.print(
            rltk::Point::new(PRICE_X, y),
            &format!("{:.1} gp", sale.1 * 1.2),
        );
        y += 1;
    }

    draw_batch.submit(6000).expect("Failed to submit");

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            rltk::VirtualKeyCode::Space => (VendorResult::SellMode, None, None, None),
            rltk::VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            rltk::VirtualKeyCode::Comma => {
                if page > 0 && inventory.len() > paged_inventory.len() {
                    (VendorResult::PreviousPage, None, None, None)
                } else {
                    (VendorResult::NoResponse, None, None, None)
                }
            }
            rltk::VirtualKeyCode::Period => {
                if paged_inventory.len() == ITEMS_PER_PAGE && inventory.len() > ITEMS_PER_PAGE {
                    (VendorResult::NextPage, None, None, None)
                } else {
                    (VendorResult::NoResponse, None, None, None)
                }
            }
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Buy,
                        None,
                        Some(paged_inventory[selection as usize].0.clone()),
                        Some(paged_inventory[selection as usize].1),
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
    page: usize,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    match mode {
        VendorMode::Buy => vendor_buy_menu(gs, ctx, vendor, page),
        VendorMode::Sell => vendor_sell_menu(gs, ctx, vendor, page),
    }
}
