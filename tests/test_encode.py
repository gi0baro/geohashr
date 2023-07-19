import pytest

from geohashr import bbox, encode, decode


@pytest.mark.parametrize("hash", ["000000000000","zzzzzzzzzzzz","bgr96qxvpd46"])
def test_cycle(hash):
    assert hash == encode(*decode(hash))


def test_simple_encode():
    assert encode(57.64911,10.40744) == 'u4pruydqqvj8'
