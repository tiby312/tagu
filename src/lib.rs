pub mod build;
pub mod tools;
use std::fmt;
pub mod attr;
pub mod elem;
use attr::*;

use elem::*;
use tools::WriteWrap;
pub mod prelude {
    pub use super::attrs;
    pub use super::elem::Elem;
    pub use super::elems;
    pub use super::format_move;
}

///
/// Render an element to a write
///
pub fn render<E: Elem + SafeElem, W: fmt::Write>(elem: E, mut writer: W) -> fmt::Result {
    ElemWrite(WriteWrap(&mut writer)).render(elem)
}

///
/// Render an element to a write
///
pub fn render_escapable<E: Elem, W: fmt::Write>(elem: E, mut writer: W) -> fmt::Result {
    ElemWrite(WriteWrap(&mut writer)).render(elem)
}

///
/// An std out that implements fmt::Write
///
pub fn stdout_fmt() -> tools::Adaptor<std::io::Stdout> {
    tools::upgrade_write(std::io::stdout())
}

///
/// call Elem::append() without having to have Elem in scope.
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
