use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let a = elem("a");
    let b = elem("b");
    let c = elem("c");
    let it = (0..5).map(|i| elem(format!("x{}", i)).inline());

    let c = tagu::util::comment("this is comment");

    tagu::render(elems!(all,m,c), tagu::stdout_fmt())
}
