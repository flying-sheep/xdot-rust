from types import ModuleType

from xdot_rs import shapes


def test_import_structure():
    assert isinstance(shapes, ModuleType)
    assert isinstance(shapes.Shape, type)
    ...
