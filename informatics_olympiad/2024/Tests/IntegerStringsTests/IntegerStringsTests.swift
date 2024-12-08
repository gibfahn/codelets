import XCTest
import IntegerStrings

/// Can't compare Tuples by default, so need a custom comparator.
/// <https://stackoverflow.com/a/38192766>
private func assertTupleEqual(_ expected: (_: Int, _: Int), _ actual: (_: Int, _: Int), file: StaticString = #file, line: UInt = #line) {
    if expected != actual {
        XCTFail("Expected \(expected) but was \(actual)", file: file, line: line)
    }
}

final class IntegerStringTests: XCTestCase {
    func testNthDigit() throws {
        let expected = 1
        let actual = nthDigit(n: 999, i: 11)
        XCTAssertEqual(expected, actual)
    }

    func testCountFives() throws {
        XCTAssertEqual(11, countFives(n: 1, i: 101))
    }

    func testFindSubstringNaive() throws {
        assertTupleEqual((1, 9), findSubstringNaive(n: 1, substring: "123456789"))
        assertTupleEqual((223, 227), findSubstringNaive(n: 1, substring: "11111"))

    }

    func testFindSubstringFaster() throws {
        assertTupleEqual((1, 9), findSubstringFaster(n: 1, substring: "123456789"))
        assertTupleEqual((223, 227), findSubstringFaster(n: 1, substring: "11111"))
    }

    func testFindSubstringFasterBig() throws {
        assertTupleEqual((1677777779, 1677777787), findSubstringFaster(n: 1, substring: "987654321"))
    }

}
