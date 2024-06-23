use std::collections::BTreeSet;

#[allow(unused_variables)]
pub fn p1(input: &str) -> u64 {
    let universe = Universe::parse(input);
    universe.all_distances_expanded(1) as u64
}

#[allow(unused_variables)]
pub fn p2(input: &str) -> u64 {
    let universe = Universe::parse(input);
    universe.all_distances_expanded(999_999) as u64
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Star {
    x: usize,
    y: usize,
}
#[derive(PartialEq, Eq, Debug, Clone)]
struct Universe {
    stars: Vec<Star>,
    empty_columns: BTreeSet<usize>,
    empty_rows: BTreeSet<usize>,
}

impl Universe {
    /// Calculate the position of a star in the expanded universe.
    /// The factor is the space added for every empty column and/or row.
    fn expand(&self, star: &Star, factor: usize) -> Star {
        let x = self.empty_columns.iter().filter(|&sx| *sx < star.x).count()
            * factor
            + star.x;
        let y =
            self.empty_rows.iter().filter(|&sy| *sy < star.y).count() * factor + star.y;
        Star { x, y }
    }

    /// Calculate the distance between two stars.
    fn distance(star1: &Star, star2: &Star) -> usize {
        star1.x.abs_diff(star2.x) + star1.y.abs_diff(star2.y)
    }

    /// Calculate the sum of all distances between each pair of stars.
    fn all_distances(stars: &[Star]) -> usize {
        let mut distance_sum = 0;
        for (index, star) in stars.iter().enumerate() {
            distance_sum += stars[index..]
                .iter()
                .fold(0, |acc, s| acc + Self::distance(star, s))
        }
        distance_sum
    }

    /// Calculate the sum of all distances between each pair of stars
    /// in the expanded universe.
    fn all_distances_expanded(&self, factor: usize) -> usize {
        let expanded_stars = self.stars.iter().map(|s| self.expand(s, factor)).collect::<Vec<_>>();
        Self::all_distances(&expanded_stars)
    }

    /// Find and add stars from this line, update empty_columns
    /// return true if the line had any stars in it.
    fn find_stars_in_line(&mut self, line: &str, y: usize) -> bool {
        let mut positions = line
            .match_indices('#')
            .map(|(x, _)| {
                self.empty_columns.remove(&x);
                Star { x, y }
            });
        let stars_before  = self.stars.len();
        self.stars.extend(&mut positions);
        self.stars.len() > stars_before
    }

    /// Create the Universe.
    fn parse(input: &str) -> Self {
        let mut input = input.split('\n');
        let first_line = input.next().unwrap_or_else(|| panic!("No universe given"));
        let mut line_counter = 0;
        let mut u = Universe {
            stars: vec![],
            empty_columns: BTreeSet::from_iter(0..first_line.len()),
            empty_rows: BTreeSet::new(),
        };
        if !u.find_stars_in_line(first_line, line_counter) {
            // this row is empty
            u.empty_rows.insert(line_counter);
        }
        for line in input {
            line_counter += 1;
            if !u.find_stars_in_line(line, line_counter) {
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
            stars: vec![],
            empty_columns: BTreeSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: BTreeSet::from_iter(vec![3, 7].into_iter()),
        };
        let star = Star { x: 3, y: 0 };
        let expanded_star = u.expand(&star, 1);
        assert_eq!(expanded_star, Star { x: 4, y: 0 });
    }

    #[test]
    fn test_distance() {
        let u = Universe {
            stars: vec![
                Star { x: 3, y: 0 },
                Star { x: 7, y: 1 },
                Star { x: 0, y: 2 },
                Star { x: 6, y: 4 },
                Star { x: 1, y: 5 },
                Star { x: 9, y: 6 },
                Star { x: 7, y: 8 },
                Star { x: 0, y: 9 },
                Star { x: 4, y: 9 },
            ],
            empty_columns: BTreeSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: BTreeSet::from_iter(vec![3, 7].into_iter()),
        };
        assert_eq!(
            Universe::distance(
                &u.expand(&u.stars[4], 1),
                &u.expand(&u.stars[8], 1)
            ),
            9
        );
        assert_eq!(
            Universe::distance(
                &u.expand(&u.stars[0], 1),
                &u.expand(&u.stars[6], 1)
            ),
            15
        );
        assert_eq!(
            Universe::distance(
                &u.expand(&u.stars[2], 1),
                &u.expand(&u.stars[5], 1)
            ),
            17
        );
        assert_eq!(
            Universe::distance(
                &u.expand(&u.stars[7], 1),
                &u.expand(&u.stars[8], 1)
            ),
            5
        );
    }

    #[test]
    fn test_all_distances() {
        let u = Universe {
            stars: vec![
                Star { x: 3, y: 0 },
                Star { x: 7, y: 1 },
                Star { x: 0, y: 2 },
                Star { x: 6, y: 4 },
                Star { x: 1, y: 5 },
                Star { x: 9, y: 6 },
                Star { x: 7, y: 8 },
                Star { x: 0, y: 9 },
                Star { x: 4, y: 9 },
            ],
            empty_columns: BTreeSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: BTreeSet::from_iter(vec![3, 7].into_iter()),
        };
        let expanded_stars =
            u.stars.iter().map(|s| u.expand(s, 1)).collect::<Vec<_>>();
        let result = Universe::all_distances(&expanded_stars);
        assert_eq!(result, 374)
    }

    #[test]
    fn test_find_stars_in_line() {
        let mut u = Universe {
            stars: vec![],
            empty_columns: BTreeSet::from_iter(0..10),
            empty_rows: BTreeSet::from_iter(0..10),
        };

        let result = u.find_stars_in_line("#...#.....", 9);
        assert_eq!(result, true);
        assert_eq!(u.stars, vec![Star { x: 0, y: 9 }, Star { x: 4, y: 9 }]);
        assert_eq!(
            u.empty_columns,
            BTreeSet::from_iter(vec![1, 2, 3, 5, 6, 7, 8, 9].into_iter())
        )
    }

    #[test]
    fn test_parse() {
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
        let result = Universe::parse(input);
        let u = Universe {
            stars: vec![
                Star { x: 3, y: 0 },
                Star { x: 7, y: 1 },
                Star { x: 0, y: 2 },
                Star { x: 6, y: 4 },
                Star { x: 1, y: 5 },
                Star { x: 9, y: 6 },
                Star { x: 7, y: 8 },
                Star { x: 0, y: 9 },
                Star { x: 4, y: 9 },
            ],
            empty_columns: BTreeSet::from_iter(vec![2, 5, 8].into_iter()),
            empty_rows: BTreeSet::from_iter(vec![3, 7].into_iter()),
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

        let universe = Universe::parse(input);
        let result =
            universe.all_distances_expanded(99) as u64;
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
