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

struct BlockTypeDefinition {
    id: BlockTypeID,
    name: String,
    texture_top: String,
    texture_bottom: String,
    texture_north: String,
    texture_south: String,
    texture_west: String,
    texture_east: String,
}

pub struct BlockTypeRegistry {
    types: HashMap<BlockTypeID, BlockType>,
}

impl BlockTypeRegistry {
    pub fn new() -> Self {
        const _types: Vec<BlockTypeDefinition> = vec![];

        Self {
            types: HashMap::new(),
        }
    }
}
