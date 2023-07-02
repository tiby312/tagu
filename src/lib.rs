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

use elem::*;
use render::PrettyFmt;
use tools::WriteWrap;

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

pub struct MyWrite<'a, T> {
    writer: ElemWrite<'a>,
    inner: T,
}

// pub enum EitherA<A, B> {
//     A(A),
//     B(B),
// }
// impl<A: Pop, B: Pop> Pop for EitherA<A, B> {
//     type Curr = EitherA<A::Curr, B::Curr>;
//     type Last = EitherA<A::Last, B::Last>;
//     fn next(self) -> (Self::Curr, Self::Last) {
//         match self {
//             EitherA::A(a) => {
//                 let (a, b) = a.next();
//                 (EitherA::A(a), EitherA::A(b))
//             }
//             EitherA::B(a) => {
//                 let (a, b) = a.next();
//                 (EitherA::B(a), EitherA::B(b))
//             }
//         }
//     }
// }
// impl<A: ElemTail, B: ElemTail> ElemTail for EitherA<A, B> {
//     fn render(self, w: &mut ElemWrite) -> std::fmt::Result {
//         match self {
//             EitherA::A(a) => a.render(w),
//             EitherA::B(a) => a.render(w),
//         }
//     }
// }

impl<'a, T> MyWrite<'a, T> {
    // pub fn eithera<B>(self) -> MyWrite<'a, EitherA<T, B>> {
    //     MyWrite{writer:self.writer,inner:EitherA::A(self.inner)}
    // }
    // pub fn eitherb<A>(self) -> MyWrite<'a, EitherA<A, T>> {
    //     MyWrite{writer:self.writer,inner:EitherA::B(self.inner)}
    // }
    pub fn put<E: Elem>(&mut self, elem: E) -> fmt::Result {
        let tail = elem.render_head(self.writer.borrow_mut2())?;
        tail.render(self.writer.borrow_mut2())
    }
    pub fn push<E: Elem>(mut self, elem: E) -> Result<MyWrite<'a, Popper<E::Tail, T>>, fmt::Error> {
        let tail = elem.render_head(self.writer.borrow_mut2())?;

        Ok(MyWrite {
            writer: self.writer,
            inner: Popper {
                elem: tail,
                last: self.inner,
            },
        })
    }
}

impl<'a, P: Pop> MyWrite<'a, P> {
    pub fn pop(mut self) -> Result<MyWrite<'a, P::Last>, fmt::Error> {
        let (e, l) = self.inner.next();
        e.render(self.writer.borrow_mut2())?;

        Ok(MyWrite {
            writer: self.writer,
            inner: l,
        })
    }
}

pub struct Sess<F> {
    func: F,
}

impl<F> Locked for Sess<F> {}

impl<F> Elem for Sess<F>
where
    F: FnOnce(MyWrite<Sentinel>) -> Result<MyWrite<Sentinel>, fmt::Error>,
{
    type Tail = ();
    fn render_head(self, writer: ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let k = MyWrite {
            writer,
            inner: Sentinel { _p: () },
        };
        let _ = (self.func)(k)?;
        Ok(())
    }
}

pub fn sess<F>(func: F) -> Sess<F>
where
    F: FnOnce(MyWrite<Sentinel>) -> Result<MyWrite<Sentinel>, fmt::Error>,
{
    Sess { func }
}

// pub fn session<W: fmt::Write>(writer: W) -> SessionStarter<W, PrettyFmt> {
//     SessionStarter::new(writer)
// }

// pub struct SessionStarter<W, F> {
//     writer: W,
//     fmt: F,
// }
// impl<W: fmt::Write> SessionStarter<W, PrettyFmt> {
//     pub fn new(writer: W) -> Self {
//         SessionStarter {
//             writer,
//             fmt: PrettyFmt::new(),
//         }
//     }
// }
// impl<W: fmt::Write, F: render::Fmt> SessionStarter<W, F> {
//     pub fn build(
//         &mut self,
//         func: impl for<'a,'b> FnOnce(MyWrite<'a,'b,Sentinel>) -> Result<MyWrite<'a,'b,Sentinel>, fmt::Error>,
//     ) -> fmt::Result {
//         let writer = &mut ElemWrite(WriteWrap(&mut self.writer), &mut self.fmt);

//         let k = MyWrite {
//             writer,
//             inner: Sentinel { _p: () },
//         };
//         let _ = func(k)?;
//         Ok(())
//     }
// }

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
