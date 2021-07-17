use crate::loading::TextureAssets;
use crate::matcher::{Collectable, Pattern, Slot, SlotContent};
use crate::GameState;
use bevy::prelude::*;
use rand::random;
use std::hint::spin_loop;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(set_camera.system())
                .with_system(prepare_board.system()),
        );
    }
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(400., 300., 999.9)),
        ..OrthographicCameraBundle::new_2d()
    });
}

fn prepare_board(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut board = Board {
        height: 9,
        width: 8,
        slots: vec![],
    };

    for row_index in 0..board.height {
        let mut row = vec![];
        for column_index in 0..board.width {
            let animal: Collectable = random();
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(animal.get_texture(&textures).into()),
                    transform: Transform::from_translation(Vec3::new(
                        column_index as f32 * 64. + 32.,
                        row_index as f32 * 64. + 32.,
                        0.,
                    )),
                    ..SpriteBundle::default()
                })
                .id();
            row.push(SlotContent {
                entity,
                collectable: animal,
            })
        }
        board.slots.push(row);
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    height: usize,
    width: usize,
    slots: Vec<Vec<SlotContent>>,
}

impl Board {
    pub fn find_patterns(&self) -> Vec<Pattern> {
        let mut patterns = vec![];
        patterns.append(&mut self.find_patterns_in_rows());
        patterns.append(&mut self.find_patterns_in_columns());

        patterns
    }

    fn find_patterns_in_rows(&self) -> Vec<Pattern> {
        let mut patterns = vec![];
        let mut count = 0;
        let mut current = None;
        for (row_index, row) in self.slots.iter().enumerate() {
            for (column, content) in row.iter().enumerate() {
                if let Some(animal) = current.take() {
                    if animal == content.collectable {
                        current = Some(animal);
                        count += 1;
                    } else {
                        current = Some(content.collectable.clone());
                        count = 1;
                    }
                } else {
                    current = Some(content.collectable.clone());
                    count = 1;
                }
                if count >= 3 {
                    patterns.push(Pattern::Line {
                        slots: vec![
                            Slot {
                                row: row_index,
                                column,
                            },
                            Slot {
                                row: row_index,
                                column: column - 1,
                            },
                            Slot {
                                row: row_index,
                                column: column - 2,
                            },
                        ],
                    })
                }
            }
            current = None;
            count = 0;
        }

        patterns
    }

    fn find_patterns_in_columns(&self) -> Vec<Pattern> {
        let mut patterns = vec![];
        let mut count = 0;
        let mut current = None;
        for column in 0..self.slots.first().unwrap().len() {
            for row_index in 0..self.slots.len() {
                let content = self.slots.get(row_index).unwrap().get(column).unwrap();
                if let Some(animal) = current.take() {
                    if animal == content.collectable {
                        current = Some(animal);
                        count += 1;
                    } else {
                        current = Some(content.collectable.clone());
                        count = 1;
                    }
                } else {
                    current = Some(content.collectable.clone());
                    count = 1;
                }
                if count >= 3 {
                    patterns.push(Pattern::Line {
                        slots: vec![
                            Slot {
                                row: row_index,
                                column,
                            },
                            Slot {
                                row: row_index - 1,
                                column,
                            },
                            Slot {
                                row: row_index - 2,
                                column,
                            },
                        ],
                    })
                }
            }
            current = None;
            count = 0;
        }

        patterns
    }

    fn neighbors(&self, position: &Slot) -> Vec<Slot> {
        let mut neighbors = vec![];
        match position {
            Slot { row: 0, column: 0 } => {
                neighbors.push(position.walk(1, 0));
                neighbors.push(position.walk(0, 1));
            }
            Slot {
                row: 0,
                column: width,
            } if width == &(self.width - 1) => {
                neighbors.push(position.walk(1, 0));
                neighbors.push(position.walk(0, -1));
            }
            Slot {
                row: height,
                column: width,
            } if width == &(self.width - 1) && height == &(self.height - 1) => {
                neighbors.push(position.walk(-1, 0));
                neighbors.push(position.walk(0, -1));
            }
            Slot {
                row: height,
                column: 0,
            } if height == &(self.height - 1) => {
                neighbors.push(position.walk(-1, 0));
                neighbors.push(position.walk(0, 1));
            }
            Slot { row: 0, column: _ } => {
                neighbors.push(position.walk(0, 1));
                neighbors.push(position.walk(0, -1));
                neighbors.push(position.walk(1, 0));
            }
            Slot { row: _, column: 0 } => {
                neighbors.push(position.walk(1, 0));
                neighbors.push(position.walk(0, 1));
                neighbors.push(position.walk(-1, 0));
            }
            Slot {
                row: height,
                column: _,
            } if height == &(self.height - 1) => {
                neighbors.push(position.walk(0, 1));
                neighbors.push(position.walk(-1, 0));
                neighbors.push(position.walk(0, -1));
            }
            Slot {
                row: _,
                column: width,
            } if width == &(self.width - 1) => {
                neighbors.push(position.walk(-1, 0));
                neighbors.push(position.walk(0, -1));
                neighbors.push(position.walk(1, 0));
            }
            Slot {
                row: height,
                column: width,
            } if height >= &0 && height < &self.height && width >= &0 && width < &self.width => {
                neighbors.push(position.walk(1, 0));
                neighbors.push(position.walk(0, 1));
                neighbors.push(position.walk(-1, 0));
                neighbors.push(position.walk(0, -1));
            }
            slot => panic!("The slot {:?} is outside of the board", slot),
        };
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use crate::matcher::{Board, Collectable, Pattern, Slot, SlotContent};
    use bevy::prelude::*;

    #[test]
    fn finds_lines_in_rows_on_small_board() {
        let mut board = Board {
            height: 3,
            width: 3,
            slots: vec![
                vec![
                    SlotContent {
                        entity: Entity::new(0),
                        collectable: Collectable::BirdOne
                    };
                    3
                ];
                3
            ],
        };
        board.slots.get_mut(1).unwrap().get_mut(1).unwrap().animal = Collectable::Red;

        assert_eq!(
            board.find_patterns_in_rows(),
            vec![
                Pattern::Line {
                    slots: vec![Slot::new(0, 2), Slot::new(0, 1), Slot::new(0, 0),]
                },
                Pattern::Line {
                    slots: vec![Slot::new(2, 2), Slot::new(2, 1), Slot::new(2, 0),]
                }
            ]
        )
    }

    #[test]
    fn finds_lines_in_rows() {
        let size = 5;
        let mut board = Board {
            height: size,
            width: size,
            slots: vec![
                vec![
                    SlotContent {
                        entity: Entity::new(0),
                        collectable: Collectable::BirdOne
                    };
                    size
                ];
                size
            ],
        };
        for index in 0..size {
            board
                .slots
                .get_mut(index)
                .unwrap()
                .get_mut(index)
                .unwrap()
                .animal = Collectable::Red;
        }

        assert_eq!(
            board.find_patterns_in_rows(),
            vec![
                Pattern::Line {
                    slots: vec![Slot::new(0, 3), Slot::new(0, 2), Slot::new(0, 1),]
                },
                Pattern::Line {
                    slots: vec![Slot::new(0, 4), Slot::new(0, 3), Slot::new(0, 2),]
                },
                Pattern::Line {
                    slots: vec![Slot::new(1, 4), Slot::new(1, 3), Slot::new(1, 2),]
                },
                Pattern::Line {
                    slots: vec![Slot::new(3, 2), Slot::new(3, 1), Slot::new(3, 0),]
                },
                Pattern::Line {
                    slots: vec![Slot::new(4, 2), Slot::new(4, 1), Slot::new(4, 0),]
                },
                Pattern::Line {
                    slots: vec![Slot::new(4, 3), Slot::new(4, 2), Slot::new(4, 1),]
                }
            ]
        )
    }

    #[test]
    fn correctly_gives_neighbors() {
        let board = Board {
            height: 5,
            width: 5,
            slots: vec![
                vec![
                    SlotContent {
                        entity: Entity::new(0),
                        collectable: Collectable::BirdOne
                    };
                    5
                ];
                5
            ],
        };
        assert_eq!(
            board.neighbors(&Slot::new(0, 0)),
            vec![Slot::new(1, 0), Slot::new(0, 1)]
        );
        assert_eq!(
            board.neighbors(&Slot::new(0, 3)),
            vec![Slot::new(0, 4), Slot::new(0, 2), Slot::new(1, 3)]
        );
        assert_eq!(
            board.neighbors(&Slot::new(2, 3)),
            vec![
                Slot::new(3, 3),
                Slot::new(2, 4),
                Slot::new(1, 3),
                Slot::new(2, 2)
            ]
        );
    }
}
