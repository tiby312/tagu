use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let a = build::elem("a");
    let b = build::elem("b");
    let c = build::elem("c");
    let it = build::from_iter((0..5).map(|i| build::elem(format_move!("x{}", i)).inline()));
    let all = a.append(b.append(c.append(it)));

    tagu::render(all, tagu::stdout_fmt())
}
