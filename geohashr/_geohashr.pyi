from typing import Tuple, TypedDict

__version__: str

class DecodeError(ValueError): ...
class EncodeError(ValueError): ...
class ParamError(SyntaxError): ...

class BBox(TypedDict):
    e: float
    n: float
    s: float
    w: float

class Neighbors(TypedDict):
    e: str
    n: str
    ne: str
    nw: str
    s: str
    se: str
    sw: str
    w: str

def bbox(hash: str) -> BBox: ...
def decode_exact(hash: str) -> Tuple[float, float, float, float]: ...
def decode(hash: str) -> Tuple[float, float]: ...
def encode(lat: float, lon: float, len: int = 12) -> str: ...
def neighbor(hash: str, direction: str) -> str: ...
def neighbors(hash: str) -> Neighbors: ...
