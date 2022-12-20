use crate::rltk;
use crate::rltk::EMBED;

rltk::embedded_resource!(SMALL_DUNGEON, "../resources/SmallDungeon_80x50.xp");

pub struct RexAssets {
    pub menu: rltk::XpFile,
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(SMALL_DUNGEON, "../resources/SmallDungeon_80x50.xp");

        RexAssets {
            menu: rltk::XpFile::from_resource("../resources/SmallDungeon_80x50.xp").unwrap(),
        }
    }
}
