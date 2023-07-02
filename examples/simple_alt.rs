use hypermelon::build;
use hypermelon::prelude::*;
use hypermelon::stack::ElemStack;

fn test<T>(a: ElemStack<T>) -> Result<ElemStack<T>, std::fmt::Error> {
    let a = a.push(build::elem("svg"))?;
    let a = a.push(build::elem("svg"))?;
    let a = a.push(build::elem("svg"))?;
    a.pop()?.pop()?.pop()
}

fn main() -> std::fmt::Result {
    let width = 100.0;
    let height = 100.0;

    let all = build::render_stack(|w| {
        let mut w = w.push(build::elem("svg").with(attrs!(
            ("xmlns", "http://www.w3.org/2000/svg"),
            ("viewBox", format_move!("0 0 {} {}", width, height))
        )))?;

        w.put(
            build::elem("style")
                .append(".test{fill:none;stroke:white;stroke-width:3}")
                .inline(),
        )?;

        let mut w = test(w)?;

        w.put(build::single("rect").with(attrs!(
            ("x1", 0),
            ("y1", 0),
            ("rx", 20),
            ("ry", 20),
            ("width", width),
            ("height", height),
            ("style", "fill:blue")
        )))?;

        let rows = (0..50)
            .step_by(10)
            .map(|r| build::single("circle").with(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r))));

        let e = build::elem("g")
            .with(("class", "test"))
            .append(build::from_iter(rows));

        w.put(e)?;

        w.pop()
    });

    hypermelon::render(all, hypermelon::stdout_fmt())
}
