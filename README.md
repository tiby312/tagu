

So the tagger crate is a nice and simple crate and might be good enough for your usecase.
hypermelon provides the same capability as tagger, but additionally
has an api that lets you use adaptors to keep chaining and appending 
elements together. 


## Example

```rust
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

    let style = build::elem("style").append(".test{fill:none;stroke:white;stroke-width:3}");

    let svg = build::elem("svg").with(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let rows = (0..50)
        .step_by(10)
        .map(|r| build::single("circle").with(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r))));

    let table = build::elem("g")
        .with(("class", "test"))
        .append(build::from_iter(rows));

    let all = svg.append(style).append(rect).append(table);

    hypermelon::render(all, hypermelon::stdout_fmt())
}
```

### Output

<img src="./assets/svg_example.svg" alt="demo">




### Which method to use?

You can append elements via building of long adaptor chains, or you can render
elements to a writer on the fly. There are pros and cons to both. With chaining,
you don't have to worry about handling errors because nothing actually gets written out
as you're chaining. A downside is that you can't build elements differently based on a condition
as you go. This is because if you have an if statement, for example, the types returned by each block have to be the same.
So you can't have one block return 2 elements, and another block return 3 elements.

Basically its a tradeoff between more flexibility in writing building blocks (if conditions/loops etc), and flexibility in passing
the build blocks around. You can mix and match because you can make elements from closures and then chain those elements together.


### Is there escape XML protection?

Attributes are fed through a escape protectors. Tag names are fed through escape protectors. User can bypass this by using `build::raw_escapable()` and `AttrWrite::writer_escapable()`. The element chaining system works by having each element implement a render_head(), and a render_tail() function. This means a user can easily only call render_head() and deliberately not call render_tail().

