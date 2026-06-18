use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let a = build::elem("a");
    let b = build::elem("b");
    let c = build::elem("c");
    let it = (0..5).map(|i| build::elem(format!("x{}", i)).inline());
    let all = build::from_iter(it).insert(c).insert(b).insert(a);
    
    let m = build::from_stack(|s|{
        let mut s=s.push(build::elem("stack"))?;
        s.put(build::elem("test").append(build::elem("inner")))?;
        s.pop()
    });

    let c = tagu::util::comment("this is comment");

    tagu::render(elems!(all,m,c), tagu::stdout_fmt())
}
