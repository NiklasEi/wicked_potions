use crate::animate::Animate;
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
    pub fn walk(&self, row_delta: i64, column_delta: i64) -> Slot {
        Slot {
            row: usize::try_from(self.row as i64 + row_delta)
                .expect("Overflow navigating the board"),
            column: usize::try_from(self.column as i64 + column_delta)
                .expect("Overflow navigating the board"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Collectable {
    Eye,
    Tongue,
    Frog,
    Heart,
    Spider,
    Jar,
    Red,
    Yellow,
    Blue,
}

impl Distribution<Collectable> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Collectable {
        match rng.gen_range(0..8) {
            0 => Collectable::Eye,
            1 => Collectable::Tongue,
            2 => Collectable::Frog,
            3 => Collectable::Heart,
            4 => Collectable::Spider,
            5 => Collectable::Jar,
            6 => Collectable::Red,
            _ => Collectable::Yellow,
        }
    }
}

impl Collectable {
    pub fn get_texture(&self, assets: &TextureAssets) -> Handle<TextureAtlas> {
        match self {
            &Collectable::Eye => assets.eye.clone(),
            &Collectable::Tongue => assets.tongue.clone(),
            &Collectable::Frog => assets.frog.clone(),
            &Collectable::Heart => assets.heart.clone(),
            &Collectable::Spider => assets.spider.clone(),
            &Collectable::Jar => assets.jar.clone(),
            &Collectable::Red => assets.red.clone(),
            &Collectable::Yellow => assets.yellow.clone(),
            &Collectable::Blue => assets.blue.clone(),
        }
    }

    pub fn get_animation(&self) -> Animate {
        match self {
            &Collectable::Eye => Animate {
                frames: 6,
                loop_animation: false,
            },
            &Collectable::Tongue => Animate {
                frames: 6,
                loop_animation: true,
            },
            &Collectable::Frog => Animate {
                frames: 6,
                loop_animation: false,
            },
            &Collectable::Heart => Animate {
                frames: 6,
                loop_animation: true,
            },
            &Collectable::Spider => Animate {
                frames: 6,
                loop_animation: false,
            },
            &Collectable::Jar => Animate {
                frames: 4,
                loop_animation: true,
            },
            &Collectable::Red => Animate {
                frames: 6,
                loop_animation: true,
            },
            &Collectable::Yellow => Animate {
                frames: 6,
                loop_animation: true,
            },
            &Collectable::Blue => Animate {
                frames: 6,
                loop_animation: true,
            },
        }
    }
}
