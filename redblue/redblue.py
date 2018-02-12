#!/usr/bin/env python

def patternmatches(pattern, s):
    patterncount = {}
    for c in pattern:
        if c not in patterncount:
            patterncount[c] = 1
        else:
            break
    else:
        # If we didn't break, no repeating chars in pattern, so must be true.
        return len(pattern) <= len(s)
    return checkstring(s, pattern, {})

def checkstring(s, pattern, matches):
    # print("Checking string {!r}, pattern {!r}, matches {!r}".format(s, pattern, matches))
    s_len = len(s)
    if len(pattern) == 0:
        return s_len == 0
    p = pattern[0] # Pattern char to check.
    if s_len == 0 or s_len < len(pattern):
        return False # Impossible for pattern to match.

    if p in matches: # We've already seen this.
        matches_p = matches[p]
        if matches_p == s[:len(matches_p)]:
            return checkstring(s[len(matches_p):], pattern[1:], matches.copy())
        else:
            return False
    else: # p could be any number of s chars, check all possibilities:
        for i in range(1, len(s)+1):
            newmatches = matches.copy()
            newmatches[p] = s[:i]
            if checkstring(s[i:], pattern[1:], newmatches):
                vals = [v for v in newmatches.values()]
                if len(vals) == len(set(vals)):
                    return True
        else: # None of the possibilities matched.
            return False

def test(pattern, string, expect):
    try:
        assert(patternmatches(pattern, string) == expect)
    except AssertionError as e:
        print(f"Failed: {pattern!r} matches {string!r} was not {expect}")
        raise e

test("abdc", "odsihpoyywepqriohweoyafpsdoyh", True);
test("abba", "redredredred", False);
test("abba", "redbluebluered", True);
test("abba", "redbluebluereda", False);
test("abba", "abcxyzxyzabc", True);
test("abba", "abcxyzxyzabc", True);
test("baab", "abcxyzxyzabc", True);
test("dzzd", "abcxyzxyzabc", True);
test("dzzd", "dzzda", False);

test("abba", "rblblr", True)
test("abab", "redblueredblue", True)
test("abba", "catdogdogcat", True)
test("abab", "redblueredblue", True)
test("abab", "catdogcatdog", True)
test("aba","catdogcat", True)
test("abba","catdogdogcat", True)
test("abcac","catdogmousecatmouse", True)
test("abcde","efghi", True)
test("a","efghi", True)
test("abab","catdogcatcat", False)
test("abab","catdogcatdogg", False)
test("abab","catdocatdog", False)
test("abab","catdogcat", False)
test("abcdefghi","cat", False)
test("abba","redblueredblue", False)
test("aba", "patrpatrr", False)
