use std::fmt::Write;

use super::*;

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
pub fn attr_from_closure<F: FnOnce(&mut AttrWrite) -> fmt::Result>(func: F) -> AttrClosure<F> {
    AttrClosure { func }
}

// pub fn raw<D: fmt::Display>(data: D) -> Raw<D> {
//     Raw { data }
// }

pub fn raw_escapable<D: fmt::Display>(data: D) -> RawEscapable<D> {
    RawEscapable { data }
}

pub fn from_iter<I: Iterator<Item = R>, R: Elem>(iter: I) -> Iter<I> {
    Iter { iter }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Closure<I> {
    func: I,
}
pub fn from_closure<F: FnOnce(&mut ElemWrite) -> fmt::Result>(func: F) -> Closure<F> {
    Closure { func }
}

impl<I: FnOnce(&mut ElemWrite) -> fmt::Result> Elem for Closure<I> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        (self.func)(w)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Iter<I> {
    iter: I,
}

impl<I: IntoIterator<Item = R>, R: Elem> Elem for Iter<I> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        for i in self.iter {
            w.render(i)?;
        }
        Ok(())
    }
}

// #[derive(Copy, Clone)]
// #[must_use]
// pub struct Raw<D> {
//     data: D,
// }

// impl<D: fmt::Display> RenderElem for Raw<D> {
//     type Tail = ();
//     fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
//         write!(w.writer(), " {}", self.data)?;
//         Ok(())
//     }
// }

impl<D: fmt::Display> Elem for D {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        write!(w.writer(), " {}", self)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct RawEscapable<D> {
    data: D,
}

impl<D: fmt::Display> Elem for RawEscapable<D> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        //TODO write one global function
        write!(w.writer_escapable(), " {}", self.data)?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Single<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Single<D, A> {
    pub fn with<AA: Attr>(self, attr: AA) -> Single<D, AA> {
        Single {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> Elem for Single<D, A> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Single { tag, attr } = self;
        w.writer_escapable().write_char('<')?;
        write!(w.writer(), "{}", tag)?;
        w.writer().write_char(' ')?;
        attr.render(&mut w.as_attr_write())?;
        w.writer_escapable().write_str(" />")?;
        Ok(())
    }
}

pub fn single<D: fmt::Display>(tag: D) -> Single<D, ()> {
    Single { tag, attr: () }
}

pub fn elem<D: fmt::Display>(tag: D) -> Element<D, ()> {
    Element { tag, attr: () }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct ElemTail<D> {
    tag: D,
}

impl<D: fmt::Display> RenderTail for ElemTail<D> {
    fn render(self, w: &mut ElemWrite) -> std::fmt::Result {
        w.writer_escapable().write_str("</")?;
        write!(w.writer(), "{}", &self.tag)?;
        w.writer_escapable().write_char('>')
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Element<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Element<D, A> {
    pub fn with<AA: Attr>(self, attr: AA) -> Element<D, AA> {
        Element {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> Elem for Element<D, A> {
    type Tail = ElemTail<D>;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Element { tag, attr } = self;

        w.writer_escapable().write_char('<')?;
        write!(w.writer(), "{}", tag)?;
        w.writer().write_char(' ')?;
        attr.render(&mut w.as_attr_write())?;
        w.writer_escapable().write_str(" >")?;

        Ok(ElemTail { tag })
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
    impl<F: FnOnce(PathSink) -> fmt::Result> Attr for PathFlexible<F> {
        fn render(self, w: &mut AttrWrite) -> fmt::Result {
            w.writer_escapable().write_str(" d=\"")?;
            (self.func)(PathSink { writer: w })?;
            w.writer_escapable().write_str("\"")
        }
    }

    pub fn path_ext<F: FnOnce(PathSink) -> fmt::Result>(func: F) -> PathFlexible<F> {
        sink::PathFlexible { func }
    }
    #[test]
    fn test() {
        use PathCommand::*;
        path_ext(|s| {
            let mut s = s.start();
            s.put(M(0, 0))?;
            s.put(L(0, 0))?;
            s.put(Z())?;
            Ok(())
        });
    }
}

pub fn path<I: IntoIterator<Item = PathCommand<D>>, D: fmt::Display>(iter: I) -> Path<I> {
    Path { iter }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Points<I> {
    iter: I,
}

pub fn points<I: IntoIterator<Item = (D, D)>, D: fmt::Display>(iter: I) -> Points<I> {
    Points { iter }
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

///
/// If you need to render something over, and over again,
/// you can instead buffer it to a string using this struct
/// for better performance at the cost of more memory.
///
/// Notice that RenderElem is only implemented for a &BufferedElem.
///
pub struct BufferedElem {
    head: String,
    tail: String,
}

impl BufferedElem {
    pub fn new<E: Elem>(elem: E) -> Result<Self, fmt::Error> {
        let mut head = String::new();
        let mut tail = String::new();
        let t = elem.render_head(&mut ElemWrite::new(&mut head))?;
        t.render(&mut ElemWrite::new(&mut tail))?;
        head.shrink_to_fit();
        tail.shrink_to_fit();
        Ok(BufferedElem { head, tail })
    }
}

pub struct BufferedTail<'a> {
    tail: &'a str,
}
impl<'a> RenderTail for BufferedTail<'a> {
    fn render(self, w: &mut ElemWrite) -> std::fmt::Result {
        write!(w.writer_escapable(), "{}", self.tail)
    }
}
impl<'a> Elem for &'a BufferedElem {
    type Tail = BufferedTail<'a>;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        write!(w.writer_escapable(), "{}", self.head)?;
        Ok(BufferedTail { tail: &self.tail })
    }
}

// pub struct DisplayEscapable<D, B> {
//     start: D,
//     end: B,
// }
// pub struct DisplayEscapableTail<'a, D> {
//     end: &'a D,
// }
// impl<A: fmt::Display, B: fmt::Display> DisplayEscapable<A, B> {
//     pub fn new(head: A, tail: B) -> Self {
//         DisplayEscapable {
//             start: head,
//             end: tail,
//         }
//     }
// }
// impl<'b, D: fmt::Display> RenderTail for DisplayEscapableTail<'b, D> {
//     fn render(self, w: &mut ElemWrite) -> std::fmt::Result {
//         write!(w.writer_escapable(), "{}", self.end)
//     }
// }
// impl<'a, A: fmt::Display, B: fmt::Display> Elem for &'a DisplayEscapable<A, B> {
//     type Tail = DisplayEscapableTail<'a, B>;
//     fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
//         write!(w.writer_escapable(), "{}", self.start)?;
//         Ok(DisplayEscapableTail { end: &self.end })
//     }
// }
