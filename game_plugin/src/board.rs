use crate::animate::{Animate, Move};
use crate::loading::{RawTextureAssets, TextureAssets};
use crate::matcher::{Collectable, Pattern, Slot, SlotContent};
use crate::{GameState, SystemLabels};
use bevy::prelude::*;
use rand::{random, thread_rng, Rng};
use std::collections::HashMap;
use std::ops::Deref;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Cauldron::new())
            .insert_resource::<Selected>(None)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(set_camera.system())
                    .with_system(prepare_board.system())
                    .with_system(setup_shop.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(
                        take_patterns
                            .system()
                            .label(SystemLabels::MatchPatterns)
                            .after(SystemLabels::Animate),
                    )
                    .with_system(
                        user_selection
                            .system()
                            .label(SystemLabels::UserInput)
                            .after(SystemLabels::MatchPatterns),
                    ),
            );
    }
}

pub type Selected = Option<Slot>;

#[derive(Debug)]
pub struct Cauldron {
    pub recipe: Recipe,
    pub content: HashMap<Collectable, usize>,
}

impl Cauldron {
    pub fn new() -> Self {
        Cauldron {
            recipe: Recipe::build_random(),
            content: HashMap::new(),
        }
    }

    pub fn new_recipe(&mut self) {
        self.recipe = Recipe::build_random();
    }
}

#[derive(Debug)]
pub struct Recipe {
    pub ingredients: Vec<Ingredients>,
}

impl Recipe {
    // ToDo: make random

    pub fn build_random() -> Self {
        // get three random collectables
        let mut rng = thread_rng();
        let mut collectables = vec![];
        while collectables.len() < 3 {
            let random = rng.gen::<Collectable>();
            if collectables.contains(&random) {
                continue;
            }
            collectables.push(random);
        }
        let ingredients = collectables
            .drain(..)
            .map(|collectable| Ingredients {
                amount: rng.gen_range(4..16),
                collectable,
            })
            .collect();
        Recipe { ingredients }
    }
}

#[derive(Debug)]
pub struct Ingredients {
    pub amount: usize,
    pub collectable: Collectable,
}

fn setup_shop(
    mut commands: Commands,
    textures: Res<RawTextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.cauldron.clone().into()),
        transform: Transform::from_translation(Vec3::new(800. - 132., 96., 1.)),
        ..SpriteBundle::default()
    });
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.shelf.clone().into()),
        transform: Transform::from_translation(Vec3::new(400., 300., 0.)),
        ..SpriteBundle::default()
    });
}

fn set_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_translation(Vec3::new(400., 300., 999.9)),
        ..OrthographicCameraBundle::new_2d()
    });
}

fn prepare_board(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut board = Board {
        height: 9,
        width: 8,
        animating: true,
        slots: vec![],
    };

    let animation_offset = board.height as f32 * 64.;
    for column_index in 0..board.width {
        let mut column = vec![];
        for row_index in 0..board.height {
            let goal = Vec2::new(
                column_index as f32 * 64. + 32. + 12.,
                row_index as f32 * 64. + 32. + 12.,
            );
            let slot = Slot {
                row: row_index,
                column: column_index,
            };
            let slot_content =
                drop_random_collectable(&mut commands, goal, animation_offset, slot, &textures);
            column.push(slot_content);
        }
        board.slots.push(column);
    }
    commands.insert_resource(board);
}

fn take_patterns(mut board: ResMut<Board>, mut commands: Commands, textures: Res<TextureAssets>) {
    if board.animating {
        return;
    }
    let mut patterns = board.find_patterns();
    if patterns.is_empty() {
        return;
    }

    let mut pattern_slots = patterns
        .drain(..)
        .flat_map(|pattern| match pattern {
            Pattern::Line { slots } => slots,
        })
        .collect::<Vec<Slot>>();
    pattern_slots.sort();
    pattern_slots.dedup();

    let entities: Vec<SlotContent> = pattern_slots
        .iter()
        .map(|slot| board.get_content(slot))
        .collect();

    for SlotContent {
        entity,
        collectable,
    } in entities
    {
        commands
            .entity(entity)
            .remove::<Slot>()
            .insert(collectable.get_animation())
            .insert(vec![Move::process()]);
    }

    let slots_to_animate = board.remove_slots(pattern_slots, &mut commands, &textures);
    for slot in slots_to_animate {
        let content = board.get_content(&slot);
        commands
            .entity(content.entity)
            .insert(vec![Move::move_to(Vec2::new(
                slot.column as f32 * 64. + 32. + 12.,
                slot.row as f32 * 64. + 32. + 12.,
            ))])
            .insert(slot);
    }
}

fn user_selection(
    mut commands: Commands,
    mut selection: ResMut<Selected>,
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
) {
    if !board.animating && mouse_buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().expect("No primary window found");
        if let Some(position) = window.cursor_position() {
            if position.x < 12. {
                return;
            }
            let column = ((position.x - 12.) / 64.) as usize;
            let row = (position.y / 64.) as usize;
            let slot = Slot { row, column };
            if slot.row >= board.height || slot.column >= board.width {
                return;
            }
            let tile_two = board.get_content(&slot);
            if let Some(one) = selection.deref() {
                let neighbors = board.neighbors(one);
                let tile_one = board.get_content(one);
                if !neighbors.contains(&slot) {
                    commands
                        .entity(tile_one.entity)
                        .remove::<Animate>()
                        .insert(TextureAtlasSprite::default());
                    commands
                        .entity(tile_two.entity)
                        .insert(tile_two.collectable.get_animation());
                    *selection = Some(slot);
                    return;
                }
                if !board.has_pattern_after_switch(one, &slot) {
                    // ToDo: "No" sound + small animation?
                    return;
                }
                commands
                    .entity(tile_one.entity)
                    .insert(vec![Move::move_to_slot(&slot)]);
                commands
                    .entity(tile_two.entity)
                    .insert(vec![Move::move_to_slot(one)]);
                board.switch(one, &slot, &mut commands);
                commands
                    .entity(tile_one.entity)
                    .remove::<Animate>()
                    .insert(TextureAtlasSprite::default());
                *selection = None;
            } else {
                commands
                    .entity(tile_two.entity)
                    .insert(tile_two.collectable.get_animation());
                *selection = Some(slot);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    height: usize,
    width: usize,
    pub animating: bool,
    slots: Vec<Vec<SlotContent>>,
}

impl Board {
    pub fn find_patterns(&self) -> Vec<Pattern> {
        let mut patterns = vec![];
        patterns.append(&mut self.find_patterns_in_columns());
        patterns.append(&mut self.find_patterns_in_rows());

        patterns
    }

    pub fn remove_slots(
        &mut self,
        mut slots: Vec<Slot>,
        commands: &mut Commands,
        textures: &TextureAssets,
    ) -> Vec<Slot> {
        slots.sort();
        slots.reverse();

        let mut slots_to_animate = vec![];

        let mut column = slots.first().unwrap().column;
        let mut row = slots.first().unwrap().row;
        for slot in slots {
            if column != slot.column {
                for row in row..self.slots.get(column).unwrap().len() {
                    slots_to_animate.push(Slot { row, column })
                }
                self.fill_column(column, commands, textures);
                row = slot.row;
                column = slot.column;
            } else {
                row = slot.row;
            }
            self.slots.get_mut(slot.column).unwrap().remove(slot.row);
        }
        for row in row..self.slots.get(column).unwrap().len() {
            slots_to_animate.push(Slot { row, column })
        }
        self.fill_column(column, commands, textures);

        slots_to_animate
    }

    fn fill_column(&mut self, column: usize, commands: &mut Commands, textures: &TextureAssets) {
        let full_rows = self.slots.get(column).unwrap().len();
        let slots_to_drop = self.height - full_rows;
        let mut new_content = vec![];
        for row in full_rows..self.height {
            let goal = Vec2::new(
                column as f32 * 64. + 32. + 12.,
                row as f32 * 64. + 32. + 12.,
            );
            let slot_content = drop_random_collectable(
                commands,
                goal,
                slots_to_drop as f32 * 64.,
                Slot { row, column },
                textures,
            );
            new_content.push(slot_content);
        }
        self.slots.get_mut(column).unwrap().append(&mut new_content);
    }

    pub fn get_content(&self, slot: &Slot) -> SlotContent {
        self.slots
            .get(slot.column)
            .unwrap()
            .get(slot.row)
            .unwrap()
            .clone()
    }

    pub fn switch(&mut self, one: &Slot, two: &Slot, commands: &mut Commands) {
        let tile_one = self.get_content(one);
        let tile_two = self.get_content(two);

        commands
            .entity(tile_one.entity)
            .insert(vec![Move::move_to_slot(two)])
            .insert(two.clone());
        commands
            .entity(tile_two.entity)
            .insert(vec![Move::move_to_slot(one)])
            .insert(one.clone());

        self.switch_in_slots(one, tile_one, two, tile_two);
    }

    fn switch_in_slots(
        &mut self,
        one: &Slot,
        tile_one: SlotContent,
        two: &Slot,
        tile_two: SlotContent,
    ) {
        self.slots.get_mut(one.column).unwrap().remove(one.row);
        self.slots
            .get_mut(one.column)
            .unwrap()
            .insert(one.row, tile_two);
        self.slots.get_mut(two.column).unwrap().remove(two.row);
        self.slots
            .get_mut(two.column)
            .unwrap()
            .insert(two.row, tile_one);
    }

    pub fn has_pattern_after_switch(&mut self, one: &Slot, two: &Slot) -> bool {
        let tile_one = self.get_content(one);
        let tile_two = self.get_content(two);
        self.switch_in_slots(one, tile_one, two, tile_two);

        let has_patterns = !self.find_patterns().is_empty();

        let tile_one = self.get_content(one);
        let tile_two = self.get_content(two);
        self.switch_in_slots(one, tile_one, two, tile_two);

        has_patterns
    }

    fn find_patterns_in_columns(&self) -> Vec<Pattern> {
        let mut patterns = vec![];
        let mut count = 0;
        let mut current = None;
        for (column_index, column) in self.slots.iter().enumerate() {
            for (row, content) in column.iter().enumerate() {
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
                                row,
                                column: column_index,
                            },
                            Slot {
                                row: row - 1,
                                column: column_index,
                            },
                            Slot {
                                row: row - 2,
                                column: column_index,
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

    fn find_patterns_in_rows(&self) -> Vec<Pattern> {
        let mut patterns = vec![];
        let mut count = 0;
        let mut current = None;
        for row in 0..self.slots.first().unwrap().len() {
            for column_index in 0..self.slots.len() {
                let content = self.slots.get(column_index).unwrap().get(row).unwrap();
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
                                column: column_index,
                                row,
                            },
                            Slot {
                                column: column_index - 1,
                                row,
                            },
                            Slot {
                                column: column_index - 2,
                                row,
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

fn drop_random_collectable(
    commands: &mut Commands,
    goal: Vec2,
    drop_height: f32,
    slot: Slot,
    textures: &TextureAssets,
) -> SlotContent {
    let collectable: Collectable = random();
    let entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: collectable.get_texture(textures),
            transform: Transform::from_translation(Vec3::new(goal.x, goal.y + drop_height, 5.)),
            ..SpriteSheetBundle::default()
        })
        .insert(vec![Move::move_to(goal)])
        .insert(slot)
        .insert(collectable.clone())
        .id();
    SlotContent {
        entity,
        collectable,
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::matcher::{Collectable, Pattern, Slot, SlotContent};
    use bevy::prelude::*;

    #[test]
    fn finds_lines_in_rows_on_small_board() {
        let mut board = Board {
            height: 3,
            width: 3,
            animating: false,
            slots: vec![
                vec![
                    SlotContent {
                        entity: Entity::new(0),
                        collectable: Collectable::Green
                    };
                    3
                ];
                3
            ],
        };
        board
            .slots
            .get_mut(1)
            .unwrap()
            .get_mut(1)
            .unwrap()
            .collectable = Collectable::Red;

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
            animating: false,
            slots: vec![
                vec![
                    SlotContent {
                        entity: Entity::new(0),
                        collectable: Collectable::Green
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
                .collectable = Collectable::Red;
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
            animating: false,
            slots: vec![
                vec![
                    SlotContent {
                        entity: Entity::new(0),
                        collectable: Collectable::Green
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
