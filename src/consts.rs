use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;

// , derive(PartialEq, Eq, Hash, Clone)
pub type EntityTypeVal = &'static str;

#[non_exhaustive]
pub struct EntityType;

impl EntityType {
    pub const PLAYER: EntityTypeVal = "PLAYER";
    pub const WALL: EntityTypeVal = "WALL";
    pub const AIR: EntityTypeVal = "AIR";
}

const LEVEL: [(char, EntityTypeVal); 3] = [
    ('@', EntityType::PLAYER),
    ('#', EntityType::WALL),
    ('.', EntityType::AIR),
];

const TEXTURE: [(EntityTypeVal, char); 3] = [
    (EntityType::PLAYER, '☺'),
    (EntityType::WALL, '█'),
    (EntityType::AIR, ' '),
];

pub fn to_map<K, V>(tuple_list: Vec<(K, V)>) -> HashMap<K, V>
where
    K: Clone + Hash + Eq + PartialEq,
    V: Clone + Hash + Eq + PartialEq,
{
    let hashmap: HashMap<K, V> = tuple_list.iter().cloned().collect();
    hashmap
}

lazy_static! {
    pub static ref TEXTURE_MAP: HashMap<EntityTypeVal, char> =
        to_map::<EntityTypeVal, char>(TEXTURE.to_vec());
    pub static ref LEVEL_MAP: HashMap<char, EntityTypeVal> =
        to_map::<char, EntityTypeVal>(LEVEL.to_vec());
}
