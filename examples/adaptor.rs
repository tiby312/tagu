use tagu::build::{elem, from_iter};
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let a = elem("a");
    let b = elem("b");
    let c = elem("c");
    let it = (0..5).map(|i| elem(format_move!("x{}", i)).inline());

    let all = from_iter(it).insert(c).append(a).insert(b).insert(a);

    tagu::render(all, tagu::stdout_fmt())
}
