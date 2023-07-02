use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let a = build::elem("a1");
    let b = build::elem("b1");
    let c = build::elem("c1");
    let it = build::from_iter((0..5).map(|i| build::elem(format_move!("x1:{}", i)).inline()));
    let all = a.append(b.append(c.append(it)));

    hypermelon::render(all, hypermelon::stdout_fmt())
}
