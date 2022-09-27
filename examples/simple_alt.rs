use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let k = &mut hypermelon::stdout_fmt();
    let mut w = hypermelon::ElemWrite::new(k);

    let svg = build::elem("svg").with(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let style =
        build::elem("style").append(build::raw(".test{fill:none;stroke:white;stroke-width:3}"));

    let rect = build::single("rect").with(attrs!(
        ("x1", 0),
        ("y1", 0),
        ("rx", 20),
        ("ry", 20),
        ("width", width),
        ("height", height),
        ("style", "fill:blue")
    ));

    let table = {
        let rows = (0..50)
            .step_by(10)
            .map(|r| build::single("circle").with(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r))));

        build::elem("g")
            .with(("class", "test"))
            .append(build::from_iter(rows))
    };

    w.render_with(svg).build(|w| {
        w.render(style)?;

        w.render(rect)?;

        w.render(table)
    })
}
