// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

@_cdecl("part1_swift_ffi")
public func part1FFI(dataBytes: UnsafeMutablePointer<UInt8>, _ count: UInt64) -> UInt64 {
    let data = Data(bytesNoCopy: dataBytes, count: Int(count), deallocator: .none)
    return part1(data) as UInt64
}

@_cdecl("part2_swift_ffi")
public func part2FFI(dataBytes: UnsafeMutablePointer<UInt8>, _ count: UInt64) -> UInt64 {
    let data = Data(bytesNoCopy: dataBytes, count: Int(count), deallocator: .none)
    return part2(data) as UInt64
}

public func part1(_ data: Data) -> UInt64 {
    let universe = Universe(data: data)
    return UInt64(universe.sum_all_distances_after_expansion())
}

public func part2(_ data: Data) -> UInt64 {
    let universe = Universe(data: data, extraSpace: 999_999)
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
    let emptyColumns: [Bool]
    let emptyRows: [Bool]
    let extraSpace: Int
}

extension Universe {
    init(data: Data, extraSpace: Int = 1) {
        let lines = data.split(separator: 10)
        var galaxies: [Galaxy] = []
        var emptyColumns = [Bool](repeating: true, count: lines.first!.count)
        var emptyRows = [Bool](repeating: false, count: lines.count)
        for (y, line) in lines.enumerated() {
            var noGalaxyInLine = true
            for (x, char) in line.enumerated() where char == UInt8(ascii: "#") {
                galaxies.append(Galaxy(x: x, y: y))
                emptyColumns[x] = false
                noGalaxyInLine = false
            }
            if noGalaxyInLine {
                emptyRows[y] = true
            }
        }
        self.galaxies = galaxies
        self.emptyColumns = emptyColumns
        self.emptyRows = emptyRows
        self.extraSpace = extraSpace
    }

    func move_by_expansion(galaxy: Galaxy) -> Galaxy {
        let x = emptyColumns[..<galaxy.x].filter { $0 }.count * extraSpace + galaxy.x
        let y = emptyRows[..<galaxy.y].filter { $0 }.count * extraSpace + galaxy.y
        return Galaxy(x: x, y: y)
    }

    func sum_all_distances_after_expansion() -> Int {
        let expandedGalaxies = galaxies.map(move_by_expansion)
        return expandedGalaxies.enumerated().reduce(0) { result, enumeratedGalaxy in
            result
                + expandedGalaxies[enumeratedGalaxy.0...].reduce(0) { result, galaxy in
                    result + enumeratedGalaxy.1.distance(to: galaxy)
                }
        }
    }
}
