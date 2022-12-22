use std::{iter::FusedIterator, str::Lines};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 22;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut lines = input.lines();
        let maze = Maze::parse(&mut lines);
        let instructions = InstructionsIter::from_str(lines.next().unwrap());
        debug_assert_eq!(lines.next(), None);

        let mut location = maze.get_start_location();
        for instruction in instructions {
            maze.execute_instruction(&mut location, instruction, &None);
        }

        location.get_solution()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut lines = input.lines();
        let maze = Maze::parse(&mut lines);
        let instructions = InstructionsIter::from_str(lines.next().unwrap());
        debug_assert_eq!(lines.next(), None);

        // note: could hardcode this, it's always 4 for sample and 50 for real input
        // TODO: move impl to maze
        let cube_size = {
            let total_tiles = maze.0.iter().map(|x| x.1.len()).sum::<usize>();
            let tile_per_side = total_tiles / 6;
            (tile_per_side as f64).sqrt() as usize
        };

        // Create a map between a position in the 2D grid, and a cube side id
        let cube_side_id_map = maze.create_grid_map(cube_size);

        // Map the grid chunks onto a cube
        let cube = {
            let mut partial_cube = Cube::<Option<CubeSideContent>>::default();

            // write first side to the top
            let start_side = cube_side_id_map[0];
            *partial_cube.get_side_mut(CubeFace(Direction3D::TOP)) = Some(CubeSideContent {
                id: start_side.0,
                facing_direction: Direction3D::ZPos,
            });

            // loop over the sides that neighbour any of the sides that are already in
            while partial_cube.sides.iter().any(|s| s.is_none()) {
                // find a side we have not mapped yet
                let mut unused_sides_iter = cube_side_id_map.iter().filter(|(pos, _)| {
                    !partial_cube
                        .sides
                        .iter()
                        .filter_map(|x| x.as_ref())
                        .any(|content| content.id == *pos)
                });

                // find an unused side that has a used neighbour
                let (unused_side, used_neighbour) = unused_sides_iter
                    .find_map(|(side_id, side_pos)| {
                        // TODO: map with Direction2D::to_unit_vec()?
                        let neighbours = [
                            (side_pos.0 + 1, side_pos.1),
                            (side_pos.0 - 1, side_pos.1),
                            (side_pos.0, side_pos.1 + 1),
                            (side_pos.0, side_pos.1 - 1),
                        ];

                        partial_cube
                            .sides
                            .iter()
                            .filter_map(|x| x.as_ref())
                            .map(|content| content.id)
                            .filter_map(|neighbour_id| {
                                // (neighbour_id, neighbour_pos)
                                let neighbour_pos = cube_side_id_map
                                    .iter()
                                    .find(|x| x.0 == neighbour_id)
                                    .unwrap()
                                    .1;

                                if neighbours.contains(&neighbour_pos) {
                                    Some(((side_id, side_pos), (neighbour_id, neighbour_pos)))
                                } else {
                                    None
                                }
                            })
                            .next()
                    })
                    .expect("there should be an unused side");

                // Calculate the direction we need to place the new face on
                let direction_from_used_neighbour = Direction2D::from_unit_vector((
                    unused_side.1 .0 - used_neighbour.1 .0,
                    -(unused_side.1 .1 - used_neighbour.1 .1), // switching between coordinate spaces
                ));

                // get the face of the reference face
                let reference_face = partial_cube
                    .sides
                    .iter()
                    .position(|side| match side {
                        Some(side) => side.id == used_neighbour.0,
                        None => false,
                    })
                    .unwrap();
                let reference_face = Cube::<()>::index_to_face(reference_face);
                let reference_content = partial_cube.get_side(reference_face).as_ref().unwrap();

                // CubeFace::calculate_neighbour(&self, facing, rotate_direction)
                let (neighbour_face, neighbour_facing) = reference_face.calculate_neighbour(
                    reference_content.facing_direction,
                    direction_from_used_neighbour,
                );

                *partial_cube.get_side_mut(neighbour_face) = Some(CubeSideContent {
                    id: *unused_side.0,
                    facing_direction: neighbour_facing,
                });
            }

            // map the cube to one without options
            Cube::<CubeSideContent> {
                sides: partial_cube
                    .sides
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            }
        };

        let wrap_info = Some(CubeWrappingInfo {
            cube_size,
            cube_side_id_map,
            cube,
        });

        let mut location = maze.get_start_location();
        for instruction in instructions {
            maze.execute_instruction(&mut location, instruction, &wrap_info);
        }

        location.get_solution()
    }
}

#[derive(Debug)]
struct Maze(Vec<MazeLine>);

impl Maze {
    pub fn parse(lines: &mut Lines) -> Self {
        Self(
            lines
                .take_while(|l| !l.is_empty())
                .map(MazeLine::parse)
                .collect(),
        )
    }

    pub fn get_start_location(&self) -> Location {
        Location {
            position: (self.0[0].0, 0),
            heading: 0,
        }
    }

    pub fn execute_instruction(
        &self,
        location: &mut Location,
        instruction: Instruction,
        wrap_info: &Option<CubeWrappingInfo>,
    ) {
        match instruction {
            Instruction::RotateLeft => location.rotate_left(),
            Instruction::RotateRight => location.rotate_right(),
            Instruction::Move(count) => {
                // TODO: use fold instead
                for _ in 0..count {
                    if let Some((wrapped_position, new_heading)) =
                        self.try_move(location, wrap_info)
                    {
                        location.position = wrapped_position;
                        location.heading = new_heading;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn try_move(
        &self,
        location: &mut Location,
        wrap_info: &Option<CubeWrappingInfo>,
    ) -> Option<((isize, isize), usize)> {
        let direction = location.get_direction();
        let new_position = (
            (location.position.0 + direction.0),
            (location.position.1 + direction.1),
        );

        match self.is_open_tile(new_position) {
            // the new position is visitable
            Some(true) => Some((new_position, location.heading)),
            // the new position cannot be visited, it is blocked
            Some(false) => None,
            // position needs to wrap
            None => {
                let (new_pos, new_dir) = match wrap_info {
                    Some(wrap_info) => {
                        // we are outside of 2D bounds, so try to figure out what should have been here
                        let old_direction = Direction2D::from_unit_vector(direction);
                        // flip coordinate system
                        let old_direction = match old_direction {
                            Direction2D::YPos => Direction2D::YNeg,
                            Direction2D::YNeg => Direction2D::YPos,
                            Direction2D::XPos | Direction2D::XNeg => old_direction,
                        };
                        let old_side_id = {
                            let grid_pos = (
                                location.position.0 / wrap_info.cube_size as isize,
                                location.position.1 / wrap_info.cube_size as isize,
                            );

                            wrap_info
                                .cube_side_id_map
                                .iter()
                                .find(|(_id, pos)| *pos == grid_pos)
                                .unwrap()
                                .0
                        };

                        let old_side_index = wrap_info
                            .cube
                            .sides
                            .iter()
                            .position(|side| side.id == old_side_id)
                            .unwrap();
                        let old_side = Cube::<()>::index_to_face(old_side_index);

                        let old_side_content = wrap_info
                            .cube
                            .sides
                            .iter()
                            .find(|s| s.id == old_side_id)
                            .unwrap();

                        // TODO: going off the bottom here, and it broke
                        // zneg.rotate_around(yneg)
                        let absolute_direction = old_side_content.facing_direction.rotate_around(
                            old_side.0.neg(),
                            old_direction.clockwise_rotations() as isize,
                        );

                        // this is the face of the cube we land on
                        let new_side = CubeFace(absolute_direction);
                        let new_side_content = wrap_info.cube.get_side(new_side);
                        let new_side_grid_position = wrap_info
                            .cube_side_id_map
                            .iter()
                            .find(|x| x.0 == new_side_content.id)
                            .unwrap()
                            .1;

                        // figure out the direction of the edge we start on. The absolute direction of that edge will be the old facing direction.
                        let new_edge_abs_dir = old_side.0;

                        // we don't need the absolute 3D direction, we need the relative 2D direction (relative to its facing direction, that is)
                        // to get that, we need to find *its* difference with the 3D facing direction of the new side
                        // I need to find how much I need to rotate the facing direction around the face center (normal vector) to make them line up
                        // this is a hacky way of doing it but it should always work, since we only rotate with 90 degrees
                        let rotation_count = [0, 1, 2, 3]
                            .into_iter()
                            .find(|num| {
                                let ret = new_side_content
                                    .facing_direction
                                    .rotate_around(new_side.0.neg(), *num);
                                ret == new_edge_abs_dir
                            })
                            .unwrap();

                        let local_edge_direction =
                            Direction2D::from_clockwise_rotations(rotation_count as usize);

                        // we now need to calculate the exact block to start at. so, we need the distance from the edge?
                        let old_rel_pos = (
                            location.position.0 % wrap_info.cube_size as isize,
                            location.position.1 % wrap_info.cube_size as isize,
                        );
                        let old_edge_pos = match old_direction {
                            Direction2D::TOP | Direction2D::BOTTOM => old_rel_pos.0,
                            Direction2D::LEFT | Direction2D::RIGHT => old_rel_pos.1,
                        };

                        let magic_old = match old_direction {
                            Direction2D::LEFT | Direction2D::BOTTOM => true,
                            Direction2D::RIGHT | Direction2D::TOP => false,
                        };
                        let magic_new = match local_edge_direction {
                            Direction2D::LEFT | Direction2D::BOTTOM => true,
                            Direction2D::RIGHT | Direction2D::TOP => false,
                        };
                        let should_flip = magic_old == magic_new;
                        let new_edge_pos = if should_flip {
                            wrap_info.cube_size as isize - 1 - old_edge_pos
                        } else {
                            old_edge_pos
                        };

                        let new_rel_pos = match local_edge_direction {
                            Direction2D::TOP => (new_edge_pos, 0),
                            Direction2D::BOTTOM => (new_edge_pos, wrap_info.cube_size as isize - 1),
                            Direction2D::LEFT => (0, new_edge_pos),
                            Direction2D::RIGHT => (wrap_info.cube_size as isize - 1, new_edge_pos),
                        };

                        let new_pos = (
                            wrap_info.cube_size as isize * new_side_grid_position.0 + new_rel_pos.0,
                            wrap_info.cube_size as isize * new_side_grid_position.1 + new_rel_pos.1,
                        );

                        // get the new heading. this uses 0 for
                        let new_heading =
                            (local_edge_direction.neg().clockwise_rotations() + 4 - 1) % 4;

                        (new_pos, new_heading)
                    }
                    None => (
                        match direction {
                            (_, 0) => (
                                self.0[new_position.1 as usize].wrap_horizontal(new_position.0),
                                new_position.1,
                            ),
                            (0, dy) => self.wrap_vertical(new_position, -dy),
                            _ => unreachable!(),
                        },
                        location.heading,
                    ),
                };

                if self.is_open_tile(new_pos).unwrap() {
                    Some((new_pos, new_dir))
                } else {
                    None
                }
            }
        }
    }

    fn is_open_tile(&self, position: (isize, isize)) -> Option<bool> {
        let Some(line) = self.0.get(position.1 as usize) else {return None;};
        if line.0 > position.0 {
            return None;
        }
        let Some(&tile) = line.1.get((position.0 - line.0) as usize) else {return None;};

        Some(!tile)
    }

    fn wrap_vertical(&self, new_position: (isize, isize), dy: isize) -> (isize, isize) {
        // move one tile so we're not outside of bounds anymore, and another one so we're at the new position to check
        let mut new_position = (new_position.0, new_position.1 + dy + dy);
        while self.is_open_tile(new_position).is_some() {
            // move another tile
            new_position = (new_position.0, new_position.1 + dy);
        }

        // we've found an oob tile, take a step back so we're back in bounds
        new_position = (new_position.0, new_position.1 - dy);
        debug_assert!(self.is_open_tile(new_position).is_some());

        new_position
    }

    fn create_grid_map(&self, cube_size: usize) -> Vec<(CubeSideId, (isize, isize))> {
        let mut id_counter = 0;
        let mut map = vec![];

        for line_idx in (0..self.0.len()).step_by(cube_size) {
            let y = line_idx / cube_size;

            let line = &self.0[line_idx];
            let x_offset = (line.0 as usize) / cube_size;
            for x_index in 0..(line.1.len() / cube_size) {
                let x = x_offset + x_index;
                map.push((CubeSideId(id_counter), (x as isize, y as isize)));
                id_counter += 1;
            }
        }

        debug_assert_eq!(map.len(), 6);

        map
    }
}

#[derive(Debug)]
struct MazeLine(isize, Vec<bool>);

impl MazeLine {
    pub fn parse(line: &str) -> Self {
        let bytes = line.as_bytes();
        let prefix = bytes.iter().take_while(|b| **b == b' ').count();

        Self(
            prefix as isize,
            bytes[prefix..].iter().map(|b| *b == b'#').collect(),
        )
    }

    pub fn wrap_horizontal(&self, x: isize) -> isize {
        (x - self.0).rem_euclid(self.1.len() as isize) + self.0
    }
}

struct InstructionsIter<'a>(&'a [u8], usize);

impl<'a> InstructionsIter<'a> {
    pub fn from_str(string: &'a str) -> Self {
        Self(string.as_bytes(), 0)
    }
}

impl<'a> Iterator for InstructionsIter<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.len() {
            return None;
        }

        self.1 += 1;
        match self.0[self.1 - 1] {
            b'L' => Some(Instruction::RotateLeft),
            b'R' => Some(Instruction::RotateRight),
            d => {
                let mut total = (d - b'0') as usize;
                while self.1 < self.0.len() && self.0[self.1].is_ascii_digit() {
                    total *= 10;
                    total += (self.0[self.1] - b'0') as usize;
                    self.1 += 1;
                }
                Some(Instruction::Move(total))
            }
        }
    }
}

impl<'a> FusedIterator for InstructionsIter<'a> {}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    RotateLeft,
    RotateRight,
    Move(usize),
}

#[derive(Debug)]
struct Location {
    position: (isize, isize),
    /// 0 means east and counting up goes to south, west and north
    heading: usize,
}

impl Location {
    fn rotate_left(&mut self) {
        self.heading += 3;
        self.heading %= 4;
    }
    fn rotate_right(&mut self) {
        self.heading += 1;
        self.heading %= 4;
    }

    // TODO: use Direction2D instead
    fn get_direction(&self) -> (isize, isize) {
        match self.heading {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        }
    }

    fn get_solution(&self) -> usize {
        (self.position.0 + 1) as usize * 4 + (self.position.1 + 1) as usize * 1000 + self.heading
    }
}

struct CubeWrappingInfo {
    cube_size: usize,
    cube: Cube<CubeSideContent>,
    cube_side_id_map: Vec<(CubeSideId, (isize, isize))>,
}

#[derive(Debug, Default)]
struct Cube<T> {
    sides: [T; 6],
}

impl<T> Cube<T> {
    pub fn get_side(&self, side: CubeFace) -> &T {
        &self.sides[side.0 as usize]
    }
    pub fn get_side_mut(&mut self, side: CubeFace) -> &mut T {
        &mut self.sides[side.0 as usize]
    }
    pub fn index_to_face(index: usize) -> CubeFace {
        CubeFace(Direction3D::from_num(index))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An axis-aligned, directional plane
struct CubeFace(Direction3D);

impl CubeFace {
    /// Given a direction/axis, calculate what side of the cube a neighbour is (and in what direction it would face)
    pub fn calculate_neighbour(
        &self,
        facing: Direction3D,
        rotate_direction: Direction2D,
    ) -> (CubeFace, Direction3D) {
        // convert `offset` into an axis, and then into a neighbour
        let new_face = match rotate_direction {
            Direction2D::RIGHT => CubeFace(self.0.rotate_around_clockwise(facing)),
            Direction2D::LEFT => CubeFace(self.0.rotate_around_counterclockwise(facing)),
            Direction2D::TOP => CubeFace(facing),
            Direction2D::BOTTOM => CubeFace(facing.neg()),
        };

        let new_facing = match rotate_direction {
            Direction2D::LEFT | Direction2D::RIGHT => facing,
            Direction2D::TOP => self.0.neg(),
            Direction2D::BOTTOM => self.0,
        };

        (new_face, new_facing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CubeSideId(usize);

#[derive(Debug, Clone)]
struct CubeSideContent {
    id: CubeSideId,
    facing_direction: Direction3D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis3D {
    /// Left to right
    X,
    /// Bottom to top
    Y,
    /// Front to back
    Z,
}

impl Axis3D {
    /// Defines clockwise rotation around the positive X axis
    const ROTATION_X_POS: [Direction3D; 4] = [
        Direction3D::ZNeg,
        Direction3D::YNeg,
        Direction3D::ZPos,
        Direction3D::YPos,
    ];
    /// Defines clockwise rotation around the positive Y axis
    const ROTATION_Y_POS: [Direction3D; 4] = [
        Direction3D::XPos,
        Direction3D::ZPos,
        Direction3D::XNeg,
        Direction3D::ZNeg,
    ];
    /// Defines clockwise rotation around the positive Y axis
    const ROTATION_Z_POS: [Direction3D; 4] = [
        Direction3D::XPos,
        Direction3D::YNeg,
        Direction3D::XNeg,
        Direction3D::YPos,
    ];

    /// Gets rotation vectors around the axis, assuming clockwise rotation when pointing to the
    /// positive side.
    pub fn get_rotation(&self) -> &'static [Direction3D; 4] {
        match self {
            Axis3D::X => &Self::ROTATION_X_POS,
            Axis3D::Y => &Self::ROTATION_Y_POS,
            Axis3D::Z => &Self::ROTATION_Z_POS,
        }
    }
}

impl From<Direction3D> for Axis3D {
    fn from(value: Direction3D) -> Self {
        match value {
            Direction3D::XPos | Direction3D::XNeg => Axis3D::X,
            Direction3D::YPos | Direction3D::YNeg => Axis3D::Y,
            Direction3D::ZPos | Direction3D::ZNeg => Axis3D::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction3D {
    /// Right (width)
    XPos,
    /// Left
    XNeg,
    /// Up (height)
    YPos,
    /// Down
    YNeg,
    /// Forward (depth)
    ZPos,
    /// Down
    ZNeg,
}

impl Direction3D {
    #[allow(unused)]
    pub const TOP: Self = Self::YPos;
    #[allow(unused)]
    pub const BOTTOM: Self = Self::YNeg;
    #[allow(unused)]
    pub const LEFT: Self = Self::XNeg;
    #[allow(unused)]
    pub const RIGHT: Self = Self::XPos;
    #[allow(unused)]
    pub const FRONT: Self = Self::ZNeg;
    #[allow(unused)]
    pub const BACK: Self = Self::ZPos;

    pub fn from_num(num: usize) -> Self {
        match num {
            _ if num == Self::XPos as usize => Self::XPos,
            _ if num == Self::XNeg as usize => Self::XNeg,
            _ if num == Self::YPos as usize => Self::YPos,
            _ if num == Self::YNeg as usize => Self::YNeg,
            _ if num == Self::ZPos as usize => Self::ZPos,
            _ if num == Self::ZNeg as usize => Self::ZNeg,
            _ => panic!("Unexpected num: {num}"),
        }
    }

    pub fn neg(self) -> Self {
        match self {
            Direction3D::XPos => Direction3D::XNeg,
            Direction3D::XNeg => Direction3D::XPos,
            Direction3D::YPos => Direction3D::YNeg,
            Direction3D::YNeg => Direction3D::YPos,
            Direction3D::ZPos => Direction3D::ZNeg,
            Direction3D::ZNeg => Direction3D::ZPos,
        }
    }

    pub fn rotate_around_clockwise(self, center: Direction3D) -> Direction3D {
        self.rotate_around(center, 1)
    }

    pub fn rotate_around_counterclockwise(self, center: Direction3D) -> Direction3D {
        self.rotate_around(center, -1)
    }

    /// Rotate around another direction clockwise multiple times
    fn rotate_around(self, center: Direction3D, times: isize) -> Direction3D {
        let rotation = Axis3D::from(center).get_rotation();

        let Some(index) = rotation.iter().position(|x| *x == self) else {
            debug_assert_eq!(Axis3D::from(self), Axis3D::from(center));
            return self;
        };

        let mut index = index as isize;

        // rotate the given amount of times
        if center.is_negative() {
            index -= times;
        } else {
            index += times;
        }

        // fit index in array
        index = index.rem_euclid(rotation.len() as isize);

        rotation[index as usize]
    }

    fn is_negative(&self) -> bool {
        match self {
            Direction3D::XPos | Direction3D::YPos | Direction3D::ZPos => false,
            Direction3D::XNeg | Direction3D::YNeg | Direction3D::ZNeg => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction2D {
    /// Up (height)
    YPos = 0,
    /// Right (width)
    XPos = 1,
    /// Down
    YNeg = 2,
    /// Left
    XNeg = 3,
}

impl Direction2D {
    #[allow(unused)]
    pub const TOP: Self = Self::YPos;
    #[allow(unused)]
    pub const BOTTOM: Self = Self::YNeg;
    #[allow(unused)]
    pub const LEFT: Self = Self::XNeg;
    #[allow(unused)]
    pub const RIGHT: Self = Self::XPos;

    // TODO: implement IVec2 struct?
    #[allow(unused)]
    pub const fn to_unit_vector(self) -> (isize, isize) {
        match self {
            Direction2D::XPos => (1, 0),
            Direction2D::XNeg => (-1, 0),
            Direction2D::YPos => (0, 1),
            Direction2D::YNeg => (0, -1),
        }
    }

    pub fn from_unit_vector(unit: (isize, isize)) -> Self {
        match unit {
            (1, 0) => Direction2D::XPos,
            (-1, 0) => Direction2D::XNeg,
            (0, 1) => Direction2D::YPos,
            (0, -1) => Direction2D::YNeg,
            x => panic!("given vector {x:?} is not a unit vector"),
        }
    }

    pub fn neg(self) -> Self {
        match self {
            Direction2D::XPos => Direction2D::XNeg,
            Direction2D::XNeg => Direction2D::XPos,
            Direction2D::YPos => Direction2D::YNeg,
            Direction2D::YNeg => Direction2D::YPos,
        }
    }

    pub fn from_clockwise_rotations(rotations: usize) -> Self {
        match rotations {
            0 => Direction2D::YPos,
            1 => Direction2D::XPos,
            2 => Direction2D::YNeg,
            3 => Direction2D::XNeg,
            n => panic!("unexpected rotation count {n}"),
        }
    }

    /// Gets the amount of clockwise 90 degree rotations this direction has moved from "up".
    pub fn clockwise_rotations(&self) -> usize {
        match self {
            Direction2D::YPos => 0,
            Direction2D::XPos => 1,
            Direction2D::YNeg => 2,
            Direction2D::XNeg => 3,
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(6032, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(191010, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(5031, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(55364, output);
}

#[test]
fn test_cube_neighbour_calc_from_start_pos() {
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::TOP),
            Direction3D::ZPos,
            Direction2D::RIGHT
        ),
        (CubeFace(Direction3D::RIGHT), Direction3D::ZPos)
    );
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::TOP),
            Direction3D::ZPos,
            Direction2D::LEFT
        ),
        (CubeFace(Direction3D::LEFT), Direction3D::ZPos)
    );
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::TOP),
            Direction3D::ZPos,
            Direction2D::BOTTOM
        ),
        (CubeFace(Direction3D::FRONT), Direction3D::YPos)
    );
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::TOP),
            Direction3D::ZPos,
            Direction2D::TOP
        ),
        (CubeFace(Direction3D::BACK), Direction3D::YNeg)
    );
}

#[test]
fn test_cube_neighbour_calc_from_other_pos() {
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::FRONT),
            Direction3D::XNeg,
            Direction2D::RIGHT
        ),
        (CubeFace(Direction3D::TOP), Direction3D::XNeg)
    );
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::FRONT),
            Direction3D::XNeg,
            Direction2D::LEFT
        ),
        (CubeFace(Direction3D::BOTTOM), Direction3D::XNeg)
    );
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::FRONT),
            Direction3D::XNeg,
            Direction2D::BOTTOM
        ),
        (CubeFace(Direction3D::RIGHT), Direction3D::ZNeg)
    );
    assert_eq!(
        CubeFace::calculate_neighbour(
            &CubeFace(Direction3D::FRONT),
            Direction3D::XNeg,
            Direction2D::TOP
        ),
        (CubeFace(Direction3D::LEFT), Direction3D::ZPos)
    );
}
