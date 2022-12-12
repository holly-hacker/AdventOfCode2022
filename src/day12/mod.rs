use std::{
    collections::VecDeque,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

use ahash::HashSet;
use tinyvec::ArrayVec;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 12;

    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        // TODO: can improve this massively, but I hate pathfinding
        // 1. use midpoints for unique chars
        let stride = input.lines().next().unwrap().len();
        let vec: Vec<_> = input
            .as_bytes()
            .iter()
            .flat_map(|f| match f {
                b'a'..=b'z' => Some((f - b'a') as i8),
                b'S' => Some(-1),
                b'E' => Some(26),
                _ => None,
            })
            .collect();
        let height = vec.len() / stride;
        debug_assert!(vec.len() % stride == 0);

        let start = vec.iter().position(|x| *x == -1).unwrap();
        let end = vec.iter().position(|x| *x == (26)).unwrap();

        let path = bfs(start, end, |i| {
            let mut ret = ArrayVec::<[usize; 4]>::default();
            let x = i % stride;
            let y = i / stride;

            if x > 0 {
                if vec[i - 1] - vec[i] <= 1 {
                    ret.push(i - 1);
                }
            }
            if x < (stride - 1) {
                if vec[i + 1] - vec[i] <= 1 {
                    ret.push(i + 1);
                }
            }
            if y > 0 {
                if vec[i - stride] - vec[i] <= 1 {
                    ret.push(i - stride);
                }
            }
            if y < (height - 1) {
                if vec[i + stride] - vec[i] <= 1 {
                    ret.push(i + stride);
                }
            }

            ret
        });

        let unwrapped = path.unwrap();
        unwrapped.depth() - 1
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        // TODO: can improve this massively, but I hate pathfinding
        // 1. search in reverse rather than brute forcing
        // 2. early exit if a path is longer than shortest found path, if any (not needed if 1)
        // 3. use midpoints for unique chars
        let stride = input.lines().next().unwrap().len();
        let vec: Vec<_> = input
            .as_bytes()
            .iter()
            .flat_map(|f| match f {
                b'a'..=b'z' => Some((f - b'a') as i8),
                b'S' => Some(0), // same as 'a'
                b'E' => Some(26),
                _ => None,
            })
            .collect();
        let height = vec.len() / stride;
        debug_assert!(vec.len() % stride == 0);

        let starts: Vec<_> = vec
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == 0)
            .map(|(i, _)| i)
            .collect();

        let end = vec.iter().position(|x| *x == (26)).unwrap();

        starts
            .iter()
            .flat_map(|start| {
                bfs(*start, end, |i| {
                    let mut ret = ArrayVec::<[usize; 4]>::default();
                    let x = i % stride;
                    let y = i / stride;

                    if x > 0 {
                        if vec[i - 1] - vec[i] <= 1 {
                            ret.push(i - 1);
                        }
                    }
                    if x < (stride - 1) {
                        if vec[i + 1] - vec[i] <= 1 {
                            ret.push(i + 1);
                        }
                    }
                    if y > 0 {
                        if vec[i - stride] - vec[i] <= 1 {
                            ret.push(i - stride);
                        }
                    }
                    if y < (height - 1) {
                        if vec[i + stride] - vec[i] <= 1 {
                            ret.push(i + stride);
                        }
                    }

                    ret
                })
            })
            .map(|x| x.depth())
            .min()
            .unwrap()
            - 1
    }
}

// Locke @ https://stackoverflow.com/a/71190546, CC BY-SA 4.0
fn bfs<F, R>(start: usize, end: usize, expand: F) -> Option<SearchPath>
where
    F: Fn(usize) -> R,
    R: IntoIterator<Item = usize>,
{
    let mut visited = HashSet::default();
    let mut queue = VecDeque::new();

    queue.push_back(SearchPath(start, None));
    visited.insert(start);

    while let Some(SearchPath(node, path)) = queue.pop_front() {
        if node == end {
            return Some(SearchPath(node, path));
        }

        let path = Rc::new(SearchPath(node, path.clone()));

        for edge in expand(node) {
            if !visited.contains(&edge) {
                visited.insert(edge);
                queue.push_back(SearchPath(edge, Some(path.clone())));
            }
        }
    }

    None
}

#[derive(Clone, PartialEq, Eq)]
pub struct SearchPath(usize, Option<Rc<SearchPath>>);

impl SearchPath {
    fn depth(&self) -> usize {
        1 + match &self.1 {
            Some(x) => x.depth(),
            None => 0,
        }
    }
}

impl Debug for SearchPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.1 {
            Some(v) => write!(f, "{:?} -> {:?}", v, &self.0),
            None => write!(f, "{:?}", &self.0),
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(31, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(370, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(29, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(363, output);
}
