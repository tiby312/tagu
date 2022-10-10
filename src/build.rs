//!
//! Functions to build elements and attributes building blocks
//!

use super::*;

///
/// Create an escapable element
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::raw_escapable("<test>I can insert my own elements!</test>");
/// hypermelon::render_escapable(k,&mut s).unwrap()
///
/// ```
///
pub fn raw_escapable<D: fmt::Display>(data: D) -> RawEscapable<D> {
    RawEscapable::new(data)
}

///
/// Create an escapable element from a closure
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::from_closure_escapable(|w|{
///
///     w.render(hypermelon::build::raw_escapable("<test/>"))?;
///     
///     w.render(hypermelon::build::single("test2"))
///
/// });
/// hypermelon::render_escapable(k,&mut s).unwrap()
///
/// ```
///
pub fn from_closure_escapable<F: FnOnce(&mut ElemWriteEscapable) -> fmt::Result>(
    func: F,
) -> ClosureEscapable<F> {
    ClosureEscapable::new(func)
}

///
/// Create an element from a closure
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::from_closure(|w|{
///
///     w.render(hypermelon::build::single("test"))
///
/// });
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
///
pub fn from_closure<F: FnOnce(&mut ElemWrite) -> fmt::Result>(func: F) -> Closure<F> {
    Closure::new(func)
}

///
/// Create an element from an iterator of elements
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::from_iter((0..10).map(|_|hypermelon::build::single("hello")));
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
///
pub fn from_iter<I: Iterator<Item = R>, R: Elem>(iter: I) -> Iter<I> {
    Iter::new(iter)
}

///
/// Create an element that has no closing tag.
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::single("hello");
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
pub fn single<D: fmt::Display>(tag: D) -> Single<D, (), &'static str, &'static str> {
    Single::new(tag)
}

///
/// Create an element.
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::elem("hello");
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
pub fn elem<D: fmt::Display>(tag: D) -> Element<D, ()> {
    Element::new(tag)
}

///
/// Box an element
///
/// ```
/// let mut s = String::new();
/// let k = hypermelon::build::single("hello");
/// let k = hypermelon::build::box_elem(k);
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
pub fn box_elem<'a, E: Elem + 'a>(elem: E) -> DynamicElement<'a> {
    DynamicElement::new(elem)
}

///
/// Create an attr from a closure.
///
/// ```
/// use hypermelon::build;
/// let mut s = String::new();
/// let k = build::elem("hello").with(
///     build::attr_from_closure(|w|
///         w.render(("test","val"))
///     )
/// );
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
pub fn attr_from_closure<F: FnOnce(&mut AttrWrite) -> fmt::Result>(func: F) -> AttrClosure<F> {
    AttrClosure::new(func)
}

///
/// Create a path attribute
///
/// ```
/// use hypermelon::build;
/// use hypermelon::attr::PathCommand;
/// let mut s = String::new();
/// let k = build::elem("hello").with(
///     build::path(
///         (0..10).map(|_|PathCommand::L(5.0,5.0))
///     )
/// );
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
pub fn path<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display>(iter: I) -> Path<I> {
    Path::new(iter)
}

///
/// Create a points attribute
///
/// ```
/// use hypermelon::build;
/// let mut s = String::new();
/// let k = build::elem("hello").with(
///     build::points(
///         (0..10).map(|_|(5.0,5.0))
///     )
/// );
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
pub fn points<I: IntoIterator<Item = (D, D)>, D: fmt::Display>(iter: I) -> Points<I> {
    Points::new(iter)
}

///
/// Create a path attribute from a closure
///
/// ```
/// use hypermelon::build;
/// use hypermelon::attr::PathCommand;
/// let mut s = String::new();
/// let k = build::elem("hello").with(
///     build::path_from_closure(|w|{
///         let mut w=w.start();
///         w.put(PathCommand::L(5.0,5.0))?;
///         w.put(PathCommand::L(5.0,5.0))
///     })
/// );
/// hypermelon::render(k,&mut s).unwrap()
///
/// ```
///
pub fn path_from_closure<F: FnOnce(PathSinkBuilder) -> std::fmt::Result>(
    func: F,
) -> PathClosure<F> {
    PathClosure::new(func)
}
