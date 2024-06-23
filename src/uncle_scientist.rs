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

    fn part1(&mut self) -> u64 {
        let mut total_distance = 0;
        for i in 0..self.galaxies.len()-1 {
            let (irow, icol) = (
                self.galaxies[i].0 + self.row_offsets[self.galaxies[i].0],
                self.galaxies[i].1 + self.col_offsets[self.galaxies[i].1]);
            for j in i+1..self.galaxies.len() {
                let (jrow, jcol) = (
                    self.galaxies[j].0 + self.row_offsets[self.galaxies[j].0],
                    self.galaxies[j].1 + self.col_offsets[self.galaxies[j].1]
                );
                total_distance += irow.abs_diff(jrow) + icol.abs_diff(jcol)
            }
        }
        total_distance as u64
    }

}

pub fn p1(input: &str) -> u64 {
    let mut universe = Universe::default();
    universe.parse(input);
    universe.part1()
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
        let mut universe = Universe::default();
        universe.parse(&buf);
        let result = universe.part1();
        assert_eq!(result, 9509330)
    }
}