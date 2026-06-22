import math
import pickle

from rule30 import Rule30Random, random


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


def test_random_unit_interval():
    r = Rule30Random(42)

    for _ in range(1000):
        value = r.random()
        assert isinstance(value, float)
        assert 0.0 <= value < 1.0, value

    value = random()
    assert isinstance(value, float)
    assert 0.0 <= value < 1.0, value
