use super::{
    AreaEndingPosition, AreaStartingPosition, BuilderChain, CellularAutomataBuilder,
    CullUnreachable, PrefabBuilder, VoronoiSpawning, WaveformCollapseBuilder, XEnd, XStart, YEnd,
    YStart,
};
use crate::map_builders::prefab_builder::prefab_sections::*;

pub fn mushroom_entrance(
    new_depth: i32,
    width: i32,
    height: i32,
    show_visualizer: bool,
) -> BuilderChain {
    let mut chain = BuilderChain::new(
        new_depth,
        width,
        height,
        "Into The Mushroom Grove",
        show_visualizer,
    );
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(AreaEndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    chain.with(VoronoiSpawning::new());
    chain.with(PrefabBuilder::sectional(UNDERGROUND_FORT));
    chain
}

pub fn mushroom_builder(
    new_depth: i32,
    width: i32,
    height: i32,
    show_visualizer: bool,
) -> BuilderChain {
    let mut chain = BuilderChain::new(
        new_depth,
        width,
        height,
        "Into The Mushroom Grove",
        show_visualizer,
    );
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(AreaEndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    chain.with(VoronoiSpawning::new());
    chain
}

pub fn mushroom_exit(
    new_depth: i32,
    width: i32,
    height: i32,
    show_visualizer: bool,
) -> BuilderChain {
    let mut chain = BuilderChain::new(
        new_depth,
        width,
        height,
        "Into The Mushroom Grove",
        show_visualizer,
    );
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(AreaEndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    chain.with(VoronoiSpawning::new());
    chain.with(PrefabBuilder::sectional(DROW_ENTRY));
    chain
}
