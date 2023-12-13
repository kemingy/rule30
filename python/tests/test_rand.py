import math
import pickle

from rule30 import Rule30Random


def test_pickle():
    r = Rule30Random()
    for _ in range(100):
        r.random()
    rr = pickle.loads(pickle.dumps(r))
    expect = r.random()
    got = rr.random()
    assert math.isclose(expect, got), (expect, got)


def test_gen_bytes():
    r = Rule30Random()
    for k in range(1000):
        n = r.getrandbits(k)
        assert 0 <= n < 2**k, (n, k, 2**k)
