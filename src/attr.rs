//!
//! Attribute trait and building blocks
//!

use super::*;
use fmt::Write;
pub trait Attr {
    fn render(self, w: &mut AttrWrite) -> std::fmt::Result;
    fn chain<R: Attr>(self, other: R) -> AttrChain<Self, R>
    where
        Self: Sized,
    {
        AttrChain {
            first: self,
            second: other,
        }
    }
}

impl Attr for () {
    fn render(self, _: &mut AttrWrite) -> std::fmt::Result {
        Ok(())
    }
}

#[must_use]
#[derive(Copy, Clone)]
pub struct AttrChain<A, B> {
    first: A,
    second: B,
}
impl<A: Attr, B: Attr> Attr for AttrChain<A, B> {
    fn render(self, w: &mut AttrWrite) -> std::fmt::Result {
        let AttrChain { first, second } = self;
        first.render(w)?;
        w.writer().write_str(" ")?;
        second.render(w)
    }
}

pub struct AttrWrite<'a>(WriteWrap<'a>);
impl<'a> AttrWrite<'a> {
    pub(super) fn new(wrap: WriteWrap<'a>) -> Self {
        AttrWrite(wrap)
    }
    pub fn render<E: Attr>(&mut self, attr: E) -> fmt::Result {
        attr.render(self)
    }
    pub fn writer(&mut self) -> tools::EscapeGuard<WriteWrap> {
        tools::escape_guard(self.0.borrow_mut())
    }

    fn writer_escapable(&mut self) -> WriteWrap {
        self.0.borrow_mut()
    }
}

impl<A: fmt::Display, B: fmt::Display> Attr for (A, B) {
    fn render(self, w: &mut AttrWrite) -> std::fmt::Result {
        let (first, second) = self;
        write!(w.writer(), " {}", first)?;
        w.writer_escapable().write_str("=\"")?;
        write!(w.writer(), "{}", second)?;
        w.writer_escapable().write_str("\"")
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct AttrClosure<I> {
    func: I,
}
impl<F: FnOnce(&mut AttrWrite) -> fmt::Result> Attr for AttrClosure<F> {
    fn render(self, w: &mut AttrWrite) -> fmt::Result {
        (self.func)(w)
    }
}
impl<F> AttrClosure<F>
where
    F: FnOnce(&mut AttrWrite) -> fmt::Result,
{
    pub fn new(func: F) -> Self {
        AttrClosure { func }
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Path<I> {
    iter: I,
}

impl<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display> Attr for Path<I> {
    fn render(self, w: &mut AttrWrite) -> std::fmt::Result {
        w.writer_escapable().write_str(" d=\"")?;

        for command in self.iter {
            command.write(w.writer())?;
        }
        w.writer_escapable().write_str("\"")
    }
}
impl<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display> Path<I> {
    pub fn new(iter: I) -> Self {
        Path { iter }
    }
}

pub mod sink {
    use super::*;
    pub struct PathSink<'a, 'b> {
        writer: &'a mut AttrWrite<'b>,
    }

    pub struct PathSink2<'a, 'b, T> {
        writer: &'a mut AttrWrite<'b>,
        _p: std::marker::PhantomData<T>,
    }
    impl<T: fmt::Display> PathSink2<'_, '_, T> {
        pub fn put(&mut self, command: PathCommand<T>) -> fmt::Result {
            command.write(self.writer.writer())
        }
    }
    impl<'a, 'b> PathSink<'a, 'b> {
        pub fn start<T>(self) -> PathSink2<'a, 'b, T> {
            PathSink2 {
                writer: self.writer,
                _p: std::marker::PhantomData,
            }
        }
    }

    pub struct PathFlexible<F> {
        func: F,
    }
    impl<F: FnOnce(PathSink) -> fmt::Result> PathFlexible<F> {
        pub fn new(func: F) -> Self {
            PathFlexible { func }
        }
    }
    impl<F: FnOnce(PathSink) -> fmt::Result> Attr for PathFlexible<F> {
        fn render(self, w: &mut AttrWrite) -> fmt::Result {
            w.writer_escapable().write_str(" d=\"")?;
            (self.func)(PathSink { writer: w })?;
            w.writer_escapable().write_str("\"")
        }
    }

    #[test]
    fn test() {
        use PathCommand::*;
        PathFlexible::new(|s| {
            let mut s = s.start();
            s.put(M(0, 0))?;
            s.put(L(0, 0))?;
            s.put(Z())?;
            Ok(())
        });
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Points<I> {
    iter: I,
}

impl<I: IntoIterator<Item = (D, D)>, D: fmt::Display> Points<I> {
    pub fn new(iter: I) -> Self {
        Points { iter }
    }
}
impl<I: IntoIterator<Item = (D, D)>, D: fmt::Display> Attr for Points<I> {
    fn render(self, w: &mut AttrWrite) -> std::fmt::Result {
        w.writer_escapable().write_str(" points=\"")?;
        for (x, y) in self.iter {
            write!(w.writer(), "{},{} ", x, y)?;
        }
        w.writer_escapable().write_str("\"")
    }
}

///
/// Construct and Write a SVG path's data.
///
/// following: [w3 spec](https://www.w3.org/TR/SVG/paths.html#PathDataGeneralInformation)
///

#[derive(Copy, Clone)]
#[must_use]
pub enum PathCommand<F> {
    /// move to
    M(F, F),
    /// relative move to
    M_(F, F),
    /// line to
    L(F, F),
    /// relative line to
    L_(F, F),
    /// horizontal to
    H(F),
    /// relative horizontal to
    H_(F),
    /// vertical to
    V(F),
    /// relative vertical to
    V_(F),
    /// curve to
    C(F, F, F, F, F, F),
    /// relative curve to
    C_(F, F, F, F, F, F),
    /// shorthand curve to
    S(F, F, F, F),
    /// relative shorthand curve to
    S_(F, F, F, F),
    /// quadratic bezier curve to
    Q(F, F, F, F),
    /// relative quadratic bezier curve to
    Q_(F, F, F, F),
    /// shorthand quadratic bezier curve to
    T(F, F),
    /// relative shorthand quadratic bezier curve to
    T_(F, F),
    /// elliptical arc
    A(F, F, F, F, F, F, F),
    /// relative elliptical arc
    A_(F, F, F, F, F, F, F),
    /// close path
    Z(),
}

impl<F> PathCommand<F> {
    #[inline(always)]
    fn write<T: fmt::Write>(&self, mut writer: T) -> fmt::Result
    where
        F: fmt::Display,
    {
        use PathCommand::*;
        match self {
            M(x, y) => {
                write!(writer, " M {} {}", x, y)
            }
            M_(x, y) => {
                write!(writer, " m {} {}", x, y)
            }
            L(x, y) => {
                write!(writer, " L {} {}", x, y)
            }
            L_(x, y) => {
                write!(writer, " l {} {}", x, y)
            }
            H(a) => {
                write!(writer, " H {}", a)
            }
            H_(a) => {
                write!(writer, " h {}", a)
            }
            V(a) => {
                write!(writer, " V {}", a)
            }
            V_(a) => {
                write!(writer, " v {}", a)
            }
            C(x1, y1, x2, y2, x, y) => {
                write!(writer, " C {} {}, {} {}, {} {}", x1, y1, x2, y2, x, y)
            }
            C_(dx1, dy1, dx2, dy2, dx, dy) => {
                write!(writer, " c {} {}, {} {}, {} {}", dx1, dy1, dx2, dy2, dx, dy)
            }
            S(x2, y2, x, y) => {
                write!(writer, " S {},{} {} {}", x2, y2, x, y)
            }
            S_(x2, y2, x, y) => {
                write!(writer, " s {},{} {} {}", x2, y2, x, y)
            }
            Q(x1, y1, x, y) => {
                write!(writer, " Q {} {}, {} {}", x1, y1, x, y)
            }
            Q_(dx1, dy1, dx, dy) => {
                write!(writer, " q {} {}, {} {}", dx1, dy1, dx, dy)
            }
            T(x, y) => {
                write!(writer, " T {} {}", x, y)
            }
            T_(x, y) => {
                write!(writer, " t {} {}", x, y)
            }
            A(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) => {
                write!(
                    writer,
                    " A {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y
                )
            }
            A_(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy) => {
                write!(
                    writer,
                    " a {} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy
                )
            }
            Z() => {
                write!(writer, " Z")
            }
        }
    }
}
