fn main() {
    cc::Build::new()
    .cpp(true)
    .std("c++20")
    .file("day11cpp.cpp")
    .compile("day11cpp")
}

