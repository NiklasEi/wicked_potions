use crate::loading::TextureAssets;
use bevy::prelude::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Line { slots: Vec<Slot> },
}

#[derive(Clone, Debug)]
pub struct SlotContent {
    pub entity: Entity,
    pub collectable: Collectable,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Slot {
    pub column: usize,
    pub row: usize,
}

impl Slot {
    pub fn new(row: usize, column: usize) -> Self {
        Slot { row, column }
    }

    pub fn walk(&self, row_delta: i64, column_delta: i64) -> Slot {
        Slot {
            row: usize::try_from(self.row as i64 + row_delta)
                .expect("Overflow navigating the board"),
            column: usize::try_from(self.column as i64 + column_delta)
                .expect("Overflow navigating the board"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Collectable {
    Eye,
    Red,
    Green,
    Blue,
}

impl Distribution<Collectable> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Collectable {
        match rng.gen_range(0..4) {
            0 => Collectable::Eye,
            1 => Collectable::Red,
            2 => Collectable::Green,
            _ => Collectable::Blue,
        }
    }
}

impl Collectable {
    pub fn get_texture(&self, assets: &TextureAssets) -> Handle<Texture> {
        match self {
            &Collectable::Eye => assets.eye.clone(),
            &Collectable::Red => assets.red.clone(),
            &Collectable::Green => assets.green.clone(),
            &Collectable::Blue => assets.green.clone(),
        }
    }
}
