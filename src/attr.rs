use super::*;

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
        use fmt::Write;
        first.render(w)?;
        w.writer().write_str(" ")?;
        second.render(w)
    }
}

pub struct AttrWrite<'a>(pub(crate) WriteWrap<'a>);
impl<'a> AttrWrite<'a> {
    pub fn render<E: Attr>(&mut self, attr: E) -> fmt::Result {
        attr.render(self)
    }
    pub fn writer(&mut self) -> tools::EscapeGuard<WriteWrap> {
        tools::escape_guard(self.0.borrow_mut())
    }

    pub(crate) fn writer_escapable(&mut self) -> WriteWrap {
        self.0.borrow_mut()
    }
}
