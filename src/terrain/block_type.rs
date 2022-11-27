use crate::assets::texture::{TextureArray, TextureID};
use crate::terrain::static_block_type_definitions;
use std::collections::HashMap;

type BlockTypeID = u16;

pub struct BlockType {
    id: BlockTypeID,
    name: String,
    texture_id_top: TextureID,
    texture_id_bottom: TextureID,
    texture_id_north: TextureID,
    texture_id_south: TextureID,
    texture_id_west: TextureID,
    texture_id_east: TextureID,
}

impl BlockType {
    pub fn id(&self) -> BlockTypeID {
        return self.id;
    }
    pub fn texture_id_top(&self) -> TextureID {
        return self.texture_id_top;
    }
    pub fn texture_id_bottom(&self) -> TextureID {
        return self.texture_id_bottom;
    }
    pub fn texture_id_north(&self) -> TextureID {
        return self.texture_id_north;
    }
    pub fn texture_id_south(&self) -> TextureID {
        return self.texture_id_south;
    }
    pub fn texture_id_west(&self) -> TextureID {
        return self.texture_id_west;
    }
    pub fn texture_id_east(&self) -> TextureID {
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
    pub fn new(main_texture_array: &TextureArray) -> Self {
        let _types: Vec<StaticBlockTypeDefinition> =
            static_block_type_definitions::block_type_definitions();
        let mut types: HashMap<BlockTypeID, BlockType> = HashMap::new();
        let mut current_id: BlockTypeID = 0;
        for definition in _types {
            let block_type = BlockType {
                id: current_id,
                name: definition.name.to_string(),
                texture_id_top: main_texture_array.get_id(definition.texture_top),
                texture_id_bottom: main_texture_array.get_id(definition.texture_bottom),
                texture_id_north: main_texture_array.get_id(definition.texture_north),
                texture_id_south: main_texture_array.get_id(definition.texture_south),
                texture_id_west: main_texture_array.get_id(definition.texture_west),
                texture_id_east: main_texture_array.get_id(definition.texture_east),
            };
            types.insert(current_id, block_type);
            current_id += 1;
        }

        Self { types }
    }
}
