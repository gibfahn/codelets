// Run with `swift test -c release` (last test is slowwww so needs release mode)

import IntegerStrings
import Testing
import XCTest

@Test func testNthDigit() throws {
    #expect(1 == nthDigit(n: 999, i: 11))
}

@Test func testCountFives() throws {
    #expect(11 == countFives(n: 1, i: 101))
}

@Test func testFindSubstringNaive() throws {
    #expect((1, 9) == findSubstringNaive(n: 1, substring: "123456789"))
    #expect((223, 227) == findSubstringNaive(n: 1, substring: "11111"))

}

@Test func testFindSubstringFaster() throws {
    #expect((1, 9) == findSubstringFaster(n: 1, substring: "123456789"))
    #expect((223, 227) == findSubstringFaster(n: 1, substring: "11111"))
}

@Test func testFindSubstringFasterBig() throws {
    #expect(
        (1_677_777_779, 1_677_777_787) == findSubstringFaster(n: 1, substring: "987654321"))
}
