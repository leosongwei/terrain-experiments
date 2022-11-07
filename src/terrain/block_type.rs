pub struct BlockType {
    id: u16,
    name: String,
    texture_id_top: u16,
    texture_id_bottom: u16,
    texture_id_north: u16,
    texture_id_south: u16,
    texture_id_west: u16,
    texture_id_east: u16,
}

impl BlockType {
    pub fn id(&self) -> u16 {
        return self.id;
    }

    pub fn upper_texture_id() {}
}

pub struct SimpleBlockTypeFactory {}

impl SimpleBlockTypeFactory {
    pub fn get_block_types() {
        let types = vec![
            BlockType {
                id: 0,
                name: "void".to_string(),
                texture_id_top: 0u16,
                texture_id_bottom: 0u16,
                texture_id_north: 0u16,
                texture_id_south: 0u16,
                texture_id_west: 0u16,
                texture_id_east: 0u16,
            },
            BlockType {
                id: 1,
                name: "air".to_string(),
                texture_id_top: 0u16,
                texture_id_bottom: 0u16,
                texture_id_north: 0u16,
                texture_id_south: 0u16,
                texture_id_west: 0u16,
                texture_id_east: 0u16,
            },
        ];
    }
}
