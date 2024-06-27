extern crate link_cplusplus;

extern "C" {
    pub fn part1_cpp(data: *const u8, size: usize) -> u64;
    pub fn part2_cpp(data: *const u8, size: usize) -> u64;
}

