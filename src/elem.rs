//!
//! Elem trait and building blocks
//!

use super::*;

pub struct ElemWriteEscapable<'a>(WriteWrap<'a>);

impl<'a> ElemWriteEscapable<'a> {
    pub fn writer_escapable(&mut self) -> WriteWrap {
        self.0.borrow_mut()
    }
    pub fn writer(&mut self) -> tools::EscapeGuard<WriteWrap> {
        tools::escape_guard(self.0.borrow_mut())
    }

    pub fn render<E: Elem>(&mut self, elem: E) -> fmt::Result {
        let tail = elem.render_head(&mut self.as_elem_write())?;
        tail.render(&mut self.as_elem_write())
    }
    fn as_elem_write(&mut self) -> ElemWrite {
        ElemWrite(WriteWrap(self.0 .0))
    }

    pub fn render_map<E: Elem, F: FnOnce() -> E>(&mut self, func: F) -> fmt::Result {
        let elem = func();
        let tail = elem.render_head(&mut self.as_elem_write())?;
        tail.render(&mut self.as_elem_write())
    }

    pub fn session<'b, E: Elem>(&'b mut self, elem: E) -> SessionEscapable<'b, 'a, E> {
        SessionEscapable { elem, writer: self }
    }

    pub fn session_map<'b, E: Elem, F: FnOnce() -> E>(
        &'b mut self,
        func: F,
    ) -> SessionEscapable<'b, 'a, E> {
        let elem = func();
        SessionEscapable { elem, writer: self }
    }
}

///
/// Render elements
///
#[must_use]
pub struct ElemWrite<'a>(pub(crate) WriteWrap<'a>);

impl<'a> ElemWrite<'a> {
    pub fn writer(&mut self) -> tools::EscapeGuard<WriteWrap> {
        tools::escape_guard(self.0.borrow_mut())
    }
    pub fn render<E: Elem + Locked>(&mut self, elem: E) -> fmt::Result {
        self.render_inner(elem)
    }

    pub fn render_map<E: Elem + Locked, F: FnOnce() -> E>(&mut self, func: F) -> fmt::Result {
        let elem = func();
        let tail = elem.render_head(self)?;
        tail.render(self)
    }

    pub fn session<'b, E: Elem + Locked>(&'b mut self, elem: E) -> Session<'b, 'a, E> {
        Session { elem, writer: self }
    }

    pub fn session_map<'b, E: Elem + Locked, F: FnOnce() -> E>(
        &'b mut self,
        func: F,
    ) -> Session<'b, 'a, E> {
        let elem = func();
        Session { elem, writer: self }
    }
    fn as_escapable(&mut self) -> ElemWriteEscapable {
        ElemWriteEscapable(WriteWrap(self.0 .0))
    }
    fn writer_escapable(&mut self) -> WriteWrap {
        self.0.borrow_mut()
    }

    fn as_attr_write(&mut self) -> AttrWrite {
        attr::AttrWrite::new(self.0.borrow_mut())
    }

    fn new(w: &'a mut dyn fmt::Write) -> Self {
        ElemWrite(WriteWrap(w))
    }

    fn render_inner<E: Elem>(&mut self, elem: E) -> fmt::Result {
        let tail = elem.render_head(self)?;
        tail.render(self)
    }
}

///
/// Alternative trait for Elem that is friendly to dyn trait.
///
pub trait RenderElem {
    fn render_head(&mut self, w: &mut ElemWrite) -> Result<(), fmt::Error>;
    fn render_tail(&mut self, w: &mut ElemWrite) -> Result<(), fmt::Error>;
}

///
/// A element that can be hidden behind a dyn trait.
///
pub struct DynamicElem<E: Elem> {
    head: Option<E>,
    tail: Option<E::Tail>,
}

impl<E: Elem> DynamicElem<E> {
    pub fn new(elem: E) -> DynamicElem<E> {
        DynamicElem {
            head: Some(elem),
            tail: None,
        }
    }
    pub fn as_dyn(&mut self) -> DynElem {
        DynElem { elem: self }
    }
}
impl<E: Elem> RenderElem for DynamicElem<E> {
    fn render_head(&mut self, w: &mut ElemWrite) -> Result<(), fmt::Error> {
        let tail = self.head.take().unwrap().render_head(w)?;
        self.tail = Some(tail);
        Ok(())
    }
    fn render_tail(&mut self, w: &mut ElemWrite) -> Result<(), fmt::Error> {
        self.tail.take().unwrap().render(w)
    }
}

///
/// Tail to DynElem
///
pub struct DynElemTail<'a> {
    elem: &'a mut dyn RenderElem,
}
impl<'a> RenderTail for DynElemTail<'a> {
    fn render(self, w: &mut ElemWrite) -> std::fmt::Result {
        self.elem.render_tail(w)
    }
}

impl<'a> Locked for DynElem<'a> {}
pub struct DynElem<'a> {
    elem: &'a mut dyn RenderElem,
}

impl<'a> Elem for DynElem<'a> {
    type Tail = DynElemTail<'a>;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        self.elem.render_head(w)?;
        Ok(DynElemTail { elem: self.elem })
    }
}

///
/// Main building block.
///
pub trait Elem {
    type Tail: RenderTail;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error>;

    fn render_closure<K>(
        self,
        w: &mut ElemWrite,
        func: impl FnOnce(&mut ElemWrite) -> Result<K, fmt::Error>,
    ) -> Result<K, fmt::Error>
    where
        Self: Sized,
    {
        let tail = self.render_head(w)?;
        let res = func(w)?;
        tail.render(w)?;
        Ok(res)
    }

    /// Render all of Self and head of other, store tail of other.
    fn chain<R: Elem>(self, other: R) -> Chain<Self, R>
    where
        Self: Sized,
    {
        Chain {
            top: self,
            bottom: other,
        }
    }

    /// Render head of Self, and all of other, store tail of self.
    fn append<R: Elem>(self, bottom: R) -> Append<Self, R>
    where
        Self: Sized,
    {
        Append { top: self, bottom }
    }
}

///
/// Indicates that the implementor does that allow arbitrary html escaping.
///
pub trait Locked {}

///
/// Append an element to another adaptor
///
#[must_use]
#[derive(Copy, Clone)]
pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: Locked, B: Locked> Locked for Append<A, B> {}

impl<A: Elem, B: Elem> Elem for Append<A, B> {
    type Tail = A::Tail;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Append { top, bottom } = self;
        let tail = top.render_head(w)?;
        w.render_inner(bottom)?;
        Ok(tail)
    }
}

///
/// Chain two elements adaptor
///
#[must_use]
#[derive(Copy, Clone)]
pub struct Chain<A, B> {
    top: A,
    bottom: B,
}
impl<A: Locked, B: Locked> Locked for Chain<A, B> {}

impl<A: Elem, B: Elem> Elem for Chain<A, B> {
    type Tail = B::Tail;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Chain { top, bottom } = self;
        w.render_inner(top)?;
        bottom.render_head(w)
    }
}

///
/// Tail to elem trait.
///
pub trait RenderTail {
    fn render(self, w: &mut ElemWrite) -> std::fmt::Result;
}

///
/// Used to start a closure session
///
#[must_use]
pub struct Session<'a, 'b, E> {
    elem: E,
    writer: &'a mut ElemWrite<'b>,
}

impl<'a, 'b, E: Elem> Session<'a, 'b, E> {
    pub fn build(self, func: impl FnOnce(&mut ElemWrite) -> fmt::Result) -> fmt::Result {
        let Session { elem, writer } = self;
        let tail = elem.render_head(writer)?;
        func(writer)?;
        tail.render(writer)
    }
}

///
/// Used to start a closure session
///
#[must_use]
pub struct SessionEscapable<'a, 'b, E> {
    elem: E,
    writer: &'a mut ElemWriteEscapable<'b>,
}

impl<'a, 'b, E: Elem> SessionEscapable<'a, 'b, E> {
    pub fn build(self, func: impl FnOnce(&mut ElemWriteEscapable) -> fmt::Result) -> fmt::Result {
        let SessionEscapable { elem, writer } = self;
        let tail = elem.render_head(&mut writer.as_elem_write())?;
        func(writer)?;
        tail.render(&mut writer.as_elem_write())
    }
}

impl<D: fmt::Display> Locked for D {}
impl<D: fmt::Display> Elem for D {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        write!(w.writer(), " {}", self)?;
        Ok(())
    }
}

impl<I: FnOnce(&mut ElemWriteEscapable) -> fmt::Result> Elem for ClosureEscapable<I> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        (self.func)(&mut w.as_escapable())?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct ClosureEscapable<I> {
    func: I,
}

impl<I: FnOnce(&mut ElemWriteEscapable) -> fmt::Result> ClosureEscapable<I> {
    pub fn new(func: I) -> ClosureEscapable<I> {
        ClosureEscapable { func }
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct Closure<I> {
    func: I,
}

impl<I: FnOnce(&mut ElemWrite) -> fmt::Result> Closure<I> {
    pub fn new(func: I) -> Closure<I> {
        Closure { func }
    }
}

impl<I: FnOnce(&mut ElemWrite) -> fmt::Result> Locked for Closure<I> {}

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
impl<I: IntoIterator<Item = R>, R: Elem> Iter<I> {
    pub fn new(iter: I) -> Iter<I> {
        Iter { iter }
    }
}
impl<I: IntoIterator<Item = R>, R: Locked> Locked for Iter<I> {}

impl<I: IntoIterator<Item = R>, R: Elem> Elem for Iter<I> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        for i in self.iter {
            w.render_inner(i)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct RawEscapable<D> {
    data: D,
}
impl<D: fmt::Display> RawEscapable<D> {
    pub fn new(data: D) -> RawEscapable<D> {
        RawEscapable { data }
    }
}
impl<D: fmt::Display> Elem for RawEscapable<D> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        write!(w.writer_escapable(), " {}", self.data)?;
        Ok(())
    }
}

use fmt::Write;

#[derive(Copy, Clone)]
#[must_use]
pub struct Single<D, A, K, Z> {
    tag: D,
    attr: A,
    start: K,
    ending: Z,
}
impl<D: fmt::Display, A: Attr, K: fmt::Display, Z: fmt::Display> Locked for Single<D, A, K, Z> {}
impl<D: fmt::Display, A: Attr, K, Z> Single<D, A, K, Z> {
    pub fn with<AA: Attr>(self, attr: AA) -> Single<D, AA, K, Z> {
        Single {
            tag: self.tag,
            attr,
            ending: self.ending,
            start: self.start,
        }
    }
    pub fn with_map<AA: Attr, F: FnOnce() -> AA>(self, attr: F) -> Single<D, AA, K, Z> {
        let attr = attr();
        self.with(attr)
    }
    pub fn with_ending<ZZ: fmt::Display>(self, ending: ZZ) -> Single<D, A, K, ZZ> {
        Single {
            tag: self.tag,
            attr: self.attr,
            ending,
            start: self.start,
        }
    }
    pub fn with_start<KK: fmt::Display>(self, start: KK) -> Single<D, A, KK, Z> {
        Single {
            tag: self.tag,
            attr: self.attr,
            ending: self.ending,
            start,
        }
    }
}
impl<D: fmt::Display, A: Attr, K: fmt::Display, Z: fmt::Display> Elem for Single<D, A, K, Z> {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Single {
            tag,
            attr,
            start,
            ending,
        } = self;
        w.writer_escapable().write_char('<')?;
        write!(w.writer(), "{}{}", start, tag)?;
        w.writer().write_char(' ')?;
        attr.render(&mut w.as_attr_write())?;
        write!(w.writer(), " {}", ending)?;
        w.writer_escapable().write_str(">")?;
        Ok(())
    }
}

impl<D: fmt::Display> Single<D, (), &'static str, &'static str> {
    pub fn new(tag: D) -> Self {
        Single {
            tag,
            attr: (),
            start: "",
            ending: "/",
        }
    }
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

impl<D: fmt::Display, A: Attr> Locked for Element<D, A> {}

impl<D: fmt::Display, A: Attr> Element<D, A> {
    pub fn with<AA: Attr>(self, attr: AA) -> Element<D, AA> {
        Element {
            tag: self.tag,
            attr,
        }
    }
    pub fn with_map<AA: Attr, F: FnOnce() -> AA>(self, attr: F) -> Element<D, AA> {
        let attr = attr();
        self.with(attr)
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
impl<D: fmt::Display> Element<D, ()> {
    pub fn new(tag: D) -> Self {
        Element { tag, attr: () }
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
    pub fn new<E: Elem + Locked>(elem: E) -> Result<Self, fmt::Error> {
        let mut head = String::new();
        let mut tail = String::new();
        let t = elem.render_head(&mut ElemWrite::new(&mut head))?;
        t.render(&mut ElemWrite::new(&mut tail))?;
        head.shrink_to_fit();
        tail.shrink_to_fit();
        Ok(BufferedElem { head, tail })
    }

    pub fn into_parts(self) -> (String, String) {
        (self.head, self.tail)
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
impl<'a> Locked for &'a BufferedElem {}

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

impl RenderTail for () {
    fn render(self, _: &mut ElemWrite) -> std::fmt::Result {
        Ok(())
    }
}
