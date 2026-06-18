use tagu::build;
use tagu::prelude::*;

fn main() -> std::fmt::Result {
    let all = build::elem("a").append_with(|| {
        elems!(
            build::single("test"),
            build::elem("b").append_with(|| {
                let it =
                    build::from_iter((0..5).map(|i| build::elem(format_move!("x{}", i)).inline()));

                build::elem("c").append_with(|| it)
            }),
            build::elem("bbbb").append_with(|| {
                elems!(
                    tagu::util::comment("this is comment"),
                    build::single("k").with(("apple", 5))
                )
            })
        )
    });

    tagu::render(all, tagu::stdout_fmt())
}
