#[derive(Default)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    row_offsets: Vec<usize>,
    col_offsets: Vec<usize>,
}

impl Universe {
    fn parse(&mut self, input: &str) {
        let lines = input.split('\n').collect::<Vec<_>>();

        let mut row_offset = 0;
        let mut col_has_galaxy = vec![false; lines[0].len()];

        for (row, line) in lines.iter().enumerate() {
            let mut galaxy_spotted = false;
            for (col, ch) in line.char_indices() {
                if ch == '#' {
                    self.galaxies.push((row, col));
                    galaxy_spotted = true;
                    col_has_galaxy[col] = true;
                }
            }
            if !galaxy_spotted {
                row_offset += 1;
            }
            self.row_offsets.push(row_offset);
        }
        let mut col_offset = 0;
        for col in col_has_galaxy {
            if !col {
                col_offset += 1;
            }
            self.col_offsets.push(col_offset)
        }
    }

    fn sum_distances(&mut self, scale: usize) -> u64 {
        let scale = scale - 1;
        self.galaxies
            .iter()
            .enumerate()
            .map(|(idx, first)| {
                let (irow, icol) = (
                    first.0 + self.row_offsets[first.0] * scale,
                    first.1 + self.col_offsets[first.1] * scale,
                );
                self.galaxies
                    .iter()
                    .skip(idx)
                    .map(|second| {
                        let (jrow, jcol) = (
                            second.0 + self.row_offsets[second.0] * scale,
                            second.1 + self.col_offsets[second.1] * scale,
                        );
                        irow.abs_diff(jrow) + icol.abs_diff(jcol)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u64
    }

    fn part1(&mut self) -> u64 {
        self.sum_distances(2)
    }

    fn part2(&mut self) -> u64 {
        self.sum_distances(1_000_000)
    }

}

pub fn p1(input: &str) -> u64 {
    let mut universe = Universe::default();
    universe.parse(input);
    universe.part1()
}

pub fn p2(input: &str) -> u64 {
    let mut universe = Universe::default();
    universe.parse(input);
    universe.part2()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

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
