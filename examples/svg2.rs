use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let all = build::from_stack(|w| {
        let mut w = w.push(build::elem("svg").with(attrs!(
            ("xmlns", "http://www.w3.org/2000/svg"),
            ("viewBox", format_move!("0 0 {} {}", width, height))
        )))?;

        w.put(
            build::elem("style")
                .append(".test{fill:none;stroke:white;stroke-width:3}")
                .inline(),
        )?;

        w.put(build::single("rect").with(attrs!(
            ("x1", 0),
            ("y1", 0),
            ("rx", 20),
            ("ry", 20),
            ("width", width),
            ("height", height),
            ("style", "fill:blue")
        )))?;

        let mut w = w.push(build::elem("g").with(("class", "test")))?;

        for r in (0..50).step_by(5) {
            if r % 10 == 0 {
                let c = build::single("circle").with(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r)));
                w.put(c)?;
            } else {
                let r = build::single("rect").with(attrs!(
                    ("x", 50 - r),
                    ("y", 50 - r),
                    ("width", r * 2),
                    ("height", r * 2)
                ));
                w.put(r)?;
            }
        }

        w.pop()?.pop()
    });

    tagu::render(all, tagu::stdout_fmt())
}
