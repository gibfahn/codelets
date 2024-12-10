import Foundation

// Uncomment to unbuffer stdout so we can see print debugging.
// https://stackoverflow.com/questions/57095407/swift-print-doesnt-appear-in-stdout-but-3rd-party-c-library-logs-do-when-runn
import Darwin
setbuf(stdout, nil)

/**
A string is generated by joining together, in order, all the integers starting with n. We are interested in
which digit appears in the ith
position in this string.
For example:
• If we start at 6 the string will begin 678910111213141516…;
• The 11th
digit in this string is 1 and the 12th
is 3;
• If we start at 999 the string will begin 999100010011002… and the 10th
digit is 0.

Write a program that reads in integers n then i (both between 1 and 259
inclusive),
indicating the first number that appears in the string and the required digit from the
string respectively. You should output the digit in the ith
position in the string.

Sample run
999 11
1
*/
public func nthDigit(n: Int, i: Int) -> Int {
    var n = n
    var s = String()
    while s.count < i {
        s.append(String(n))
        n += 1
    }
    return s[s.index(s.startIndex, offsetBy: i - 1)].wholeNumberValue!
}

print("11th digit counting from 999 [should be 1]:", nthDigit(n: 999, i: 11))

// --------------------------------------------------

/**
 Consider the string generated when n = 1.

 How many occurrences of the digit 5 appear in the first 101
digits?
 */
public func countFives(n: Int, i: Int) -> Int {
    var n = n
    var s = String()

    while s.count < i {
        s.append(String(n))
        n += 1
    }
    s = String(s.prefix(i - 1))
    let fives = s.filter{ $0 == "5"}
    return fives.count
}



// --------------------------------------------------


/**
Naive solution to this problem:

Consider the string generated when n
=
1. The substring 123456789 first appears in this string between
positions 1 and 9 inclusive.String.IndexString.Index
Where does the substring 11111 first appear? Where does the substring 987654321 first appear?
*/
public func findSubstringNaive(n: Int, substring: String) -> (Int, Int) {
    var n = n
    var s = String()

    while true {
        s.append(String(n))
        n += 1
        if let range = s.range(of: substring) {
            // print(s)
            return (
                s.distance(from: s.startIndex, to: range.lowerBound) + 1,
                s.distance(from: s.startIndex, to: range.upperBound)
            )
        }
    }
}

/**
Slightly better solution to this problem:

Consider the string generated when n
=
1. The substring 123456789 first appears in this string between
positions 1 and 9 inclusive.String.IndexString.Index
Where does the substring 11111 first appear? Where does the substring 987654321 first appear?
*/
public func findSubstringFaster(n: Int, substring: String) -> (Int, Int) {
    var n = n
    var s = String()
    var length = 0

    var substring_prefixes: [String] = []
    for len in (1...substring.count - 1).reversed() {
        substring_prefixes.append(String(substring.prefix(len)))
    }

    while true {
        s.append(String(n))
        if let range = s.range(of: substring) {
            return (
                s.distance(from: s.startIndex, to: range.lowerBound) + 1 + length,
                s.distance(from: s.startIndex, to: range.upperBound) + length
            )
        }
        var found = false
        for prefix in substring_prefixes {
            if s.hasSuffix(prefix) {
                found = true
                length += (s.count - prefix.count)
                s = prefix
                break
            }
        }
        if !found {
            length += s.count
            s.removeAll()
        }
        n += 1
    }
}