use super::components::*;
use crate::{rltk, provide_all_components};
use specs::prelude::*;
use specs::saveload::{
    DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator,
};
use std::convert::Infallible;
use std::fs;
use std::fs::File;
use std::path::Path;

macro_rules! serialize_individually {
    (
        $ecs:expr,
        $( 
            $type:ty
        ),*
    ) => {
        {
            let writer = File::create("./savegame.json").unwrap();
            let mut serializer = serde_json::Serializer::new(writer);
            $(
                SerializeComponents::<Infallible, SimpleMarker<SerializeMe>>::serialize(
                    &( $ecs.read_storage::<$type>(), ),
                    &$ecs.entities(),
                    &$ecs.read_storage::<SimpleMarker<SerializeMe>>(),
                    &mut serializer,
                )
                .unwrap();
            )*
        }
    };
}

macro_rules! deserialize_individually {
    (
        $ecs:expr,
        $( 
            $type:ty
        ),*
    ) => {
        let data = fs::read_to_string("./savegame.json").unwrap();
        let mut de = serde_json::Deserializer::from_str(&data);

        {
            $(
                DeserializeComponents::<Infallible, _>::deserialize(
                    &mut ( &mut $ecs.write_storage::<$type>(), ),
                    &mut $ecs.entities(),
                    &mut $ecs.write_storage::<SimpleMarker<SerializeMe>>(),
                    &mut $ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>(),
                    &mut de,
                )
                .unwrap();
            )*
        }
    };
}

pub fn save_game(ecs: &mut World) {
    // Create helper
    let mapcopy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let dungeon_master = ecs
        .get_mut::<super::map::MasterDungeonMap>()
        .unwrap()
        .clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
    let savehelper2 = ecs
        .create_entity()
        .with(DMSerializationHelper {
            map: dungeon_master,
            log: crate::gamelog::clone_log(),
            events: crate::gamelog::clone_events(),
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();


    provide_all_components!(
        serialize_individually,
        ecs
    );

    // Clean up
    ecs.delete_entity(savehelper).expect("Crash on cleanup");
    ecs.delete_entity(savehelper2).expect("Crash on cleanup");
}

pub fn load_game(ecs: &mut World) {
    {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    provide_all_components!(
        deserialize_individually,
        ecs
    );

    let mut deleteme: Option<Entity> = None;
    let mut deleteme2: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let helper2 = ecs.read_storage::<DMSerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e, h) in (&entities, &helper).join() {
            let mut worldmap = ecs.write_resource::<super::map::Map>();
            *worldmap = h.map.clone();
            crate::spatial::set_size((worldmap.height * worldmap.width) as usize);
            deleteme = Some(e);
        }
        for (e, h) in (&entities, &helper2).join() {
            let mut dungeonmaster = ecs.write_resource::<super::map::MasterDungeonMap>();
            *dungeonmaster = h.map.clone();
            deleteme2 = Some(e);
            crate::gamelog::restore_log(&mut h.log.clone());
            crate::gamelog::load_events(h.events.clone());
        }
        for (e, _p, pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    ecs.delete_entity(deleteme.unwrap())
        .expect("Unable to delete helper");
    ecs.delete_entity(deleteme2.unwrap())
        .expect("Unable to delete helper");
}

pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}

pub fn delete_save() {
    if Path::new("./savegame.json").exists() {
        std::fs::remove_file("./savegame.json").expect("Unable to delete file");
    }
}
