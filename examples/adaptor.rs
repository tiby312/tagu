use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let a = build::elem("a");
    let b = build::elem("b");
    let c = build::elem("c");
    let it = (0..5).map(|i| build::elem(format_move!("x{}", i)).inline());
    let all = build::from_iter(it).insert(c).append(a).insert(b).insert(a);

    tagu::render(all, tagu::stdout_fmt())
}
