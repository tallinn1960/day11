pub mod uncle_scientist;
#[cfg(feature = "Swift")]
pub mod swift;

use std::collections::HashSet;

pub fn p1(input: &str) -> u64 {
    let universe = Universe::big_bang(input, 1);
    universe.sum_of_all_distances_after_expansion() as u64
}

pub fn p2(input: &str) -> u64 {
    let universe = Universe::big_bang(input, 999_999);
    universe.sum_of_all_distances_after_expansion() as u64
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
    extra_space: usize,
}

impl Universe {
    /// Calculate the new position of a galaxy in the expanded universe.
    /// Returns the galaxy with its new position.
    fn move_by_expansion(&self, galaxy: &Galaxy) -> Galaxy {
        let x = self
            .empty_columns
            .iter()
            .filter(|&sx| *sx < galaxy.x)
            .count()
            * self.extra_space
            + galaxy.x;
        let y = self.empty_rows.iter().filter(|&sy| *sy < galaxy.y).count()
            * self.extra_space
            + galaxy.y;
        Galaxy { x, y }
    }

    /// Calculate the sum of the distances between each pair of galaxies
    /// in the expanded universe.
    fn sum_of_all_distances_after_expansion(&self) -> usize {
        // Expand the universe.
        let expanded_galaxies = self
            .galaxies
            .iter()
            .map(|galaxy: &Galaxy| self.move_by_expansion(galaxy))
            .collect::<Vec<_>>();
        // Calculate and sum over the distances of all galaxy pairs.
        expanded_galaxies
            .iter()
            .enumerate()
            .flat_map(|(index, galaxy)| {
                expanded_galaxies[index..]
                    .iter()
                    .map(|other_galaxy| galaxy.distance(other_galaxy))
            })
            .sum()
    }

    /// Find and add galaxies from this line, update empty_columns,
    /// return true if the line had any galaxies in it.
    fn find_galaxies_in_line(&mut self, line: &str, y: usize) -> bool {
        let mut positions = line
            .match_indices('#')
            .inspect(|(x, _)| {
                self.empty_columns.remove(x);
            })
            .map(|(x, _)| Galaxy { x, y })
            .peekable();
        let found = positions.peek().is_some();
        self.galaxies.extend(&mut positions);
        found
    }

    /// Create the Universe. It all started with a Big Bang.
    /// 
    /// the_word: In the beginning was the Word, and the Word was with God, and the Word was God. 
    /// He was with God in the beginning. Through Him all things were made, and without 
    /// Him nothing was made that has been made.
    /// 
    /// extra_space: the mess we have today.
    fn big_bang(the_word: &str, extra_space: usize) -> Self {
        let mut input = the_word.split('\n');
        let first_line =
            input.next().unwrap_or_else(|| panic!("Can't create the universe without the Word!"));
        let mut line_counter = 0;
        let mut universe = Universe {
            galaxies: vec![],
            empty_columns: HashSet::from_iter(0..first_line.len()),
            empty_rows: HashSet::new(),
            extra_space,
        };
        if !universe.find_galaxies_in_line(first_line, line_counter) {
            // this row is empty
            universe.empty_rows.insert(line_counter);
        }
        for line in input {
            line_counter += 1;
            if !universe.find_galaxies_in_line(line, line_counter) {
                universe.empty_rows.insert(line_counter);
            }
        }
        universe // There you have it. Use it wisely.
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
            extra_space: 1,
        };
        let galaxy = Galaxy { x: 3, y: 0 };
        let expanded_galaxy = u.move_by_expansion(&galaxy);
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
            extra_space: 1,
        };
        assert_eq!(
            u.move_by_expansion(&u.galaxies[4])
                .distance(&u.move_by_expansion(&u.galaxies[8])),
            9
        );
        assert_eq!(
            u.move_by_expansion(&u.galaxies[0])
                .distance(&u.move_by_expansion(&u.galaxies[6])),
            15
        );
        assert_eq!(
            u.move_by_expansion(&u.galaxies[2])
                .distance(&u.move_by_expansion(&u.galaxies[5])),
            17
        );
        assert_eq!(
            u.move_by_expansion(&u.galaxies[7])
                .distance(&u.move_by_expansion(&u.galaxies[8])),
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
            extra_space: 1,
        };
        let result = u.sum_of_all_distances_after_expansion();
        assert_eq!(result, 374)
    }

    #[test]
    fn test_find_galaxies_in_line() {
        let mut u = Universe {
            galaxies: vec![],
            empty_columns: HashSet::from_iter(0..10),
            empty_rows: HashSet::from_iter(0..10),
            extra_space: 1,
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
        let result = Universe::big_bang(input, 1);
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
            extra_space: 1,
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

        let universe = Universe::big_bang(input, 99);
        let result = universe.sum_of_all_distances_after_expansion() as u64;
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
