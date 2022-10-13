//! Build xml / html / svg programmatically by chaining structs together or by closures. Instead of using a templating engine, write data/markup that 'looks like' rust.
//!
//! You can find hypermelon on [github](https://github.com/tiby312/hypermelon) and [crates.io](https://crates.io/crates/hypermelon).
//! Documentation at [docs.rs](https://docs.rs/hypermelon)

pub mod tools;
use std::fmt;
pub mod attr;
pub mod build;
pub mod elem;
use attr::*;

use elem::*;
use tools::WriteWrap;

pub mod prelude {
    //! The hypermelon prelude
    pub use super::attrs;
    pub use super::elem::Elem;
    pub use super::elems;
    pub use super::format_move;
}

pub fn renderer() -> Renderer<PrettyFmt<'static>> {
    Renderer::new()
}
pub struct Renderer<D: Fmt> {
    fmt: D,
}
impl Renderer<PrettyFmt<'static>> {
    pub fn new() -> Self {
        Renderer {
            fmt: PrettyFmt::new(),
        }
    }
}
impl<D: Fmt> Renderer<D> {
    pub fn with_fmt<K: Fmt>(self, a: K) -> Renderer<K> {
        Renderer { fmt: a }
    }
    pub fn render<E: Elem + Locked, W: fmt::Write>(
        &mut self,
        elem: E,
        mut writer: W,
    ) -> fmt::Result {
        ElemWrite(WriteWrap(&mut writer), &mut self.fmt).render(elem)
    }
    pub fn render_escapable<E: Elem, W: fmt::Write>(
        &mut self,
        elem: E,
        mut writer: W,
    ) -> fmt::Result {
        let e = &mut ElemWrite(WriteWrap(&mut writer), &mut self.fmt);
        let tail = elem.render_head(e)?;
        tail.render(e)
    }
}

///
/// Render elements to a writer
///
pub fn render<E: Elem + Locked, W: fmt::Write>(elem: E, writer: W) -> fmt::Result {
    Renderer::new().render(elem, writer)
}

///
/// Render elements to a writer that allows for escaping elements.
///
pub fn render_escapable<E: Elem, W: fmt::Write>(elem: E, writer: W) -> fmt::Result {
    Renderer::new().render_escapable(elem, writer)
}

///
/// An std out that implements fmt::Write
///
pub fn stdout_fmt() -> tools::Adaptor<std::io::Stdout> {
    tools::upgrade_write(std::io::stdout())
}

///
/// call `Elem::append()` without having to have Elem in scope.
///
pub fn append<R: Elem, K: Elem>(a: R, k: K) -> Append<R, K> {
    a.append(k)
}

///
/// Chain together a list of attrs
///
#[macro_export]
macro_rules! attrs {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::attr::Attr;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}

///
/// Chain together a list of elements
///
#[macro_export]
macro_rules! elems {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::elem::Elem;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*

            a
        }
    };
}

pub trait Fmt {
    fn push(&mut self);
    fn pop(&mut self);
    fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
    fn end_tag(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
}

pub struct PrettyFmt<'a> {
    tabs: usize,
    tab_char: &'a str,
}
impl PrettyFmt<'static> {
    pub fn new() -> Self {
        PrettyFmt {
            tabs: 0,
            tab_char: "\t",
        }
    }
}
impl<'a> PrettyFmt<'a> {
    pub fn with_tab<'b>(self, tab: &'b str) -> PrettyFmt<'b> {
        PrettyFmt {
            tabs: self.tabs,
            tab_char: tab,
        }
    }
}

impl Fmt for PrettyFmt<'_> {
    fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
        for _ in 0..self.tabs {
            write!(w, "{}", self.tab_char)?;
        }

        Ok(())
    }
    fn push(&mut self) {
        self.tabs += 1;
    }
    fn pop(&mut self) {
        self.tabs -= 1;
    }
    fn end_tag(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(w, "")
    }
}

pub struct NoFmt;
impl Fmt for NoFmt {
    fn tabs(&mut self, _: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }
    fn push(&mut self) {}
    fn pop(&mut self) {}
    fn end_tag(&mut self, _: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }
}
