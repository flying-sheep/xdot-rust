import pytest
import xdot_rs
from xdot_rs.draw import Pen
from xdot_rs.shapes import Ellipse


def test_import_structure():
    assert callable(xdot_rs.parse)  # not a FunctionType?
    assert isinstance(xdot_rs.ShapeDraw, type)


@pytest.mark.parametrize("input", ["e 27 90 27 18"])
def test_parse(input):
    assert xdot_rs.parse(input) == [xdot_rs.ShapeDraw(Ellipse(27, 90, 27, 18), Pen())]


def test_parse_error():
    with pytest.raises(ValueError, match=r"error Tag"):
        xdot_rs.parse("")
