use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let rect = build::single("rect").with(attrs!(
        ("x1", 0),
        ("y1", 0),
        ("rx", 20),
        ("ry", 20),
        ("width", width),
        ("height", height),
        ("style", "fill:blue")
    ));

    let style = build::elem("style")
        .inline()
        .append(".test{fill:none;stroke:white;stroke-width:3}");

    let svg = build::elem("svg").with(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let rows = build::from_stack(|mut f| {
        for r in (0..50).step_by(5) {
            if r % 10 == 0 {
                let c = build::single("circle").with(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r)));
                f.put(c)?;
            } else {
                let r = build::single("rect").with(attrs!(
                    ("x", 50 - r),
                    ("y", 50 - r),
                    ("width", r * 2),
                    ("height", r * 2)
                ));
                f.put(r)?;
            }
        }
        Ok(f)
    });

    let table = build::elem("g").with(("class", "test")).append(rows);

    let all = svg.append(style).append(rect).append(table);

    hypermelon::render(all, hypermelon::stdout_fmt())
}
