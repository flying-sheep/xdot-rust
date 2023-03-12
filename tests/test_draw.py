from types import ModuleType

from xdot_rs import draw


def test_import_structure():
    assert isinstance(draw, ModuleType)
    assert isinstance(draw.FontCharacteristics, type)
    assert isinstance(draw.Rgba, type)
    assert isinstance(draw.Style, type)
    assert isinstance(draw.Pen, type)
