//! Drawable shapes included in [Shape].

/// A drawable shape including closed shapes, lines, and text.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Ellipse(Ellipse),
    Points(Points),
    Text(Text),
}
#[cfg(feature= "pyo3")]
unsafe impl pyo3::type_object::PyTypeInfo for Shape {
    type AsRefTarget = pyo3::PyCell<Self>;
    const NAME: &'static str = "Shape";
    const MODULE: ::std::option::Option<&'static str> = ::std::option::Option::None;
    #[inline]
    fn type_object_raw(py: pyo3::Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
        use pyo3::type_object::LazyStaticType;
        static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
        TYPE_OBJECT.get_or_init::<Self>(py)
    }
}
#[cfg(feature= "pyo3")]
impl pyo3::PyClass for Shape {
    type Frozen = pyo3::pyclass::boolean_struct::False;
}
#[cfg(feature= "pyo3")]
impl pyo3::impl_::pyclass::PyClassImpl for Shape {
    const DOC: &'static str = "A drawable shape including closed shapes, lines, and text.";
    const IS_BASETYPE: bool = false;
    const IS_SUBCLASS: bool = false;
    type Layout = pyo3::PyCell<Shape>;
    type BaseType = pyo3::PyAny;
    type ThreadChecker = pyo3::impl_::pyclass::ThreadCheckerStub<Shape>;
    type PyClassMutability = <<pyo3::PyAny as pyo3::impl_::pyclass::PyClassBaseType>::PyClassMutability as pyo3::impl_::pycell::PyClassMutability>::MutableChild;
    type Dict = pyo3::impl_::pyclass::PyClassDummySlot;
    type WeakRef = pyo3::impl_::pyclass::PyClassDummySlot;
    type BaseNativeType = pyo3::PyAny;

    fn items_iter() -> pyo3::impl_::pyclass::PyClassItemsIter {
        use pyo3::impl_::pyclass::*;
        let collector = PyClassImplCollector::<Shape>::new();
        static INTRINSIC_ITEMS: PyClassItems = PyClassItems { slots: &[], methods: &[] };
        PyClassItemsIter::new(&INTRINSIC_ITEMS, collector.py_methods())
    }
}

/// A horizontal ellipse shape.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub struct Ellipse {
    pub filled: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl From<Ellipse> for Shape {
    fn from(val: Ellipse) -> Self {
        Shape::Ellipse(val)
    }
}

/// Type of shape defined by a sequence of points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub enum PointsType {
    Polygon,
    Polyline,
    BSpline,
}
/// Shape defined by a sequence of points (line or closed shape).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub struct Points {
    pub filled: bool,
    pub typ: PointsType,
    pub points: Vec<(f32, f32)>,
}
impl From<Points> for Shape {
    fn from(val: Points) -> Self {
        Shape::Points(val)
    }
}

/// Horizontal text alignment: left, center, or right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
/// Multiline text for node or edge labels.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub align: TextAlign,
    pub width: f32,
    pub text: String,
}
impl From<Text> for Shape {
    fn from(val: Text) -> Self {
        Shape::Text(val)
    }
}

/// External image, currently unimplemented.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature= "pyo3", pyo3::pyclass)]
pub struct ExternalImage;

#[cfg(feature= "pyo3")]
#[pyo3::pymodule]
#[pyo3(name = "shapes")]
pub fn pymodule(_py: pyo3::Python, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
    m.add_class::<Shape>()?;
    m.add_class::<Ellipse>()?;
    m.add_class::<PointsType>()?;
    m.add_class::<Points>()?;
    m.add_class::<TextAlign>()?;
    m.add_class::<Text>()?;
    m.add_class::<ExternalImage>()?;
    Ok(())
}
