use crate::terrain::dice;
use glam::IVec3;

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
}

impl DiceCoord {
    pub fn new(xyz: IVec3) -> Self {
        Self { xyz }
    }
}

impl ChunkCoord {
    pub fn new(xyz: IVec3) -> Self {
        Self { xyz }
    }
}
