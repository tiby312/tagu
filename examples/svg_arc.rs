use hypermelon::build;
use hypermelon::prelude::*;

use hypermelon::attr::PathCommand::*;
fn main() -> std::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let svg = build::elem("svg").with(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let path1 = build::single("path").with(attrs!(
        ("stroke", "black"),
        ("stroke-width", 2),
        ("fill", "green"),
        ("fill-opacity", 0.5),
        build::path([M(200, 120), Q(300, 50, 400, 120), T(500, 120)])
    ));

    let path2 = build::single("path").with(attrs!(
        ("stroke", "black"),
        ("stroke-width", 2),
        ("fill", "blue"),
        ("fill-opacity", 0.5),
        build::path([M(300, 200), H_(-150), A_(150, 150, 0, 1, 0, 150, -150), Z()])
    ));

    let all = svg.append(path1).append(path2);

    hypermelon::render(all, hypermelon::stdout_fmt())
}
