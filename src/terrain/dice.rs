#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Block {
    type_id: u16,
}

const DICE_SIZE: usize = 16;

pub struct Dice {
    array: [[[Block; DICE_SIZE]; DICE_SIZE]; DICE_SIZE],
    gl_id: u16,
}

impl Dice {
    pub fn new() -> Self {
        Dice {
            array: [[[Block { type_id: 0 }; DICE_SIZE]; DICE_SIZE]; DICE_SIZE],
            gl_id: 0,
        }
    }

    pub fn loaded(&self) -> bool {
        if self.gl_id == 0 {
            true
        } else {
            false
        }
    }
}

pub struct Chunk {}
