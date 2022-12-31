use crate::rltk;
use crate::systems::sound_system::SoundSystem;
use crate::{rex_assets::RexAssets, RunState, State};

use super::print_menu_option;

#[derive(PartialEq, Copy, Clone)]
pub enum OptionsMenuSelection {
    ToggleFps,
    ToggleMapVisualizer,
    ToggleCheatMenu,
    BackgroundVolume {
        change: VolumeChange,
    },
    EffectsVolume {
        change: VolumeChange,
    },
    Quit,
}

#[derive(PartialEq, Copy, Clone)]
pub enum VolumeChange {
    Increase,
    Decrease,
    None,
}

#[derive(PartialEq, Copy, Clone)]
pub enum OptionsMenuResult {
    NoSelection { selected: OptionsMenuSelection },
    Selected { selected: OptionsMenuSelection },
}

pub fn options_menu(gs: &mut State, ctx: &mut rltk::BTerm) -> OptionsMenuResult {
    let mut draw_batch = rltk::DrawBatch::new();
    let runstate = gs.ecs.fetch::<RunState>();
    let assets = gs.ecs.fetch::<RexAssets>();
    let sound_system = gs.ecs.fetch::<SoundSystem>();
    ctx.render_xp_sprite(&assets.menu, 0, 5);

    draw_batch.draw_double_box(
        rltk::Rect::with_size(20, 18, 40, 12),
        rltk::ColorPair::new(rltk::RGB::named(rltk::WHEAT), rltk::RGB::named(rltk::BLACK)),
    );

    draw_batch.print_color_centered(
        20,
        "Options",
        rltk::ColorPair::new(
            rltk::RGB::named(rltk::YELLOW),
            rltk::RGB::named(rltk::BLACK),
        ),
    );
    draw_batch.print_color_centered(
        21,
        "Use Up/Down Arrows to Select Option",
        rltk::ColorPair::new(rltk::RGB::named(rltk::GRAY), rltk::RGB::named(rltk::BLACK)),
    );

    draw_batch.submit(6000).expect("Failed to submit");

    let mut y = 24;
    if let RunState::OptionsMenu {
        menu_selection: selection,
    } = *runstate
    {
        let help_text: &str = match selection {
            OptionsMenuSelection::BackgroundVolume { .. } |
            OptionsMenuSelection::EffectsVolume { .. } => "Use Left/Right Arrows to Change Value",
            OptionsMenuSelection::Quit => "Use Enter to Go Back",
            _ => "Use Enter to Toggle Value",
        };
        draw_batch.print_color_centered(
            22,
            help_text,
            rltk::ColorPair::new(rltk::RGB::named(rltk::GRAY), rltk::RGB::named(rltk::BLACK)),
        );

        let fps_tracker_title = format!("Show FPS Tracker: {}", gs.game_options.show_fps);
        print_menu_option(
            &mut draw_batch,
            y,
            selection == OptionsMenuSelection::ToggleFps,
            &fps_tracker_title,
        );
        y += 1;

        let map_visualizer_title = format!(
            "Show Map Visualizer: {}",
            gs.game_options.show_map_visualizer
        );
        print_menu_option(
            &mut draw_batch,
            y,
            selection == OptionsMenuSelection::ToggleMapVisualizer,
            &map_visualizer_title,
        );
        y += 1;

        let cheat_menu_title = format!(
            "Allow Cheat Menu: {}",
            gs.game_options.show_cheat_menu
        );
        print_menu_option(
            &mut draw_batch,
            y,
            selection == OptionsMenuSelection::ToggleCheatMenu,
            &cheat_menu_title,
        );
        y += 1;

        let background_volume_title = format!(
            "Background Volume: {}",
            sound_system.get_background_volume(),
        );
        print_menu_option(
            &mut draw_batch,
            y,
            selection == OptionsMenuSelection::BackgroundVolume { change: VolumeChange::None },
            &background_volume_title,
        );
        y += 1;

        let effect_volume_title = format!(
            "Sound Effect Volume: {}",
            sound_system.get_effects_volume(),
        );
        print_menu_option(
            &mut draw_batch,
            y,
            selection == OptionsMenuSelection::EffectsVolume { change: VolumeChange::None },
            &effect_volume_title,
        );
        y += 1;

        print_menu_option(
            &mut draw_batch,
            y,
            selection == OptionsMenuSelection::Quit,
            "Back",
        );

        draw_batch.submit(6000).expect("Failed to submit");

        match ctx.key {
            None => {
                return OptionsMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                rltk::VirtualKeyCode::Escape => {
                    return OptionsMenuResult::NoSelection {
                        selected: OptionsMenuSelection::Quit,
                    }
                }
                rltk::VirtualKeyCode::Up => {
                    let newselection;
                    match selection {
                        OptionsMenuSelection::ToggleFps => {
                            newselection = OptionsMenuSelection::Quit
                        }
                        OptionsMenuSelection::ToggleMapVisualizer => {
                            newselection = OptionsMenuSelection::ToggleFps
                        }
                        OptionsMenuSelection::ToggleCheatMenu => {
                            newselection = OptionsMenuSelection::ToggleMapVisualizer
                        }
                        OptionsMenuSelection::BackgroundVolume { .. } => {
                            newselection = OptionsMenuSelection::ToggleCheatMenu
                        }
                        OptionsMenuSelection::EffectsVolume { .. } => {
                            newselection = OptionsMenuSelection::BackgroundVolume { change: VolumeChange::None }
                        }
                        OptionsMenuSelection::Quit => {
                            newselection = OptionsMenuSelection::EffectsVolume { change: VolumeChange::None }
                        }
                    }
                    return OptionsMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                rltk::VirtualKeyCode::Down => {
                    let newselection;
                    match selection {
                        OptionsMenuSelection::ToggleFps => {
                            newselection = OptionsMenuSelection::ToggleMapVisualizer
                        }
                        OptionsMenuSelection::ToggleMapVisualizer => {
                            newselection = OptionsMenuSelection::ToggleCheatMenu
                        }
                        OptionsMenuSelection::ToggleCheatMenu => {
                            newselection = OptionsMenuSelection::BackgroundVolume { change: VolumeChange::None }
                        }
                        OptionsMenuSelection::BackgroundVolume { .. } => {
                            newselection = OptionsMenuSelection::EffectsVolume { change: VolumeChange::None }
                        }
                        OptionsMenuSelection::EffectsVolume { .. } => {
                            newselection = OptionsMenuSelection::Quit
                        }
                        OptionsMenuSelection::Quit => {
                            newselection = OptionsMenuSelection::ToggleFps
                        }
                    }
                    return OptionsMenuResult::NoSelection {
                        selected: newselection,
                    };
                }
                rltk::VirtualKeyCode::Left => {
                    match selection {
                        OptionsMenuSelection::BackgroundVolume { .. } => {
                            return OptionsMenuResult::Selected { 
                                selected: OptionsMenuSelection::BackgroundVolume { 
                                    change: VolumeChange::Decrease,
                                } 
                            }
                        }
                        OptionsMenuSelection::EffectsVolume { .. } => {
                            return OptionsMenuResult::Selected { 
                                selected: OptionsMenuSelection::EffectsVolume { 
                                    change: VolumeChange::Decrease,
                                } 
                            }
                        }
                        _ => {}
                    }
                    return OptionsMenuResult::NoSelection {
                        selected: selection,
                    };
                }
                rltk::VirtualKeyCode::Right => {
                    match selection {
                        OptionsMenuSelection::BackgroundVolume { .. } => {
                            return OptionsMenuResult::Selected { 
                                selected: OptionsMenuSelection::BackgroundVolume { 
                                    change: VolumeChange::Increase,
                                } 
                            }
                        }
                        OptionsMenuSelection::EffectsVolume { .. } => {
                            return OptionsMenuResult::Selected { 
                                selected: OptionsMenuSelection::EffectsVolume { 
                                    change: VolumeChange::Increase,
                                } 
                            }
                        }
                        _ => {}
                    }
                    return OptionsMenuResult::NoSelection {
                        selected: selection,
                    };
                }
                rltk::VirtualKeyCode::Return => {
                    return OptionsMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return OptionsMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }

    OptionsMenuResult::NoSelection {
        selected: OptionsMenuSelection::Quit,
    }
}
