from cc_sandbox.objstore import ObjStore

import sys

N = 10000
message = b'\\(-"-;) Praying that memory leak would not happen..'
before = sys.getrefcount(message)
store = ObjStore()
for _ in range(N):
    store.push(message)
del store

after = sys.getrefcount(message)
print(after)

