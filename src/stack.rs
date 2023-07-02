use super::*;
pub struct Sentinel {
    _p: (),
}

pub trait Pop {
    type Curr: ElemTail;
    type Last;
    fn next(self) -> (Self::Curr, Self::Last);
}
impl<E: ElemTail, O> Pop for Popper<E, O> {
    type Curr = E;
    type Last = O;
    fn next(self) -> (E, O) {
        (self.elem, self.last)
    }
}

pub struct Popper<E, O> {
    elem: E,
    last: O,
}

pub struct ElemStack<'a, T> {
    writer: ElemWrite<'a>,
    inner: T,
}

impl<'a, T> ElemStack<'a, T> {
    pub fn put<E: Elem>(&mut self, elem: E) -> fmt::Result {
        let tail = elem.render_head(self.writer.borrow_mut2())?;
        tail.render(self.writer.borrow_mut2())
    }
    pub fn push<E: Elem>(
        mut self,
        elem: E,
    ) -> Result<ElemStack<'a, Popper<E::Tail, T>>, fmt::Error> {
        let tail = elem.render_head(self.writer.borrow_mut2())?;

        Ok(ElemStack {
            writer: self.writer,
            inner: Popper {
                elem: tail,
                last: self.inner,
            },
        })
    }
}

impl<'a, P: Pop> ElemStack<'a, P> {
    pub fn pop(mut self) -> Result<ElemStack<'a, P::Last>, fmt::Error> {
        let (e, l) = self.inner.next();
        e.render(self.writer.borrow_mut2())?;

        Ok(ElemStack {
            writer: self.writer,
            inner: l,
        })
    }
}

pub struct Sess<F> {
    func: F,
}
impl<F> Sess<F>
where
    F: FnOnce(ElemStack<Sentinel>) -> Result<ElemStack<Sentinel>, fmt::Error>,
{
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Locked for Sess<F> {}

impl<F> Elem for Sess<F>
where
    F: FnOnce(ElemStack<Sentinel>) -> Result<ElemStack<Sentinel>, fmt::Error>,
{
    type Tail = ();
    fn render_head(self, writer: ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let k = ElemStack {
            writer,
            inner: Sentinel { _p: () },
        };
        let _ = (self.func)(k)?;
        Ok(())
    }
}
