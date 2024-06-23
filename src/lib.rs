pub mod uncle_scientist;

use std::collections::HashSet;

#[allow(unused_variables)]
pub fn p1(input: &str) -> u64 {
    let universe = Universe::big_bang(input);
    universe.all_distances_expanded(1) as u64
}

#[allow(unused_variables)]
pub fn p2(input: &str) -> u64 {
    let universe = Universe::big_bang(input);
    universe.all_distances_expanded(999_999) as u64
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    /// Calculate the distance between two galaxies.
    fn distance(&self, other_galaxy: &Galaxy) -> usize {
        self.x.abs_diff(other_galaxy.x) + self.y.abs_diff(other_galaxy.y)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Universe {
    galaxies: Vec<Galaxy>,
    empty_columns: HashSet<usize>,
    empty_rows: HashSet<usize>,
}

impl Universe {
    /// Calculate the position of a galaxy in the expanded universe.
    /// The factor is the space added for every empty column and/or row.
    fn expand(&self, galaxy: &Galaxy, factor: usize) -> Galaxy {
        let x = self
            .empty_columns
            .iter()
            .filter(|&sx| *sx < galaxy.x)
            .count()
            * factor
            + galaxy.x;
        let y = self.empty_rows.iter().filter(|&sy| *sy < galaxy.y).count()
            * factor
            + galaxy.y;
        Galaxy { x, y }
    }

    /// Calculate the sum of all distances between each pair of galaxies
    /// in the expanded universe.
    fn all_distances_expanded(&self, factor: usize) -> usize {
        // Expand the universe.
        let expanded_galaxies = self
            .galaxies
            .iter()
            .map(|galaxy: &Galaxy| self.expand(galaxy, factor))
            .collect::<Vec<_>>();
        // Calculate and sum over the distances of all galaxy pairs.
        expanded_galaxies
            .iter()
            .enumerate()
            .fold(0, |acc, (index, galaxy)| {
                acc + expanded_galaxies[index..]
                    .iter()
                    .fold(0, |acc, other_galaxy| {
                        acc + galaxy.distance(other_galaxy)
                    })
            })
    }

    /// Find and add galaxies from this line, update empty_columns
    /// return true if the line had any galaxies in it.
    fn find_galaxies_in_line(&mut self, line: &str, y: usize) -> bool {
        let mut positions = line.match_indices('#').map(|(x, _)| {
            self.empty_columns.remove(&x);
            Galaxy { x, y }
        });
        // It's stupid that match_indices gives no indication
        // on whether a match was found at all. So we need
        // this hack to see if we found something.
        let previous_galaxy_count = self.galaxies.len();
        self.galaxies.extend(&mut positions);
        self.galaxies.len() > previous_galaxy_count
    }

    /// Create the Universe.
    fn big_bang(input: &str) -> Self {
        let mut input = input.split('\n');
        let first_line =
            input.next().unwrap_or_else(|| panic!("No universe given"));
        let mut line_counter = 0;
        let mut u = Universe {
            galaxies: vec![],
            empty_columns: HashSet::from_iter(0..first_line.len()),
            empty_rows: HashSet::new(),
        };
        if !u.find_galaxies_in_line(first_line, line_counter) {
            // this row is empty
            u.empty_rows.insert(line_counter);
        }
        for line in input {
            line_counter += 1;
            if !u.find_galaxies_in_line(line, line_counter) {
                u.empty_rows.insert(line_counter);
            }
        }

        u
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_expand() {
        let u = Universe {
            galaxies: vec![],
            empty_columns: HashSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: HashSet::from_iter(vec![3, 7].into_iter()),
        };
        let galaxy = Galaxy { x: 3, y: 0 };
        let expanded_galaxy = u.expand(&galaxy, 1);
        assert_eq!(expanded_galaxy, Galaxy { x: 4, y: 0 });
    }

    #[test]
    fn test_distance() {
        let u = Universe {
            galaxies: vec![
                Galaxy { x: 3, y: 0 },
                Galaxy { x: 7, y: 1 },
                Galaxy { x: 0, y: 2 },
                Galaxy { x: 6, y: 4 },
                Galaxy { x: 1, y: 5 },
                Galaxy { x: 9, y: 6 },
                Galaxy { x: 7, y: 8 },
                Galaxy { x: 0, y: 9 },
                Galaxy { x: 4, y: 9 },
            ],
            empty_columns: HashSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: HashSet::from_iter(vec![3, 7].into_iter()),
        };
        assert_eq!(
            u.expand(&u.galaxies[4], 1)
                .distance(&u.expand(&u.galaxies[8], 1)),
            9
        );
        assert_eq!(
            u.expand(&u.galaxies[0], 1)
                .distance(&u.expand(&u.galaxies[6], 1)),
            15
        );
        assert_eq!(
            u.expand(&u.galaxies[2], 1)
                .distance(&u.expand(&u.galaxies[5], 1)),
            17
        );
        assert_eq!(
            u.expand(&u.galaxies[7], 1)
                .distance(&u.expand(&u.galaxies[8], 1)),
            5
        );
    }

    #[test]
    fn test_all_distances_expanded() {
        let u = Universe {
            galaxies: vec![
                Galaxy { x: 3, y: 0 },
                Galaxy { x: 7, y: 1 },
                Galaxy { x: 0, y: 2 },
                Galaxy { x: 6, y: 4 },
                Galaxy { x: 1, y: 5 },
                Galaxy { x: 9, y: 6 },
                Galaxy { x: 7, y: 8 },
                Galaxy { x: 0, y: 9 },
                Galaxy { x: 4, y: 9 },
            ],
            empty_columns: HashSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: HashSet::from_iter(vec![3, 7].into_iter()),
        };
        let result = u.all_distances_expanded(1);
        assert_eq!(result, 374)
    }

    #[test]
    fn test_find_galaxies_in_line() {
        let mut u = Universe {
            galaxies: vec![],
            empty_columns: HashSet::from_iter(0..10),
            empty_rows: HashSet::from_iter(0..10),
        };

        let result = u.find_galaxies_in_line("#...#.....", 9);
        assert_eq!(result, true);
        assert_eq!(
            u.galaxies,
            vec![Galaxy { x: 0, y: 9 }, Galaxy { x: 4, y: 9 }]
        );
        assert_eq!(
            u.empty_columns,
            HashSet::from_iter(vec![1, 2, 3, 5, 6, 7, 8, 9].into_iter())
        )
    }

    #[test]
    fn test_big_bang() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = Universe::big_bang(input);
        let u = Universe {
            galaxies: vec![
                Galaxy { x: 3, y: 0 },
                Galaxy { x: 7, y: 1 },
                Galaxy { x: 0, y: 2 },
                Galaxy { x: 6, y: 4 },
                Galaxy { x: 1, y: 5 },
                Galaxy { x: 9, y: 6 },
                Galaxy { x: 7, y: 8 },
                Galaxy { x: 0, y: 9 },
                Galaxy { x: 4, y: 9 },
            ],
            empty_columns: HashSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: HashSet::from_iter(vec![3, 7].into_iter()),
        };
        assert_eq!(result, u);
    }

    #[test]
    fn test_p1_sample() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = p1(input);
        assert_eq!(result, 374)
    }

    #[test]
    fn test_p2_sample() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let universe = Universe::big_bang(input);
        let result = universe.all_distances_expanded(99) as u64;
        assert_eq!(result, 8410)
    }

    #[test]
    fn test_part1() {
        let mut f = File::open("input.txt").expect("Can't open input file!");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p1(&buf);
        assert_eq!(result, 9509330)
    }

    #[test]
    fn test_part2() {
        let mut f = File::open("input.txt").expect("Can't open input file!");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p2(&buf);
        assert_eq!(result, 635832237682)
    }
}
