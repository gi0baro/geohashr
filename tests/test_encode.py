import pytest

from geohashr import bbox, encode, decode


@pytest.mark.parametrize("hash", ["000000000000","zzzzzzzzzzzz","bgr96qxvpd46"])
def test_cycle(hash):
    assert hash == encode(*decode(hash))
