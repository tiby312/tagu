use room::build;
use room::prelude::*;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let rect = build::single("rect").with_attr(attrs!(
        ("x1", 0),
        ("y1", 0),
        ("rx", 20),
        ("ry", 20),
        ("width", width),
        ("height", height),
        ("style", "fill:blue")
    ));

    let style =
        build::elem("style").append(build::raw(".test{fill:none;stroke:white;stroke-width:3}"));

    let svg = build::elem("svg").with_attr(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let rows = (0..50)
        .step_by(10)
        .map(|r| build::single("circle").with_attr(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r))));

    let table = build::elem("g").with_attr(("class", "test")).append(rows);

    let all = svg.append(style).append(rect).append(table);

    all.render_with(room::stdout_fmt())
}
