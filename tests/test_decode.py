import pytest

from geohashr import bbox, decode, encode


@pytest.mark.parametrize(("lat", "lon"), [(51.566141, -0.009434)])
def test_cycle(lat, lon):
    r1, r2 = decode(encode(lat, lon))
    assert round(r1, 6) == lat
    assert round(r2, 6) == lon


@pytest.mark.parametrize(
    "params",
    [
        ("0", (-90.0, -180.0)),
        ("1", (-90.0, -135.0)),
        ("2", (-45.0, -180.0)),
        ("3", (-45.0, -135.0)),
        ("4", (-90.0, -90.0)),
        ("5", (-90.0, -45.0)),
        ("6", (-45.0, -90.0)),
        ("7", (-45.0, -45.0)),
        ("8", (0.0, -180.0)),
        ("9", (0.0, -135.0)),
        ("b", (45.0, -180.0)),
        ("c", (45.0, -135.0)),
        ("d", (0.0, -90.0)),
        ("e", (0.0, -45.0)),
        ("f", (45.0, -90.0)),
        ("g", (45.0, -45.0)),
        ("h", (-90.0, 0.0)),
        ("j", (-90.0, 45.0)),
        ("k", (-45.0, 0.0)),
        ("m", (-45.0, 45.0)),
        ("n", (-90.0, 90.0)),
        ("p", (-90.0, 135.0)),
        ("q", (-45.0, 90.0)),
        ("r", (-45.0, 135.0)),
        ("s", (0.0, 0.0)),
        ("t", (0.0, 45.0)),
        ("u", (45.0, 0.0)),
        ("v", (45.0, 45.0)),
        ("w", (0.0, 90.0)),
        ("x", (0.0, 135.0)),
        ("y", (45.0, 90.0)),
        ("z", (45.0, 135.0)),
    ],
)
def test_hash_segment(params):
    segment, target = params
    ret = bbox(segment)
    assert (ret["s"], ret["w"]) == target
    assert ret["n"] - ret["s"] == 45
    assert ret["e"] - ret["w"] == 45


def test_simple_decode():
    lat, lon = decode("u4pruydqqvj8")
    assert (57.64911, 10.40744) == (round(lat, 6), round(lon, 6))
