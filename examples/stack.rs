use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let all = build::from_stack(|stack| {
        let a = build::elem("a2");
        let b = build::elem("b2");
        let c = build::elem("c2");

        let mut stack = stack.push(a)?.push(b)?.push(c)?;

        let k=build::elem("chicken").append(build::raw("").inline());
        stack.put(build::elem(format_move!("x2:{}", 1)).append(k))?;
        stack.put(build::elem(format_move!("x2:{}", 2)))?;
        // stack.put(build::elem(format_move!("x2:{}", 3)))?;

        stack.pop()?.pop()?.pop()
    });

    hypermelon::render(all, hypermelon::stdout_fmt())
}
