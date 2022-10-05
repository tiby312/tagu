use super::*;

#[must_use]
pub struct ElemWrite<'a>(pub(crate) WriteWrap<'a>);

impl<'a> ElemWrite<'a> {
    pub fn writer_escapable(&mut self) -> WriteWrap {
        self.0.borrow_mut()
    }

    pub fn writer(&mut self) -> tools::EscapeGuard<WriteWrap> {
        tools::escape_guard(self.0.borrow_mut())
    }

    pub(crate) fn as_attr_write(&mut self) -> AttrWrite {
        attr::AttrWrite(self.0.borrow_mut())
    }

    pub fn new(w: &'a mut dyn fmt::Write) -> Self {
        ElemWrite(WriteWrap(w))
    }

    pub fn render<E: Elem>(&mut self, elem: E) -> fmt::Result {
        let tail = elem.render_head(self)?;
        tail.render(self)
    }

    pub fn render_with<'b, E: Elem>(&'b mut self, elem: E) -> SessionStart<'b, 'a, E> {
        SessionStart { elem, writer: self }
    }

    pub fn render_map<E: Elem, F: FnOnce() -> E>(&mut self, func: F) -> fmt::Result {
        let elem = func();
        let tail = elem.render_head(self)?;
        tail.render(self)
    }

    pub fn render_map_with<'b, E: Elem, F: FnOnce() -> E>(
        &'b mut self,
        func: F,
    ) -> SessionStart<'b, 'a, E> {
        let elem = func();
        SessionStart { elem, writer: self }
    }
}

pub trait RenderElem {
    fn render_head(&mut self, w: &mut ElemWrite) -> Result<(), fmt::Error>;
    fn render_tail(&mut self, w: &mut ElemWrite) -> Result<(), fmt::Error>;
}

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

pub struct DynElemTail<'a> {
    elem: &'a mut dyn RenderElem,
}
impl<'a> RenderTail for DynElemTail<'a> {
    fn render(self, w: &mut ElemWrite) -> std::fmt::Result {
        self.elem.render_tail(w)
    }
}
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

pub trait Elem {
    type Tail: RenderTail;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error>;

    // /// Render head and tail.
    // fn render_all(self, w: &mut ElemWrite) -> fmt::Result
    // where
    //     Self: Sized,
    // {
    //     let next = self.render_head(w)?;
    //     next.render(w)
    // }

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

#[must_use]
#[derive(Copy, Clone)]
pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: Elem, B: Elem> Elem for Append<A, B> {
    type Tail = A::Tail;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Append { top, bottom } = self;
        let tail = top.render_head(w)?;
        w.render(bottom)?;
        Ok(tail)
    }
}
#[must_use]
#[derive(Copy, Clone)]
pub struct Chain<A, B> {
    top: A,
    bottom: B,
}

impl<A: Elem, B: Elem> Elem for Chain<A, B> {
    type Tail = B::Tail;
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let Chain { top, bottom } = self;
        w.render(top)?;
        bottom.render_head(w)
    }
}

pub trait RenderTail {
    fn render(self, w: &mut ElemWrite) -> std::fmt::Result;
}

impl RenderTail for () {
    fn render(self, _: &mut ElemWrite) -> std::fmt::Result {
        Ok(())
    }
}

#[must_use]
pub struct SessionStart<'a, 'b, E> {
    elem: E,
    writer: &'a mut ElemWrite<'b>,
}

impl<'a, 'b, E: Elem> SessionStart<'a, 'b, E> {
    pub fn build(self, func: impl FnOnce(&mut ElemWrite) -> fmt::Result) -> fmt::Result {
        let SessionStart { elem, writer } = self;
        let tail = elem.render_head(writer)?;
        func(writer)?;
        tail.render(writer)
    }
}

use fmt::Write;

impl<D: fmt::Display> Elem for D {
    type Tail = ();
    fn render_head(self, w: &mut ElemWrite) -> Result<Self::Tail, fmt::Error> {
        write!(w.writer(), " {}", self)?;
        Ok(())
    }
}
