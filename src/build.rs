use super::*;

pub fn raw_escapable<D: fmt::Display>(data: D) -> RawEscapable<D> {
    RawEscapable::new(data)
}

pub fn from_closure_escapable<F: FnOnce(&mut ElemWriteEscapable) -> fmt::Result>(
    func: F,
) -> ClosureEscapable<F> {
    ClosureEscapable::new(func)
}

pub fn from_closure<F: FnOnce(&mut ElemWrite) -> fmt::Result>(func: F) -> Closure<F> {
    Closure::new(func)
}
pub fn from_iter<I: Iterator<Item = R>, R: Elem>(iter: I) -> Iter<I> {
    Iter::new(iter)
}

pub fn single<D: fmt::Display>(tag: D) -> Single<D, (), &'static str, &'static str> {
    Single::new(tag)
}

pub fn elem<D: fmt::Display>(tag: D) -> Element<D, ()> {
    Element::new(tag)
}

pub fn attr_from_closure<F: FnOnce(&mut AttrWrite) -> fmt::Result>(func: F) -> AttrClosure<F> {
    AttrClosure::new(func)
}

pub fn path<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display>(iter: I) -> Path<I> {
    Path::new(iter)
}

pub fn points<I: IntoIterator<Item = (D, D)>, D: fmt::Display>(iter: I) -> Points<I> {
    Points::new(iter)
}
