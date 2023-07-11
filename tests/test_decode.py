import pytest

from geohashr import encode, decode


@pytest.mark.parametrize(("lat", "lon"), [(51.566141, -0.009434)])
def test_cycle(lat, lon):
    r1, r2 = decode(encode(lat, lon))
    assert round(r1, 6) == lat
    assert round(r2, 6) == lon
