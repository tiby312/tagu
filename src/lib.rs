pub mod build;
pub mod tools;
use std::fmt;

pub mod prelude {
    pub use super::attrs;
    pub use super::elems;
    pub use super::format_move;
    pub use super::Attr;
    pub use super::RenderElem;
}

#[must_use]
pub struct WriteWrap<'a>(pub &'a mut dyn fmt::Write);

impl<'a> WriteWrap<'a> {
    pub fn new<W: fmt::Write>(w: &'a mut W) -> Self {
        WriteWrap(w)
    }

    pub fn render<E: RenderElem>(&mut self, elem: E) -> fmt::Result {
        elem.render_all(self)
    }

    pub fn session<'b, E: RenderElem>(&'b mut self, elem: E) -> SessionStart<'b, 'a, E> {
        SessionStart { elem, writer: self }
    }
}

#[must_use]
pub struct SessionStart<'a, 'b, E> {
    elem: E,
    writer: &'a mut WriteWrap<'b>,
}

impl<'a, 'b, E: RenderElem> SessionStart<'a, 'b, E> {
    pub fn build(self, func: impl FnOnce(&mut WriteWrap) -> fmt::Result) -> fmt::Result {
        let SessionStart { elem, writer } = self;
        let tail = elem.render_head(writer)?;
        func(writer)?;
        tail.render(writer)
    }
}

impl fmt::Write for WriteWrap<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.0.write_char(c)
    }
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {
        self.0.write_fmt(args)
    }
}

pub trait Attr {
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result;
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
    fn render(self, _: &mut WriteWrap) -> std::fmt::Result {
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
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result {
        let AttrChain { first, second } = self;
        use fmt::Write;
        first.render(w)?;
        w.write_str(" ")?;
        second.render(w)
    }
}

impl<A: fmt::Display, B: fmt::Display> Attr for (A, B) {
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result {
        let (first, second) = self;
        use fmt::Write;
        write!(crate::tools::escape_guard(&mut *w), " {}", first)?;
        w.write_str("=\"")?;
        write!(crate::tools::escape_guard(&mut *w), "{}", second)?;
        w.write_str("\"")
    }
}

pub trait RenderTail {
    fn render(self, w: &mut WriteWrap) -> std::fmt::Result;
}

impl RenderTail for () {
    fn render(self, _: &mut WriteWrap) -> std::fmt::Result {
        Ok(())
    }
}

pub trait RenderElem {
    type Tail: RenderTail;
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error>;

    fn render_with<W: fmt::Write>(self, mut w: W) -> fmt::Result
    where
        Self: Sized,
    {
        self.render_all(&mut WriteWrap(&mut w))
    }
    /// Render head and tail.
    fn render_all(self, w: &mut WriteWrap) -> fmt::Result
    where
        Self: Sized,
    {
        let next = self.render_head(w)?;
        next.render(w)
    }

    /// Render all of Self and head of other, store tail of other.
    fn chain<R: RenderElem>(self, other: R) -> Chain<Self, R>
    where
        Self: Sized,
    {
        Chain {
            top: self,
            bottom: other,
        }
    }

    /// Render head of Self, and all of other, store tail of self.
    fn append<R: RenderElem>(self, bottom: R) -> Append<Self, R>
    where
        Self: Sized,
    {
        Append { top: self, bottom }
    }
}

pub fn hide_impl<R: RenderElem>(a: R) -> impl RenderElem {
    a
}
#[must_use]
#[derive(Copy, Clone)]
pub struct Append<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem, B: RenderElem> RenderElem for Append<A, B> {
    type Tail = A::Tail;
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        let Append { top, bottom } = self;
        let tail = top.render_head(w)?;
        bottom.render_all(w)?;
        Ok(tail)
    }
}
#[must_use]
#[derive(Copy, Clone)]
pub struct Chain<A, B> {
    top: A,
    bottom: B,
}

impl<A: RenderElem, B: RenderElem> RenderElem for Chain<A, B> {
    type Tail = B::Tail;
    fn render_head(self, w: &mut WriteWrap) -> Result<Self::Tail, fmt::Error> {
        let Chain { top, bottom } = self;
        top.render_all(w)?;
        bottom.render_head(w)
    }
}

#[test]
fn test_svg() {
    let potato = build::elem("potato");
    let chicken = build::elem("chicken").with_attr(("a", "a").chain(("b", "b")));
    let html = build::elem("html").with_attr(("a", "a"));

    let k = html.append(chicken.chain(potato));
    //let k=html.append(potato).append(chicken);
    //let html = elem("html", crate::empty_attr);

    let mut w = crate::tools::upgrade_write(std::io::stdout());
    k.render_all(&mut WriteWrap(&mut w)).unwrap();
    println!();
}

#[macro_export]
macro_rules! attrs {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::Attr;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}

#[macro_export]
macro_rules! elems {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::RenderElem;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}

pub fn stdout_fmt() -> tools::Adaptor<std::io::Stdout> {
    tools::upgrade_write(std::io::stdout())
}
