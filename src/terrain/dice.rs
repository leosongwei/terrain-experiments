use crate::mesh::Vertex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Block {
    type_id: u16,
}

pub const DICE_SIZE: usize = 16;

pub trait Dice {
    fn new_empty() -> Self;
    fn build_mesh() -> Vec<Vertex>;
}

pub struct SolidDice {
    array: [[[Block; DICE_SIZE]; DICE_SIZE]; DICE_SIZE],
    gl_id: u16,
}

impl Dice for SolidDice {
    fn new_empty() -> Self {
        Self {
            array: [[[Block { type_id: 0 }; DICE_SIZE]; DICE_SIZE]; DICE_SIZE],
            gl_id: 0,
        }
    }

    fn build_mesh() -> Vec<Vertex> {
        todo!()
    }
}

pub struct Chunk {}
