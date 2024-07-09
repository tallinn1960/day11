import XCTest

@testable import Day11

extension Galaxy: @retroactive Equatable {
  public static func == (lhs: Galaxy, rhs: Galaxy) -> Bool {
    return lhs.x == rhs.x && lhs.y == rhs.y
  }
}

final class Day11Tests: XCTestCase {
  func test_big_bang() {
    let data = """
      ...#......
      .......#..
      #.........
      ..........
      ......#...
      .#........
      .........#
      ..........
      .......#..
      #...#.....
      """.data(using: .utf8)!
    let universe = Universe(data: data)
    XCTAssertEqual(universe.galaxies.count, 9)
    XCTAssertEqual(
      universe.columnIsEmpty, [false, false, true, false, false, true, false, false, true, false])
    XCTAssertEqual(
      universe.rowIsEmpty, [false, false, false, true, false, false, false, true, false, false])
    XCTAssertEqual(
      universe.galaxies,
      [
      Galaxy(x: 3, y: 0),
      Galaxy(x: 7, y: 1),
      Galaxy(x: 0, y: 2),
      Galaxy(x: 6, y: 4),
      Galaxy(x: 1, y: 5),
      Galaxy(x: 9, y: 6),
      Galaxy(x: 7, y: 8),
      Galaxy(x: 0, y: 9),
      Galaxy(x: 4, y: 9)])

  }

  func test_p1_sample() {
        let data = """
      ...#......
      .......#..
      #.........
      ..........
      ......#...
      .#........
      .........#
      ..........
      .......#..
      #...#.....
      """.data(using: .utf8)!
    let universe = Universe(data: data)
    XCTAssertEqual(universe.sum_all_distances_after_expansion(), 374)
  }
}
