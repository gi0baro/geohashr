import pytest

from geohashr import bbox


@pytest.mark.parametrize(
    ("hash", "out"),
    [
        (
            "000000000000",
            {
                "w": -180.0,
                "s": -90.0,
                "e": -179.99999966472387,
                "n": -89.99999983236194,
            },
        ),
        (
            "zzzzzzzzzzzz",
            {"w": 179.99999966472387, "s": 89.99999983236194, "e": 180.0, "n": 90.0},
        ),
        (
            "bgr96qxvpd46",
            {
                "w": -135.59328015893698,
                "s": 63.5375514999032,
                "e": -135.59327982366085,
                "n": 63.537551667541265,
            },
        ),
    ],
)
def test_bbox(hash, out):
    assert out == bbox(hash)
