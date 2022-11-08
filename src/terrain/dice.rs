#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Block {
    type_id: u16,
}

const DICE_SIZE: usize = 16;

pub struct Dice {
    array: [[[Block; DICE_SIZE]; DICE_SIZE]; DICE_SIZE],
}

impl Dice {
    pub fn new() -> Self {
        Dice {
            array: [[[Block { type_id: 0 }; DICE_SIZE]; DICE_SIZE]; DICE_SIZE],
        }
    }
}

pub struct Chunk {}
