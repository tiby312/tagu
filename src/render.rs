//!
//! Functionality to change rendering pretty print vs none.
//!

use super::*;
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

impl Default for Renderer<PrettyFmt<'static>> {
    fn default() -> Self {
        Self::new()
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

pub trait Fmt {
    fn push(&mut self);
    fn pop(&mut self);
    fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
    fn end_tag(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
    fn set_inline_mode(&mut self, val: bool);
    fn is_inline_mode(&mut self) -> bool;
}

pub struct PrettyFmt<'a> {
    tabs: usize,
    tab_char: &'a str,
    inline: bool,
}

impl Default for PrettyFmt<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl PrettyFmt<'static> {
    pub fn new() -> Self {
        PrettyFmt {
            tabs: 0,
            tab_char: "\t",
            inline: false,
        }
    }
}
impl<'a> PrettyFmt<'a> {
    pub fn with_tab(self, tab: &str) -> PrettyFmt {
        PrettyFmt {
            tabs: self.tabs,
            tab_char: tab,
            inline: self.inline,
        }
    }
}

impl Fmt for PrettyFmt<'_> {
    fn set_inline_mode(&mut self, val: bool) {
        self.inline = val;
    }
    fn is_inline_mode(&mut self) -> bool {
        self.inline
    }
    fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
        if !self.inline {
            for _ in 0..self.tabs {
                write!(w, "{}", self.tab_char)?;
            }
        }

        Ok(())
    }
    fn push(&mut self) {
        //if !self.inline {
        self.tabs += 1;
        //}
    }
    fn pop(&mut self) {
        //if !self.inline {
        self.tabs -= 1;
        //}
    }
    fn end_tag(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
        if !self.inline {
            writeln!(w)?;
        }
        Ok(())
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
    fn set_inline_mode(&mut self, _: bool) {}

    fn is_inline_mode(&mut self) -> bool {
        true
    }
}
