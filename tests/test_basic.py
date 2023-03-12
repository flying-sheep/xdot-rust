from types import ModuleType

import xdot_rs


def test_import_structure_base():
    pass


def test_import_structure_draw():
    assert isinstance(xdot_rs.draw, ModuleType)


def test_import_structure_shapes():
    assert isinstance(xdot_rs.shapes, ModuleType)
    assert isinstance(xdot_rs.shapes.Shape, type)
