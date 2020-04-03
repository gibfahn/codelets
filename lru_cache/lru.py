#!/usr/bin/env python3

from collections import OrderedDict

class LRUCache:

    def __init__(self, capacity):
        self.capacity = capacity
        self.cache = OrderedDict()

    def get(self, key):
        if key not in self.cache:
            return None
        v = self.cache.pop(key)
        self.cache[key] = v
        return v

    def put(self, key, value):
        if key in self.cache:
            self.cache.pop(key)
        else:
            if len(self.cache) == self.capacity:
                self.cache.popitem(last = False)

        self.cache[key] = value


cache = LRUCache(2)
assert cache.get(2) == None
cache.put(2, 6)
assert cache.get(1) == None
cache.put(1, 5)
cache.put(1, 2)
assert cache.get(1) == 2
assert cache.get(2) == 6

cache = LRUCache(3)
cache.put(1, 1)
cache.put(2, 2)
cache.put(3, 3)
cache.put(4, 4)
assert cache.get(4) == 4
assert cache.get(3) == 3
assert cache.get(2) == 2
assert cache.get(1) == None
cache.put(5, 5)
assert cache.get(1) == None
assert cache.get(2) == 2
assert cache.get(3) == 3
assert cache.get(4) == None
assert cache.get(5) == 5
print("Works!")
