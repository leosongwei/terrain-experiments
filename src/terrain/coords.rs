use crate::terrain::chunk;
use crate::terrain::dice;
use glam::IVec3;

use super::chunk::CHUNK_SIZE;

struct WorldCoord {
    xyz: IVec3,
}
struct BlockCoord {
    xyz: IVec3,
}
struct DiceCoord {
    xyz: IVec3,
}
struct ChunkCoord {
    xyz: IVec3,
}

impl WorldCoord {
    pub fn new(xyz: IVec3) -> Self {
        Self { xyz }
    }
}

impl BlockCoord {
    pub fn new(xyz: IVec3) -> Self {
        Self { xyz }
    }

    pub fn toWorldCoord(&self, dice_coord: DiceCoord, chunk_coord: ChunkCoord) -> WorldCoord {
        let world_x = self.xyz.x
            + dice_coord.xyz.x * dice::DICE_SIZE as i32
            + chunk_coord.xyz.x * chunk::CHUNK_SIZE as i32;
        let world_y = self.xyz.y
            + dice_coord.xyz.y * dice::DICE_SIZE as i32
            + chunk_coord.xyz.y * chunk::CHUNK_SIZE as i32;
        let world_z = self.xyz.z
            + dice_coord.xyz.z * dice::DICE_SIZE as i32
            + chunk_coord.xyz.z * chunk::CHUNK_SIZE as i32;
        WorldCoord {
            xyz: IVec3::new(world_x, world_y, world_z),
        }
    }
}

impl DiceCoord {
    pub fn new(xyz: IVec3) -> Self {
        Self { xyz }
    }

    pub fn baseWorldCoord(&self, chunk_coord: ChunkCoord) -> WorldCoord {
        WorldCoord {
            xyz: IVec3::new(
                self.xyz.x * dice::DICE_SIZE as i32 + chunk_coord.xyz.x * CHUNK_SIZE as i32,
                self.xyz.y * dice::DICE_SIZE as i32 + chunk_coord.xyz.y * CHUNK_SIZE as i32,
                self.xyz.z * dice::DICE_SIZE as i32 + chunk_coord.xyz.z * CHUNK_SIZE as i32,
            ),
        }
    }
}

impl ChunkCoord {
    pub fn new(xyz: IVec3) -> Self {
        Self { xyz }
    }

    pub fn baseWorldCoord(&self) -> WorldCoord {
        WorldCoord {
            xyz: IVec3::new(
                self.xyz.x * CHUNK_SIZE as i32,
                self.xyz.y * CHUNK_SIZE as i32,
                self.xyz.z * CHUNK_SIZE as i32,
            ),
        }
    }
}
