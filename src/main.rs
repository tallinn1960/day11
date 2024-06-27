use std::{fs::File, io::Read};

use day11::{p1, p2};


fn main() {
    let mut f = File::open("input.txt").expect("can't open file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("can't read file");
    let result = p1(&buf);
    println!("part1_rust: {result}");
    let result = p2(&buf);
    println!("part2_rust: {result}");
    let result = unsafe { day11_cpp::part1_cpp(buf.as_ptr(), buf.len()) };
    println!("part1_cpp: {result}");
    let result = unsafe { day11_cpp::part2_cpp(buf.as_ptr(), buf.len()) };
    println!("part2_cpp: {result}");
    #[cfg(feature = "Swift")]
    {
        let result = day11_swift::part1_swift(buf.as_mut_str());
        println!("part1_swift: {result}");
        let result = day11_swift::part2_swift(buf.as_mut_str());
        println!("part2_swift: {result}");
    }
}
