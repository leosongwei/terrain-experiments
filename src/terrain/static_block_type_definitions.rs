use crate::terrain::block_type::StaticBlockTypeDefinition;

pub fn block_type_definitions() -> Vec<StaticBlockTypeDefinition> {
    return vec![
        StaticBlockTypeDefinition {
            name: "not_generated",
            texture_top: "nope",
            texture_bottom: "nope",
            texture_north: "nope",
            texture_south: "nope",
            texture_west: "nope",
            texture_east: "nope",
        },
        StaticBlockTypeDefinition {
            name: "air",
            texture_top: "nope",
            texture_bottom: "nope",
            texture_north: "nope",
            texture_south: "nope",
            texture_west: "nope",
            texture_east: "nope",
        },
        StaticBlockTypeDefinition {
            name: "dirt",
            texture_top: "dirt",
            texture_bottom: "dirt",
            texture_north: "dirt",
            texture_south: "dirt",
            texture_west: "dirt",
            texture_east: "dirt",
        },
        StaticBlockTypeDefinition {
            name: "grass",
            texture_top: "grass_top",
            texture_bottom: "dirt",
            texture_north: "grass_side",
            texture_south: "grass_side",
            texture_west: "grass_side",
            texture_east: "grass_side",
        },
    ];
}
