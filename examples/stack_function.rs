use std::fmt;
use tagu::build;
use tagu::prelude::*;
use tagu::stack::ElemStack;

fn func<T>(stack: ElemStack<T>) -> Result<ElemStack<T>, fmt::Error> {
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
    let all = build::from_stack(|stack| func(stack.push(build::elem("ha"))?)?.pop());

    tagu::render(all, tagu::stdout_fmt())
}
