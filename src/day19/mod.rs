use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 19;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = parse_input(input);

        input
            .iter()
            .map(get_quality::<24>)
            // .inspect(|quality| println!("quality: {}", quality))
            .enumerate()
            .map(|(i, v)| (i + 1) * v)
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let input = parse_input(input);

        input
            .iter()
            .take(3)
            .map(get_quality::<32>)
            // .inspect(|quality| println!("quality: {}", quality))
            .product()
    }
}

fn get_quality<const MAX_TIME: usize>(costs: &Blueprint) -> usize {
    get_quality_recursive::<MAX_TIME>(costs, State::new())
}

fn get_quality_recursive<const MAX_TIME: usize>(blueprint: &Blueprint, state: State) -> usize {
    // NOTE: a lot of the optimizations here are not needed/not effective
    if state.time > MAX_TIME {
        return 0;
    }
    if state.time == MAX_TIME {
        return state.geodes;
    }

    let mut best_outcome = 0;

    let time_left = MAX_TIME - state.time;

    let can_build_ore_robot = blueprint.ore_robot <= state.ore;
    let can_build_clay_robot = blueprint.clay_robot <= state.ore;
    let can_build_obsidian_robot =
        blueprint.obsidian_robot.0 <= state.ore && blueprint.obsidian_robot.1 <= state.clay;
    let can_build_geode_robot =
        blueprint.geode_robot.0 <= state.ore && blueprint.geode_robot.1 <= state.obsidian;

    let have_enough_ore_gen = state.ore_robots >= blueprint.max_qty_needed(ResourceType::Ore);
    let have_enough_clay_gen = state.clay_robots >= blueprint.max_qty_needed(ResourceType::Clay);
    let have_enough_obsidian_gen =
        state.obsidian_robots >= blueprint.max_qty_needed(ResourceType::Obsidian);

    let can_keep_rebuying_ore_robot = state.ore_robots >= blueprint.ore_robot;
    let can_keep_rebuying_clay_robot = state.ore_robots >= blueprint.clay_robot;
    let can_keep_rebuying_obsidian_robot = state.ore_robots >= blueprint.obsidian_robot.0
        && state.clay_robots >= blueprint.obsidian_robot.1;
    let can_keep_rebuying_geode_robot = state.ore_robots >= blueprint.geode_robot.0
        && state.obsidian_robots >= blueprint.geode_robot.1;

    let have_enough_ore = state.ore > blueprint.max_qty_needed(ResourceType::Ore) * time_left;
    let have_enough_clay = state.clay > blueprint.max_qty_needed(ResourceType::Clay) * time_left;
    let have_enough_obsidian =
        state.obsidian > blueprint.max_qty_needed(ResourceType::Obsidian) * time_left;

    // consider waiting to buy each type of robot

    // Geode
    if state.obsidian_robots > 0 && state.time < (MAX_TIME - 1) {
        let mut state = state.clone();

        while !state.can_buy(ResourceType::Geode, blueprint) {
            state.tick();
        }

        state.buy(ResourceType::Geode, blueprint);
        state.tick();

        best_outcome = best_outcome.max(get_quality_recursive::<MAX_TIME>(blueprint, state));
    }

    // no need to check other options if we can just keep rebuying the geode robot
    if can_keep_rebuying_geode_robot {
        return best_outcome;
    }

    // Obsidian
    if state.clay_robots > 0 && !have_enough_obsidian_gen && state.time < (MAX_TIME - 2) {
        let mut state = state.clone();

        while !state.can_buy(ResourceType::Obsidian, blueprint) {
            state.tick();
        }

        state.buy(ResourceType::Obsidian, blueprint);
        state.tick();

        best_outcome = best_outcome.max(get_quality_recursive::<MAX_TIME>(blueprint, state));
    }

    // don't bother waiting or buying other stuff if we're close to geode buy loop
    if can_keep_rebuying_obsidian_robot
        && !have_enough_obsidian_gen
        && state.ore_robots >= blueprint.geode_robot.0
    {
        return best_outcome;
    }

    // Clay
    if !have_enough_clay_gen && state.time < (MAX_TIME - 3) {
        let mut state = state.clone();

        while !state.can_buy(ResourceType::Clay, blueprint) {
            state.tick();
        }

        state.buy(ResourceType::Clay, blueprint);
        state.tick();

        best_outcome = best_outcome.max(get_quality_recursive::<MAX_TIME>(blueprint, state));
    }

    // Ore
    if !have_enough_ore_gen && state.time < (MAX_TIME - 2) {
        let mut state = state.clone();

        while !state.can_buy(ResourceType::Ore, blueprint) {
            state.tick();
        }

        state.buy(ResourceType::Ore, blueprint);
        state.tick();

        best_outcome = best_outcome.max(get_quality_recursive::<MAX_TIME>(blueprint, state));
    }

    // consider doing nothing
    let mut should_not_wait = false;

    // if we need 1 more resource and we can keep buying its robot, don't wait
    should_not_wait |= !have_enough_ore_gen
        && (can_keep_rebuying_ore_robot && have_enough_clay && have_enough_obsidian);
    should_not_wait |= !have_enough_clay_gen
        && (can_keep_rebuying_clay_robot && have_enough_ore && have_enough_obsidian);
    should_not_wait |= !have_enough_obsidian_gen
        && (can_keep_rebuying_obsidian_robot && have_enough_ore && have_enough_clay);

    // we have enough resources to buy anything that is available (using ore)
    let should_buy_with_ore =
        (can_build_ore_robot && can_build_clay_robot) && state.clay_robots == 0;
    should_not_wait |= should_buy_with_ore;

    // we have enough resources to buy anything that is available (using ore or clay)
    let should_buy_with_ore_or_clay =
        (can_build_ore_robot && can_build_clay_robot && can_build_obsidian_robot)
            && state.obsidian_robots == 0;
    should_not_wait |= should_buy_with_ore_or_clay;

    // we have enough resources to buy anything that is available (using ore or clay or geode)
    let should_buy_with_ore_or_clay_or_obsidian = (can_build_ore_robot
        && can_build_clay_robot
        && can_build_obsidian_robot
        && can_build_geode_robot)
        && state.geode_robots == 0;
    should_not_wait |= should_buy_with_ore_or_clay_or_obsidian;

    // wait until the end. only useful if we have geode robots
    if !should_not_wait && state.geode_robots != 0 {
        let mut state = state;
        while state.time < MAX_TIME {
            state.tick();
        }
        best_outcome =
            best_outcome.max(get_quality_recursive::<MAX_TIME>(blueprint, state.clone()));
    }

    best_outcome
}

#[derive(Clone, Default)]
struct State {
    time: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,

    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,

    pending_robot_build: Option<ResourceType>,
}

impl State {
    fn new() -> Self {
        Self {
            time: 0,
            ore_robots: 1,
            ..Default::default()
        }
    }

    fn tick(&mut self) {
        self.time += 1;

        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;

        // build happens after calculating gains
        match self.pending_robot_build.take() {
            Some(ResourceType::Ore) => self.ore_robots += 1,
            Some(ResourceType::Clay) => self.clay_robots += 1,
            Some(ResourceType::Obsidian) => self.obsidian_robots += 1,
            Some(ResourceType::Geode) => self.geode_robots += 1,
            None => (),
        }
    }

    fn start_build(&mut self, robot_type: ResourceType) {
        debug_assert!(self.pending_robot_build.is_none());
        self.pending_robot_build = Some(robot_type);
    }

    const fn can_buy(&self, ore_type: ResourceType, blueprint: &Blueprint) -> bool {
        match ore_type {
            ResourceType::Ore => blueprint.ore_robot <= self.ore,
            ResourceType::Clay => blueprint.clay_robot <= self.ore,
            ResourceType::Obsidian => {
                blueprint.obsidian_robot.0 <= self.ore && blueprint.obsidian_robot.1 <= self.clay
            }
            ResourceType::Geode => {
                blueprint.geode_robot.0 <= self.ore && blueprint.geode_robot.1 <= self.obsidian
            }
        }
    }

    fn buy(&mut self, geode: ResourceType, blueprint: &Blueprint) {
        match geode {
            ResourceType::Ore => {
                self.ore -= blueprint.ore_robot;
                self.start_build(ResourceType::Ore);
            }
            ResourceType::Clay => {
                self.ore -= blueprint.clay_robot;
                self.start_build(ResourceType::Clay);
            }
            ResourceType::Obsidian => {
                self.ore -= blueprint.obsidian_robot.0;
                self.clay -= blueprint.obsidian_robot.1;
                self.start_build(ResourceType::Obsidian);
            }
            ResourceType::Geode => {
                self.ore -= blueprint.geode_robot.0;
                self.obsidian -= blueprint.geode_robot.1;
                self.start_build(ResourceType::Geode);
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" costs ");
            split.next().expect("first part");

            let ore_cost = split.next().expect("ore cost");
            let ore = fast_parse_int(ore_cost.split_once(' ').unwrap().0);

            let clay_cost = split.next().expect("clay cost");
            let clay = fast_parse_int(clay_cost.split_once(' ').unwrap().0);

            let obs_cost = split.next().expect("obs cost");
            let (obs_cost_1, obs_cost_2) = obs_cost.split_once(' ').unwrap();
            let obs_1 = fast_parse_int(obs_cost_1);
            let obs_2 = fast_parse_int(obs_cost_2["ore and ".len()..].split_once(' ').unwrap().0);

            let geode_cost = split.next().expect("geode cost");
            let (geode_cost_1, geode_cost_2) = geode_cost.split_once(' ').unwrap();
            let geode_1 = fast_parse_int(geode_cost_1);
            let geode_2 =
                fast_parse_int(geode_cost_2["ore and ".len()..].split_once(' ').unwrap().0);

            Blueprint {
                ore_robot: ore,
                clay_robot: clay,
                obsidian_robot: (obs_1, obs_2),
                geode_robot: (geode_1, geode_2),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Blueprint {
    /// Price in ore
    ore_robot: usize,
    /// Price in ore
    clay_robot: usize,
    /// Price in ore and clay
    obsidian_robot: (usize, usize),
    /// Price in ore and obsidian
    geode_robot: (usize, usize),
}

impl Blueprint {
    pub fn max_qty_needed(&self, ore_type: ResourceType) -> usize {
        match ore_type {
            ResourceType::Ore => self
                .ore_robot
                .max(self.clay_robot)
                .max(self.obsidian_robot.0)
                .max(self.geode_robot.0),
            ResourceType::Clay => self.obsidian_robot.1,
            ResourceType::Obsidian => self.geode_robot.1,
            ResourceType::Geode => unreachable!(),
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(33, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1981, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(56 * 62, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(10962, output);
}

#[test]
fn simulate_example_run() {
    let blueprint = parse_input(Day::INPUT_SAMPLE)[0].clone();
    let mut state = State::new();

    debug_assert_eq!(state.time, 0);
    debug_assert_eq!(state.ore, 0);

    state.tick();
    debug_assert_eq!(state.time, 1);
    debug_assert_eq!(state.ore, 1);

    state.tick();
    debug_assert_eq!(state.time, 2);
    debug_assert_eq!(state.ore, 2);

    state.ore -= blueprint.clay_robot;
    state.start_build(ResourceType::Clay);
    debug_assert_eq!(state.clay_robots, 0);

    state.tick(); // this tick buys the robot
    debug_assert_eq!(state.time, 3);
    debug_assert_eq!(state.ore, 1);
    debug_assert_eq!(state.clay, 0);
    debug_assert_eq!(state.clay_robots, 1);

    state.tick(); // in this tick, the robot is first active
    debug_assert_eq!(state.time, 4);
    debug_assert_eq!(state.ore, 2);
    debug_assert_eq!(state.clay, 1);
}
