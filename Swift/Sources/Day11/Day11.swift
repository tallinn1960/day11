// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

@_cdecl("part1_swift_ffi")
public func part1FFI(data_bytes: UnsafeMutablePointer<UInt8>, _ count: UInt64) -> UInt64 {
  let data = Data(bytesNoCopy: data_bytes, count: Int(count), deallocator: .none)
  return part1(data) as UInt64
}

@_cdecl("part2_swift_ffi")
public func part21FFI(data_bytes: UnsafeMutablePointer<UInt8>, _ count: UInt64) -> UInt64 {
  let data = Data(bytesNoCopy: data_bytes, count: Int(count), deallocator: .none)
  return part2(data) as UInt64
}

public func part1(_ data: Data) -> UInt64 {
  let universe = Universe(data: data)
  return UInt64(universe.sum_all_distances_after_expansion())
}

public func part2(_ data: Data) -> UInt64 {
  let universe = Universe(data: data, extra_space: 999_999)
  return UInt64(universe.sum_all_distances_after_expansion())
}

struct Galaxy {
  let x: Int
  let y: Int
}

extension Galaxy {
  func distance(to other: Galaxy) -> Int {
    return abs(x - other.x) + abs(y - other.y)
  }
}

struct Universe {
  let galaxies: [Galaxy]
  let empty_columns: Set<Int>
  let empty_rows: Set<Int>
  let extra_space: Int
}

extension Universe {
  init(data: Data, extra_space: Int = 1) {
    let lines = data.split(separator: 10)
    var galaxies: [Galaxy] = []
    var empty_columns: Set<Int> = Set(0..<lines.first!.count)
    var empty_rows: Set<Int> = []
    for (y, line) in lines.enumerated() {
      var galaxies_found_in_line = false
      for (x, char) in line.enumerated() {
        if char == UInt8(ascii: "#") {
          galaxies.append(Galaxy(x: x, y: y))
          empty_columns.remove(x)
          galaxies_found_in_line = true
        }
      }
      if !galaxies_found_in_line {
        empty_rows.insert(y)
      }
    }
    self.galaxies = galaxies
    self.empty_columns = empty_columns
    self.empty_rows = empty_rows
    self.extra_space = extra_space
  }

  func move_by_expansion(galaxy: Galaxy) -> Galaxy {
    let x = empty_columns.filter({ $0 < galaxy.x }).count * extra_space + galaxy.x
    let y = empty_rows.filter({ $0 < galaxy.y }).count * extra_space + galaxy.y
    return Galaxy(x: x, y: y)
  }

  func sum_all_distances_after_expansion() -> Int {
    let expanded_galaxies = galaxies.map(move_by_expansion)
    return expanded_galaxies.enumerated().reduce(0) { (result, enum_galaxy) in
      result
        + expanded_galaxies[enum_galaxy.0...].reduce(0) { (result, galaxy) in
          result + enum_galaxy.1.distance(to: galaxy)
        }
    }
  }

}
