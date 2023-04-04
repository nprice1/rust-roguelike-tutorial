extern crate bracket_lib;
extern crate serde;

pub use bracket_lib::prelude as rltk;

use rltk::{BTerm, GameState, Point};
use rodio::OutputStream;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod damage_system;
mod gamelog;
mod gamesystem;
mod gui;
pub mod map_builders;
pub mod random_table;
pub mod raws;
pub mod rex_assets;
pub mod saveload_system;
mod spawner;
pub use gamesystem::*;

use crate::systems::sound_system::SoundSystem;

pub mod effects;
#[macro_use]
extern crate lazy_static;
pub mod rng;
pub mod spatial;
mod systems;

macro_rules! register {
    (
        $ecs:expr,
        $(
            $type:ty
        ),*
    ) => {
        $(
            $ecs.register::<$type>();
        )*
    };
}

#[derive(PartialEq, Copy, Clone)]
pub enum VendorMode {
    Buy,
    Sell,
    Steal,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    Ticking,
    ShowInventory {
        page: usize,
    },
    ShowDropItem {
        page: usize,
    },
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    NextLevel,
    PreviousLevel,
    TownPortal,
    ShowRemoveItem {
        page: usize,
    },
    GameOver,
    MagicMapReveal {
        row: i32,
    },
    MapGeneration,
    ShowCheatMenu,
    ShowVendor {
        vendor: Entity,
        mode: VendorMode,
        page: usize,
    },
    TeleportingToOtherLevel {
        x: i32,
        y: i32,
        depth: i32,
    },
    ShowRemoveCurse {
        page: usize,
    },
    ShowIdentify {
        page: usize,
    },
    OptionsMenu {
        menu_selection: gui::OptionsMenuSelection,
    },
}

struct GameOptions {
    show_fps: bool,
    show_map_visualizer: bool,
    show_cheat_menu: bool,
}

pub struct State {
    pub ecs: World,
    mapgen_next_state: Option<RunState>,
    mapgen_history: Vec<Map>,
    mapgen_index: usize,
    mapgen_timer: f32,
    dispatcher: Box<dyn systems::UnifiedDispatcher + 'static>,
    game_options: GameOptions,
}

impl State {
    fn run_systems(&mut self) {
        self.dispatcher.run_now(&mut self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    #[allow(clippy::cognitive_complexity)]
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);
        ctx.cls();
        systems::particle_system::update_particles(&mut self.ecs, ctx);

        match newrunstate {
            RunState::MainMenu { .. } => {}
            RunState::GameOver { .. } => {}
            RunState::OptionsMenu { .. } => {}
            _ => {
                camera::render_camera(&self.ecs, ctx);
                gui::draw_ui(&self.ecs, ctx);
            }
        }

        match newrunstate {
            RunState::MapGeneration => {
                if !self.game_options.show_map_visualizer {
                    newrunstate = self.mapgen_next_state.unwrap();
                } else {
                    ctx.cls();
                    if self.mapgen_index < self.mapgen_history.len()
                        && self.mapgen_index < self.mapgen_history.len()
                    {
                        camera::render_debug_map(&self.mapgen_history[self.mapgen_index], ctx);
                    }

                    self.mapgen_timer += ctx.frame_time_ms;
                    if self.mapgen_timer > 250.0 {
                        self.mapgen_timer = 0.0;
                        self.mapgen_index += 1;
                        if self.mapgen_index >= self.mapgen_history.len() {
                            //self.mapgen_index -= 1;
                            newrunstate = self.mapgen_next_state.unwrap();
                        }
                    }
                }
            }
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
                if newrunstate != RunState::AwaitingInput {
                    crate::gamelog::record_event("Turn", 1);
                }
            }
            RunState::Ticking => {
                let mut should_change_target = false;
                while newrunstate == RunState::Ticking {
                    self.run_systems();
                    self.ecs.maintain();
                    match *self.ecs.fetch::<RunState>() {
                        RunState::AwaitingInput => {
                            newrunstate = RunState::AwaitingInput;
                            should_change_target = true;
                        }
                        RunState::MagicMapReveal { .. } => {
                            newrunstate = RunState::MagicMapReveal { row: 0 }
                        }
                        RunState::TownPortal => newrunstate = RunState::TownPortal,
                        RunState::TeleportingToOtherLevel { x, y, depth } => {
                            newrunstate = RunState::TeleportingToOtherLevel { x, y, depth }
                        }
                        RunState::ShowRemoveCurse { page } => {
                            newrunstate = RunState::ShowRemoveCurse { page }
                        }
                        RunState::ShowIdentify { page } => {
                            newrunstate = RunState::ShowIdentify { page }
                        }
                        _ => newrunstate = RunState::Ticking,
                    }
                }
                if should_change_target {
                    player::end_turn_targeting(&mut self.ecs);
                }
            }
            RunState::ShowInventory { page } => {
                let result = gui::show_inventory(self, ctx, page);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::Ticking;
                        }
                    }
                    gui::ItemMenuResult::NextPage => {
                        newrunstate = RunState::ShowInventory { page: page + 1 }
                    }
                    gui::ItemMenuResult::PreviousPage => {
                        newrunstate = RunState::ShowInventory { page: page - 1 }
                    }
                }
            }
            RunState::ShowCheatMenu => {
                let result = gui::show_cheat_mode(self, ctx);
                match result {
                    gui::CheatMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::CheatMenuResult::NoResponse => {}
                    gui::CheatMenuResult::TeleportToExit => {
                        self.goto_level(1);
                        self.mapgen_next_state = Some(RunState::PreRun);
                        newrunstate = RunState::MapGeneration;
                    }
                    gui::CheatMenuResult::Heal => {
                        let player = self.ecs.fetch::<Entity>();
                        let mut pools = self.ecs.write_storage::<Pools>();
                        let mut player_pools = pools.get_mut(*player).unwrap();
                        player_pools.hit_points.current = player_pools.hit_points.max;
                        newrunstate = RunState::AwaitingInput;
                    }
                    gui::CheatMenuResult::Reveal => {
                        let mut map = self.ecs.fetch_mut::<Map>();
                        for v in map.revealed_tiles.iter_mut() {
                            *v = true;
                        }
                        newrunstate = RunState::AwaitingInput;
                    }
                    gui::CheatMenuResult::GodMode => {
                        let player = self.ecs.fetch::<Entity>();
                        let mut pools = self.ecs.write_storage::<Pools>();
                        let mut player_pools = pools.get_mut(*player).unwrap();
                        player_pools.god_mode = true;
                        newrunstate = RunState::AwaitingInput;
                    }
                    gui::CheatMenuResult::LearnSpells => {
                        let player = self.ecs.fetch::<Entity>();
                        let spells = self.ecs.read_storage::<SpellTemplate>();
                        let names = self.ecs.read_storage::<Name>();
                        let mut known_spells = self.ecs.write_storage::<KnownSpells>();
                        let entities = self.ecs.entities();

                        let mut updated_spells = Vec::new();
                        for (_entity, name, template) in (&entities, &names, &spells).join() {
                            updated_spells.push(KnownSpell {
                                display_name: name.name.clone(),
                                mana_cost: template.mana_cost,
                            });
                        }
                        known_spells
                            .insert(
                                *player,
                                KnownSpells {
                                    spells: updated_spells,
                                },
                            )
                            .expect("Unable to insert spells");

                        newrunstate = RunState::AwaitingInput;
                    }
                    gui::CheatMenuResult::AllItems => {
                        let player_entity = *self.ecs.fetch::<Entity>();
                        raws::give_all_items(&mut self.ecs, player_entity);

                        newrunstate = RunState::AwaitingInput;
                    }
                }
            }
            RunState::ShowDropItem { page } => {
                let result = gui::drop_item_menu(self, ctx, page);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToDropItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::Ticking;
                    }
                    gui::ItemMenuResult::NextPage => {
                        newrunstate = RunState::ShowDropItem { page: page + 1 }
                    }
                    gui::ItemMenuResult::PreviousPage => {
                        newrunstate = RunState::ShowDropItem { page: page - 1 }
                    }
                }
            }
            RunState::ShowRemoveItem { page } => {
                let result = gui::remove_item_menu(self, ctx, page);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToRemoveItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::Ticking;
                    }
                    gui::ItemMenuResult::NextPage => {
                        newrunstate = RunState::ShowRemoveItem { page: page + 1 }
                    }
                    gui::ItemMenuResult::PreviousPage => {
                        newrunstate = RunState::ShowRemoveItem { page: page - 1 }
                    }
                }
            }
            RunState::ShowRemoveCurse { page } => {
                let result = gui::remove_curse_menu(self, ctx, page);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        self.ecs.write_storage::<CursedItem>().remove(item_entity);
                        newrunstate = RunState::Ticking;
                    }
                    gui::ItemMenuResult::NextPage => {
                        newrunstate = RunState::ShowRemoveCurse { page: page + 1 }
                    }
                    gui::ItemMenuResult::PreviousPage => {
                        newrunstate = RunState::ShowRemoveCurse { page: page - 1 }
                    }
                }
            }
            RunState::ShowIdentify { page } => {
                let result = gui::identify_menu(self, ctx, page);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        if let Some(name) = self.ecs.read_storage::<Name>().get(item_entity) {
                            let mut dm = self.ecs.fetch_mut::<MasterDungeonMap>();
                            dm.identified_items.insert(name.name.clone());
                        }
                        newrunstate = RunState::Ticking;
                    }
                    gui::ItemMenuResult::NextPage => {
                        newrunstate = RunState::ShowIdentify { page: page + 1 }
                    }
                    gui::ItemMenuResult::PreviousPage => {
                        newrunstate = RunState::ShowIdentify { page: page - 1 }
                    }
                }
            }
            RunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::NextPage => {}
                    gui::ItemMenuResult::PreviousPage => {}
                    gui::ItemMenuResult::Selected => {
                        if self.ecs.read_storage::<SpellTemplate>().get(item).is_some() {
                            let mut intent = self.ecs.write_storage::<WantsToCastSpell>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToCastSpell {
                                        spell: item,
                                        target: result.1,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::Ticking;
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item,
                                        target: result.1,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::Ticking;
                        }
                    }
                }
            }
            RunState::ShowVendor { vendor, mode, page } => {
                use crate::raws::*;
                let result = gui::show_vendor_menu(self, ctx, vendor, mode, page);
                match result.0 {
                    gui::VendorResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::VendorResult::NoResponse => {}
                    gui::VendorResult::Sell => {
                        let price = self
                            .ecs
                            .read_storage::<Item>()
                            .get(result.1.unwrap())
                            .unwrap()
                            .base_value
                            * 0.8;
                        self.ecs
                            .write_storage::<Pools>()
                            .get_mut(*self.ecs.fetch::<Entity>())
                            .unwrap()
                            .gold += price;
                        self.ecs
                            .delete_entity(result.1.unwrap())
                            .expect("Unable to delete");
                        self.ecs
                            .read_resource::<SoundSystem>()
                            .play_sound_effects(vec![String::from("buy_sell.wav")]);
                    }
                    gui::VendorResult::Buy => {
                        let tag = result.2.unwrap();
                        let price = result.3.unwrap();
                        let mut pools = self.ecs.write_storage::<Pools>();
                        let player_entity = self.ecs.fetch::<Entity>();
                        let mut identified = self.ecs.write_storage::<IdentifiedItem>();
                        identified
                            .insert(*player_entity, IdentifiedItem { name: tag.clone() })
                            .expect("Unable to insert");
                        std::mem::drop(identified);
                        let player_pools = pools.get_mut(*player_entity).unwrap();
                        std::mem::drop(player_entity);
                        if player_pools.gold >= price {
                            player_pools.gold -= price;
                            std::mem::drop(pools);
                            let player_entity = *self.ecs.fetch::<Entity>();
                            crate::raws::spawn_named_item(
                                &RAWS.lock().unwrap(),
                                &mut self.ecs,
                                &tag,
                                SpawnType::Carried { by: player_entity },
                            );
                            self.ecs
                                .fetch::<SoundSystem>()
                                .play_sound_effects(vec![String::from("buy_sell.wav")]);
                        }
                    }
                    gui::VendorResult::Steal => {
                        let tag = result.2.unwrap();
                        let price = result.3.unwrap();
                        let player_entity = self.ecs.fetch::<Entity>();
                        let attributes = self.ecs.read_storage::<Attributes>();
                        let player_attributes = attributes.get(*player_entity).unwrap();
                        // Calculate the value needed for success
                        let target_value = match price {
                            i if i < 50.0 => 10,
                            i if i < 100.0 => 15,
                            i if i < 300.0 => 20,
                            _ => 25,
                        };
                        let natural_roll = crate::rng::roll_dice(1, 20);
                        let quickness_bonus = player_attributes.quickness.bonus;
                        // No matter what happens, you get the item
                        let mut identified = self.ecs.write_storage::<IdentifiedItem>();
                        identified
                            .insert(*player_entity, IdentifiedItem { name: tag.clone() })
                            .expect("Unable to insert");
                        std::mem::drop(identified);
                        std::mem::drop(player_entity);
                        std::mem::drop(attributes);
                        let player_entity = *self.ecs.fetch::<Entity>();
                        crate::raws::spawn_named_item(
                            &RAWS.lock().unwrap(),
                            &mut self.ecs,
                            &tag,
                            SpawnType::Carried { by: player_entity },
                        );
                        if natural_roll + quickness_bonus > target_value {
                            // Successful theft
                            self.ecs
                                .fetch::<SoundSystem>()
                                .play_sound_effects(vec![String::from("steal.wav")]);
                        } else {
                            // Failed to steal
                            let mut factions = self.ecs.write_storage::<Faction>();
                            factions
                                .insert(
                                    player_entity,
                                    Faction {
                                        name: "Thief".to_string(),
                                    },
                                )
                                .expect("Unable to insert");
                            self.ecs
                                .fetch::<SoundSystem>()
                                .play_sound_effects(vec![String::from("failure.wav")]);
                            // Exit the vendor menu
                            newrunstate = RunState::AwaitingInput
                        }
                    }
                    gui::VendorResult::BuyMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Buy,
                            page: 0,
                        }
                    }
                    gui::VendorResult::SellMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Sell,
                            page: 0,
                        }
                    }
                    gui::VendorResult::StealMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Steal,
                            page: 0,
                        }
                    }
                    gui::VendorResult::PreviousPage => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: mode,
                            page: page - 1,
                        }
                    }
                    gui::VendorResult::NextPage => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: mode,
                            page: page + 1,
                        }
                    }
                }
            }
            RunState::MainMenu { .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::Options => {
                            newrunstate = RunState::OptionsMenu {
                                menu_selection: gui::OptionsMenuSelection::ToggleFps,
                            }
                        }
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::AwaitingInput;
                            saveload_system::delete_save();
                        }
                        gui::MainMenuSelection::SaveGame => {
                            saveload_system::save_game(&mut self.ecs);
                            newrunstate = RunState::MainMenu {
                                menu_selection: gui::MainMenuSelection::Quit,
                            };
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::OptionsMenu { .. } => {
                let result = gui::options_menu(self, ctx);
                match result {
                    gui::OptionsMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::OptionsMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::OptionsMenuResult::Selected { selected } => match selected {
                        gui::OptionsMenuSelection::ToggleFps => {
                            self.game_options.show_fps = !self.game_options.show_fps;
                            newrunstate = RunState::OptionsMenu {
                                menu_selection: selected,
                            }
                        }
                        gui::OptionsMenuSelection::ToggleMapVisualizer => {
                            self.game_options.show_map_visualizer =
                                !self.game_options.show_map_visualizer;
                            newrunstate = RunState::OptionsMenu {
                                menu_selection: selected,
                            }
                        }
                        gui::OptionsMenuSelection::ToggleCheatMenu => {
                            self.game_options.show_cheat_menu = !self.game_options.show_cheat_menu;
                            newrunstate = RunState::OptionsMenu {
                                menu_selection: selected,
                            }
                        }
                        gui::OptionsMenuSelection::BackgroundVolume { change } => {
                            let volume_change;
                            match change {
                                gui::VolumeChange::None => {
                                    volume_change = 0.0;
                                }
                                gui::VolumeChange::Increase => {
                                    volume_change = 1.0;
                                }
                                gui::VolumeChange::Decrease => {
                                    volume_change = -1.0;
                                }
                            }
                            self.ecs
                                .fetch::<SoundSystem>()
                                .change_background_volume(volume_change);
                            newrunstate = RunState::OptionsMenu {
                                menu_selection: gui::OptionsMenuSelection::BackgroundVolume {
                                    change: gui::VolumeChange::None,
                                },
                            }
                        }
                        gui::OptionsMenuSelection::EffectsVolume { change } => {
                            let volume_change;
                            match change {
                                gui::VolumeChange::None => {
                                    volume_change = 0.0;
                                }
                                gui::VolumeChange::Increase => {
                                    volume_change = 1.0;
                                }
                                gui::VolumeChange::Decrease => {
                                    volume_change = -1.0;
                                }
                            }
                            self.ecs
                                .fetch::<SoundSystem>()
                                .change_effect_volume(volume_change);
                            newrunstate = RunState::OptionsMenu {
                                menu_selection: gui::OptionsMenuSelection::EffectsVolume {
                                    change: gui::VolumeChange::None,
                                },
                            }
                        }
                        gui::OptionsMenuSelection::Quit => {
                            newrunstate = RunState::MainMenu {
                                menu_selection: gui::MainMenuSelection::Options,
                            }
                        }
                    },
                }
            }
            RunState::GameOver => {
                let result = gui::game_over(ctx);
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        newrunstate = RunState::MapGeneration;
                        self.mapgen_next_state = Some(RunState::MainMenu {
                            menu_selection: gui::MainMenuSelection::NewGame,
                        });
                    }
                }
            }
            RunState::NextLevel => {
                self.goto_level(1);
                self.mapgen_next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            }
            RunState::PreviousLevel => {
                self.goto_level(-1);
                self.mapgen_next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            }
            RunState::TownPortal => {
                // Spawn the portal
                spawner::spawn_town_portal(&mut self.ecs);

                // Transition
                let map_depth = self.ecs.fetch::<Map>().depth;
                let destination_offset = 0 - (map_depth - 1);
                self.goto_level(destination_offset);
                self.mapgen_next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            }
            RunState::TeleportingToOtherLevel { x, y, depth } => {
                self.goto_level(depth - 1);
                let player_entity = self.ecs.fetch::<Entity>();
                if let Some(pos) = self.ecs.write_storage::<Position>().get_mut(*player_entity) {
                    pos.x = x;
                    pos.y = y;
                }
                let mut ppos = self.ecs.fetch_mut::<rltk::Point>();
                ppos.x = x;
                ppos.y = y;
                self.mapgen_next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            }
            RunState::MagicMapReveal { row } => {
                let mut map = self.ecs.fetch_mut::<Map>();
                for x in 0..map.width {
                    let idx = map.xy_idx(x as i32, row);
                    map.revealed_tiles[idx] = true;
                }
                if row == map.height - 1 {
                    newrunstate = RunState::Ticking;
                } else {
                    newrunstate = RunState::MagicMapReveal { row: row + 1 };
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);

        rltk::render_draw_buffer(ctx).expect("Failed to draw buffer");
        if self.game_options.show_fps {
            ctx.print(1, 59, &format!("FPS: {}", ctx.fps));
        }
    }
}

impl State {
    fn goto_level(&mut self, offset: i32) {
        freeze_level_entities(&mut self.ecs);

        // Build a new map and place the player
        let current_depth = self.ecs.fetch::<Map>().depth;
        self.generate_world_map(current_depth + offset, offset);

        // Notify the player
        gamelog::Logger::new().append("You change level.").log();
    }

    fn game_over_cleanup(&mut self) {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }

        // Spawn a new player
        {
            let player_entity = spawner::player(&mut self.ecs, 0, 0);
            let mut player_entity_writer = self.ecs.write_resource::<Entity>();
            *player_entity_writer = player_entity;
        }

        // Replace the world maps
        self.ecs.insert(map::MasterDungeonMap::new());

        // Build a new map and place the player
        self.generate_world_map(1, 0);
    }

    fn generate_world_map(&mut self, new_depth: i32, offset: i32) {
        self.mapgen_index = 0;
        self.mapgen_timer = 0.0;
        self.mapgen_history.clear();
        let map_building_info = map::level_transition(
            &mut self.ecs,
            new_depth,
            offset,
            self.game_options.show_map_visualizer,
        );
        if let Some(history) = map_building_info {
            self.mapgen_history = history;
        } else {
            map::thaw_level_entities(&mut self.ecs);
        }

        gamelog::clear_log();
        gamelog::Logger::new()
            .append("Welcome to")
            .color(rltk::CYAN)
            .append("Rusty Roguelike")
            .log();

        gamelog::clear_events();
    }
}

fn main() -> rltk::BError {
    use rltk::BTermBuilder;
    let mut context = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_title("Roguelike Tutorial")
        .with_font("vga8x16.png", 8, 16)
        .with_sparse_console(80, 30, "vga8x16.png")
        .with_vsync(false)
        .with_fps_cap(60.0)
        .build()?;
    context.with_post_scanlines(true);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sound_system = systems::sound_system::SoundSystem::new(&stream_handle);
    let mut gs = State {
        ecs: World::new(),
        mapgen_next_state: Some(RunState::MainMenu {
            menu_selection: gui::MainMenuSelection::NewGame,
        }),
        mapgen_index: 0,
        mapgen_history: Vec::new(),
        mapgen_timer: 0.0,
        dispatcher: systems::build(),
        game_options: GameOptions {
            show_fps: true,
            show_map_visualizer: false,
            show_cheat_menu: true,
        },
    };
    provide_all_components!(register, gs.ecs);
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    raws::load_raws();

    gs.ecs.insert(map::MasterDungeonMap::new());
    gs.ecs.insert(Map::new(1, 64, 64, "New Map"));
    gs.ecs.insert(Point::new(0, 0));
    let player_entity = spawner::player(&mut gs.ecs, 0, 0);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MapGeneration {});
    gs.ecs
        .insert(systems::particle_system::ParticleBuilder::new());
    gs.ecs.insert(rex_assets::RexAssets::new());
    gs.ecs.insert(sound_system);

    gs.generate_world_map(1, 0);

    rltk::main_loop(context, gs)
}
