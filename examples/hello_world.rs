use hypermelon::build;
use hypermelon::elem::Locked;
use hypermelon::prelude::*;
use hypermelon::stack::ElemStack;


// Elements are chained together to be rendered later.
fn adaptor_example() -> impl Elem + Locked {
    let a = build::elem("a1");
    let b = build::elem("b1");
    let c = build::elem("c1");
    let it = build::from_iter((0..5).map(|i| build::elem(format_move!("x1:{}", i)).inline()));
    a.append(b.append(c.append(it)))
}

// Elements are rendered on the fly requiring error handling.
fn stack_example<T>(stack: ElemStack<T>) -> Result<ElemStack<T>, std::fmt::Error> {
    let a = build::elem("a2");
    let b = build::elem("b2");
    let c = build::elem("c2");

    let mut stack = stack.push(a)?.push(b)?.push(c)?;

    for i in 0..5 {
        let e = build::elem(format_move!("x2:{}", i)).inline();
        stack.put(e)?;
    }
    stack.pop()?.pop()?.pop()
}

fn main() -> std::fmt::Result {
    let all = build::from_stack(|mut w| {
        w.put(adaptor_example())?;

        let mut w = stack_example(w)?;

        w.put(build::raw("Here can't escape html: <foo>"))?;
        Ok(w)
    });

    hypermelon::render(all, hypermelon::stdout_fmt())
}
