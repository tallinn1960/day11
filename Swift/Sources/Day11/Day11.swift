// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

@_cdecl("part1_swift_ffi")
public func part1FFI(data_bytes: UnsafeMutablePointer<UInt8>, _ count: UInt64) -> UInt64 {
    let data = Data(bytesNoCopy: data_bytes, count: Int(count), deallocator: .none)
    return part1(data) as UInt64
}

public func part1(_ data: Data) -> UInt64 {
    print("Day 11, Part 1")
    return 0
}