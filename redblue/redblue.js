#!/usr/bin/env node

const assert = require('assert');

function matches(pattern, input) {
  const map = {};
  let repeated_chars = false;
  for (let p of pattern) {
    if (map[p] === undefined) {
      map[p] = '';
    } else {
      repeated_chars = true;
    }
  }
  if (! repeated_chars) { return true; }
  return check_string(pattern, input, map);
}

function check_string(pattern, input, map) {
  if (pattern === '' || input === '') {
    return (pattern === '' && input === '');
  }
  let p = pattern[0];
  let p_string = map[p];
  if (p_string === '') {
    for (let i = 1; i < input.length; i++) {
      let new_map = Object.assign({}, map);
      new_map[p] = input.slice(0,i);
      if (check_string(pattern.slice(1), input.slice(i), new_map)) {
        const vals = Object.values(new_map);
        const hasDups = vals.some((value, index) => vals.indexOf(value, index + 1) !== -1);
        if (!hasDups) {
          return true;
        }
      }
    }
    return false;
  } else {
    if (p_string.length <= input.length && p_string === input.slice(0,p_string.length)) {
      return check_string(pattern.slice(1), input.slice(p_string.length), Object.assign({}, map));
    } else {
      return false;
    }
  }
}

assert.strictEqual(matches("abdc", "odsihpoyywepqriohweoyafpsdoyh"), true);
assert.strictEqual(matches("abba", "redbluebluered"), true);
assert.strictEqual(matches("abba", "redredredred"), false);
assert.strictEqual(matches("abba", "redbluebluereda"), false);
assert.strictEqual(matches("abba", "abcxyzxyzabc"), true);
assert.strictEqual(matches("abba", "abcxyzxyzabc"), true);
assert.strictEqual(matches("baab", "abcxyzxyzabc"), true);
assert.strictEqual(matches("dzzd", "abcxyzxyzabc"), true);
assert.strictEqual(matches("dzzd", "dzzda"), false);
assert.strictEqual(matches("aba", "patrpatrr"), false);

console.log('All done!');
