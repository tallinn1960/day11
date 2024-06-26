use std::{
    fs::File,
    io::Read,
};

use day11::{p1, p2};



fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("can't read file");
    let result = p1(&buf);
    println!("{result}");
    let result = p2(&buf);
    println!("{result}");
    #[cfg(feature = "Swift")] {
        let result = day11::swift::part1_swift( buf.as_mut_str());
        println!("{result}");
        let result = day11::swift::part2_swift(buf.as_mut_str());
        println!("{result}")
    }
}

