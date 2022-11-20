use crate::terrain::static_block_type_definitions;
use std::collections::HashMap;

type BlockTypeID = u16;

pub struct BlockType {
    id: BlockTypeID,
    name: String,
    texture_id_top: u16,
    texture_id_bottom: u16,
    texture_id_north: u16,
    texture_id_south: u16,
    texture_id_west: u16,
    texture_id_east: u16,
}

impl BlockType {
    pub fn id(&self) -> BlockTypeID {
        return self.id;
    }
    pub fn texture_id_top(&self) -> u16 {
        return self.texture_id_top;
    }
    pub fn texture_id_bottom(&self) -> u16 {
        return self.texture_id_bottom;
    }
    pub fn texture_id_north(&self) -> u16 {
        return self.texture_id_north;
    }
    pub fn texture_id_south(&self) -> u16 {
        return self.texture_id_south;
    }
    pub fn texture_id_west(&self) -> u16 {
        return self.texture_id_west;
    }
    pub fn texture_id_east(&self) -> u16 {
        return self.texture_id_east;
    }
}

pub struct StaticBlockTypeDefinition {
    pub name: &'static str,
    pub texture_top: &'static str,
    pub texture_bottom: &'static str,
    pub texture_north: &'static str,
    pub texture_south: &'static str,
    pub texture_west: &'static str,
    pub texture_east: &'static str,
}

pub struct BlockTypeRegistry {
    types: HashMap<BlockTypeID, BlockType>,
}

impl BlockTypeRegistry {
    pub fn new() -> Self {
        let _types: Vec<StaticBlockTypeDefinition> =
            static_block_type_definitions::block_type_definitions();

        Self {
            types: HashMap::new(),
        }
    }
}
