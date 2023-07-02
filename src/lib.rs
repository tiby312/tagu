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
pub mod render;

use build::elem;
use elem::*;
use render::PrettyFmt;
use tools::WriteWrap;

use crate::build::single;

pub mod prelude {
    //! The hypermelon prelude
    pub use super::attrs;
    pub use super::elem::Elem;
    pub use super::elems;
    pub use super::format_move;
}

pub struct Sentinel {
    _p: (),
}

pub struct Doop<E, O> {
    elem: E,
    last: O,
}

pub struct MyWrite<W, F, T> {
    writer: W,
    fmt: F,
    inner: T,
}

impl<W: fmt::Write, F: render::Fmt, T> MyWrite<W, F, T> {
    pub fn put<E: Elem>(&mut self, elem: E) -> fmt::Result {
        let ee = &mut ElemWrite(WriteWrap(&mut self.writer), &mut self.fmt);
        let tail = elem.render_head(ee)?;
        tail.render(ee)
    }
    pub fn push<E: Elem>(
        mut self,
        elem: E,
    ) -> Result<MyWrite<W, F, Doop<E::Tail, T>>, fmt::Error> {
        let tail = elem.render_head(&mut ElemWrite(WriteWrap(&mut self.writer), &mut self.fmt))?;

        Ok(MyWrite {
            writer: self.writer,
            fmt: self.fmt,
            inner: Doop {
                elem: tail,
                last: self.inner,
            },
        })
    }
}
impl<W: fmt::Write, F: render::Fmt, E: ElemTail, T> MyWrite<W, F, Doop<E, T>> {
    pub fn pop(mut self) -> Result<MyWrite<W, F, T>, fmt::Error> {
        self.inner
            .elem
            .render(&mut ElemWrite(WriteWrap(&mut self.writer), &mut self.fmt))?;

        Ok(MyWrite {
            writer: self.writer,
            fmt: self.fmt,
            inner: self.inner.last,
        })
    }
}

#[test]
fn testo() {
    let mut s = String::new();
    session(&mut s, PrettyFmt::new(), |o| {
        let mut k = o.push(elem("svg"))?.push(elem("foo"))?;
        k.put(single("ya"))?;
        k.pop()?.pop()
    })
    .unwrap();
    println!("{}", s);
    panic!();
}

pub fn session<F: render::Fmt, W: fmt::Write>(
    writer: W,
    fmt: F,
    func: impl FnOnce(MyWrite<W, F, Sentinel>) -> Result<MyWrite<W, F, Sentinel>, fmt::Error>,
) -> fmt::Result {
    let k = MyWrite {
        writer,
        fmt,
        inner: Sentinel { _p: () },
    };
    let _ = func(k)?;
    Ok(())
}

///
/// Render elements to a writer
///
pub fn render<E: Elem + Locked, W: fmt::Write>(elem: E, writer: W) -> fmt::Result {
    render::Renderer::new().render(elem, writer)
}

///
/// Render elements to a writer that allows for escaping elements.
///
pub fn render_escapable<E: Elem, W: fmt::Write>(elem: E, writer: W) -> fmt::Result {
    render::Renderer::new().render_escapable(elem, writer)
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
