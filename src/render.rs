//!
//! Functionality to change rendering pretty print vs none.
//!

use super::*;

pub struct Renderer {
    fmt: inline::InlineController,
}
impl Renderer {
    pub fn new() -> Self {
        Renderer {
            fmt: inline::InlineController::new(),
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    // pub fn with_fmt<K: Fmt>(self, a: K) -> Renderer<K> {
    //     Renderer { fmt: a }
    // }
    pub fn render<E: Elem + Locked, W: fmt::Write>(
        &mut self,
        elem: E,
        mut writer: W,
    ) -> fmt::Result {
        ElemWrite(WriteWrap(&mut writer), &mut self.fmt).render_inner(elem)?;
        self.fmt.reset_for_tail(&mut writer)
    }
    pub fn render_escapable<E: Elem, W: fmt::Write>(
        &mut self,
        elem: E,
        mut writer: W,
    ) -> fmt::Result {
        let mut e = ElemWrite(WriteWrap(&mut writer), &mut self.fmt);
        let tail = elem.render_head(e.borrow_mut2())?;
        tail.render(e)?;
        self.fmt.reset_for_tail(&mut writer)
    }
}

// pub trait Fmt {
//     fn push(&mut self);
//     fn pop(&mut self);
//     fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
//     fn end_tag(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
//     fn set_inline_mode(&mut self, val: bool);
//     fn is_inline_mode(&mut self) -> bool;
// }

// pub struct PrettyFmt {
//     tabs: usize,
//     tab_char: &'static str,
//     inline: bool,
// }

// impl Default for PrettyFmt {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl PrettyFmt {
//     pub fn new() -> Self {
//         PrettyFmt {
//             tabs: 0,
//             tab_char: "\t",
//             inline: false,
//         }
//     }
// }
// impl PrettyFmt {
//     pub fn with_tab(self, tab: &'static str) -> PrettyFmt {
//         PrettyFmt {
//             tabs: self.tabs,
//             tab_char: tab,
//             inline: self.inline,
//         }
//     }
// }

pub mod inline {
    #[derive(Clone, Debug)]
    pub enum InlineSigl {
        Inline,
        Pretty,
    }
    impl InlineSigl {
        fn as_bool(&self) -> bool {
            match self {
                InlineSigl::Inline => true,
                InlineSigl::Pretty => false,
            }
        }
        // fn clone(&self)->Self{
        //     match self{
        //         InlineSigl::Inline=>InlineSigl::Inline,
        //         InlineSigl::Pretty=>InlineSigl::Pretty
        //     }
        // }
    }
    use super::*;
    pub struct InlineController {
        pub inline: InlineSigl,
        pub ignore_tab:bool,
        pub tabs: isize,
        tab_char: &'static str,
        pub extra:Option<()>
    }
    impl InlineController {
        pub fn new() -> Self {
            InlineController {
                inline: InlineSigl::Pretty,
                tabs: 0,
                tab_char: "\t",
                extra:None,
                ignore_tab:false
            }
        }



        pub fn start(
            &mut self,
            w: &mut dyn fmt::Write,
            inline: InlineSigl,
        ) -> Result<InlineSigl, fmt::Error> {
            let og=self.inline.clone();

            match inline{
                InlineSigl::Inline=>{}
                InlineSigl::Pretty=>{
                    match self.inline{
                        InlineSigl::Inline=>{}
                        InlineSigl::Pretty=>{
                            self.inline = inline;

                            self.tabs(w)?;
                            //writeln!(w)?;
                            self.tabs += 1;
                        }
                    }
                }
            }
            //dbg!(&self.inline,&inline);
            // match self.inline {
            //     InlineSigl::Inline => {},
            //     InlineSigl::Pretty => match inline {
            //         InlineSigl::Inline => {
            //             self.inline = inline.clone();
                        
            //         }
            //         InlineSigl::Pretty => {
            //             self.inline = inline;

            //             self.tabs(w)?;
            //             writeln!(w)?;
            //             self.tabs += 1;
                        
            //         }
            //     },
            // }
            Ok(og)
        }

        pub fn end(&mut self, w: &mut dyn fmt::Write, original_inline: InlineSigl) -> fmt::Result {
            match (&original_inline, &self.inline) {
                (InlineSigl::Pretty,_) => {
                    // if let InlineSigl::Pretty=k{
                    //     //dbg!("hay");
                    //     writeln!(w)?;
                    //     self.tabs(w)?;
                    // }
                    // writeln!(w)?;
                    // self.tabs(w)?;
                    self.tabs -= 1;
                }
                _ => {}
            }
            self.inline = original_inline;

            Ok(())
        }
        pub fn reset_for_tail(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
            if let InlineSigl::Pretty = &self.inline {
                writeln!(w)?;
                //write!(w,"[[{}]]",self.tabs)?;
                //self.tabs(w)?;
                
            }
            Ok(())
        }
        pub fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
            if let InlineSigl::Pretty = &self.inline {
                if !self.ignore_tab{
                    for _ in 0..self.tabs {
                        write!(w, "{}", self.tab_char)?;
                    }
                }
            }

            Ok(())
        }
    }
}

// impl Fmt for PrettyFmt {
//     fn set_inline_mode(&mut self, val: bool) {
//         self.inline = val;
//     }
//     fn is_inline_mode(&mut self) -> bool {
//         self.inline
//     }
//     fn tabs(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
//         if !self.inline {
//             for _ in 0..self.tabs {
//                 write!(w, "{}", self.tab_char)?;
//             }
//         }

//         Ok(())
//     }
//     fn push(&mut self) {
//         //if !self.inline {
//         self.tabs += 1;
//         //}
//     }
//     fn pop(&mut self) {
//         //if !self.inline {
//         self.tabs -= 1;
//         //}
//     }
//     fn end_tag(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
//         if !self.inline {
//             writeln!(w)?;
//         }
//         Ok(())
//     }
// }

// pub struct NoFmt;
// impl Fmt for NoFmt {
//     fn tabs(&mut self, _: &mut dyn fmt::Write) -> fmt::Result {
//         Ok(())
//     }
//     fn push(&mut self) {}
//     fn pop(&mut self) {}
//     fn end_tag(&mut self, _: &mut dyn fmt::Write) -> fmt::Result {
//         Ok(())
//     }
//     fn set_inline_mode(&mut self, _: bool) {}

//     fn is_inline_mode(&mut self) -> bool {
//         true
//     }
// }
