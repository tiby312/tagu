use super::*;

pub fn raw<D: fmt::Display>(data: D) -> Raw<D> {
    Raw { data }
}

pub fn raw_escapable<D: fmt::Display>(data: D) -> RawEscapable<D> {
    RawEscapable { data }
}

pub struct Raw<D> {
    data: D,
}

impl<D: fmt::Display> RenderElem for Raw<D> {
    type Tail = ();
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        use std::fmt::Write;
        //TODO write one global function
        write!(crate::tools::escape_guard(w), " {}", self.data)?;
        Ok(())
    }
}

pub struct RawEscapable<D> {
    data: D,
}

impl<D: fmt::Display> RenderElem for RawEscapable<D> {
    type Tail = ();
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        use std::fmt::Write;
        //TODO write one global function
        write!(w, " {}", self.data)?;
        Ok(())
    }
}

pub struct Single<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Single<D, A> {
    pub fn with_attr<AA: Attr>(self, attr: AA) -> Single<D, AA> {
        Single {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> RenderElem for Single<D, A> {
    type Tail = ();
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        use fmt::Write;
        let Single { tag, attr } = self;
        w.write_char('<')?;
        write!(crate::tools::escape_guard(&mut *w), "{}", tag)?;
        w.write_char(' ')?;
        attr.render(w)?;
        w.write_str(" />")?;
        Ok(())
    }
}

pub fn single<D: fmt::Display>(tag: D) -> Single<D, ()> {
    Single { tag: tag, attr: () }
}

pub fn elem<D: fmt::Display>(tag: D) -> Elem<D, ()> {
    Elem { tag, attr: () }
}

#[derive(Copy, Clone)]
pub struct ElemTail<D> {
    tag: D,
}

impl<D: fmt::Display> RenderTail for ElemTail<D> {
    fn render(self, w: &mut MyWrite) -> std::fmt::Result {
        use fmt::Write;
        w.write_str("</")?;
        write!(tools::escape_guard(&mut *w), "{}", &self.tag)?;
        w.write_char('>')
    }
}

#[derive(Copy, Clone)]
pub struct Elem<D, A> {
    tag: D,
    attr: A,
}

impl<D: fmt::Display, A: Attr> Elem<D, A> {
    pub fn with_attr<AA: Attr>(self, attr: AA) -> Elem<D, AA> {
        Elem {
            tag: self.tag,
            attr,
        }
    }
}
impl<D: fmt::Display, A: Attr> RenderElem for Elem<D, A> {
    type Tail = ElemTail<D>;
    fn render_head(self, w: &mut MyWrite) -> Result<Self::Tail, fmt::Error> {
        let Elem { tag, attr } = self;

        use fmt::Write;
        w.write_char('<')?;
        write!(crate::tools::escape_guard(&mut *w), "{}", tag)?;
        w.write_char(' ')?;
        attr.render(w)?;
        w.write_str(" >")?;

        Ok(ElemTail { tag })
    }
}
