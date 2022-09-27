use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let width = 500.0;
    let height = 400.0;

    let svg = build::elem("svg").with(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let polygon = build::elem("polygon").with(attrs!(
        ("stroke", "black"),
        ("stroke-width", 2),
        ("fill", "green"),
        ("fill-opacity", 0.5),
        build::points([(100, 100), (200, 100), (300, 300), (100, 200)])
    ));

    let all = svg.append(polygon);

    hypermelon::render(all, hypermelon::stdout_fmt())
}
