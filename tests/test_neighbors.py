from contextlib import nullcontext

import pytest

from geohashr import EncodeError, ParamError, neighbor, neighbors


@pytest.mark.parametrize(
    ("hash", "raises", "out"),
    [
        ("************", pytest.raises(EncodeError), {}),
        (
            "000000000000",
            nullcontext(),
            {
                "e": "000000000002",
                "n": "000000000001",
                "ne": "000000000003",
                "nw": "pbpbpbpbpbpc",
                "s": "bpbpbpbpbpbp",
                "se": "bpbpbpbpbpbr",
                "sw": "zzzzzzzzzzzz",
                "w": "pbpbpbpbpbpb",
            },
        ),
        (
            "zzzzzzzzzzzz",
            nullcontext(),
            {
                "e": "bpbpbpbpbpbp",
                "n": "pbpbpbpbpbpb",
                "ne": "000000000000",
                "nw": "pbpbpbpbpbp8",
                "s": "zzzzzzzzzzzy",
                "se": "bpbpbpbpbpbn",
                "sw": "zzzzzzzzzzzw",
                "w": "zzzzzzzzzzzx",
            },
        ),
        (
            "bgr96qxvpd46",
            nullcontext(),
            {
                "e": "bgr96qxvpd4d",
                "n": "bgr96qxvpd47",
                "ne": "bgr96qxvpd4e",
                "nw": "bgr96qxvpd45",
                "s": "bgr96qxvpd43",
                "se": "bgr96qxvpd49",
                "sw": "bgr96qxvpd41",
                "w": "bgr96qxvpd44",
            },
        ),
    ],
)
def test_neighbors(hash, raises, out):
    with raises:
        assert out == neighbors(hash)


@pytest.mark.parametrize(
    ("direction", "out"),
    [
        ("e", "bgr96qxvpd4d"),
        ("n", "bgr96qxvpd47"),
        ("ne", "bgr96qxvpd4e"),
        ("nw", "bgr96qxvpd45"),
        ("s", "bgr96qxvpd43"),
        ("se", "bgr96qxvpd49"),
        ("sw", "bgr96qxvpd41"),
        ("w", "bgr96qxvpd44"),
    ],
)
def test_neighbor(direction, out):
    assert neighbor("bgr96qxvpd46", direction) == out


def test_neighbor_invalid_direction():
    with pytest.raises(ParamError):
        assert neighbor("bgr96qxvpd46", "g")
