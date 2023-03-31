Build xml / html / svg programmatically by chaining structs together or by closures. Instead of using a templating engine, write data/markup that 'looks like' rust. Use has control over formatting via the `inline()` function.

You can find hypermelon on [github](https://github.com/tiby312/hypermelon) and [crates.io](https://crates.io/crates/hypermelon).
Documentation at [docs.rs](https://docs.rs/hypermelon)


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

    let style = build::elem("style")
        .inline()
        .append(".test{fill:none;stroke:white;stroke-width:3}");

    let svg = build::elem("svg").with(attrs!(
        ("xmlns", "http://www.w3.org/2000/svg"),
        ("viewBox", format_move!("0 0 {} {}", width, height))
    ));

    let rows = (0..50).step_by(10).map(|r| {
        let a = if r % 20 == 0 {
            build::single("circle")
                .with(attrs!(("cx", 50.0), ("cy", 50.0), ("r", r)))
                .some()
        } else {
            None
        };

        let b = if r % 20 != 0 {
            build::single("rect")
                .with(attrs!(("width", 30.0), ("height", 30.0)))
                .some()
        } else {
            None
        };

        a.chain(b)
    });

    let table = build::elem("g")
        .with(("class", "test"))
        .append(build::from_iter(rows));

    let all = svg.append(style).append(rect).append(table);

    hypermelon::render(all, hypermelon::stdout_fmt())
}

```

### Output

### Text:
```html
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
	<style> .test{fill:none;stroke:white;stroke-width:3}</style>
	<rect x1="0" y1="0" rx="20" ry="20" width="100" height="100" style="fill:blue"/>
	<g class="test">
		<circle cx="50" cy="50" r="0"/>
		<rect width="30" height="30"/>
		<circle cx="50" cy="50" r="20"/>
		<rect width="30" height="30"/>
		<circle cx="50" cy="50" r="40"/>
	</g>
</svg>
```
#### Image:

<img src="./assets/svg_example.svg" alt="demo">


See other example outputs at [https://github.com/tiby312/hypermelon/tree/main/assets](https://github.com/tiby312/hypermelon/tree/main/assets)



### Which method to use?

You can append elements via building of long adaptor chains, or you can render
elements to a writer on the fly. With chaining,
you don't have to worry about handling errors because nothing actually gets written out
as you're chaining. 
You can mix and match because you can make elements from closures and then chain those elements together.

### Inline function

By default tags insertion newlines and tabs. If you call `inline()` on an element, all elements
within it will be inlined. 

### Is there escape XML protection?

Attributes are fed through a escape protectors. Tag names are fed through escape protectors. 
User can bypass this using the `raw_escpapable()` or `from_closure_escapable()` functions. This returns the only element type that doesnt implement `elem::Locked`.
`render()` requires that the chained together element implements `Locked`. If the user chains in a raw element, the whole
chain will not implement `Locked`. Instead the user would have to use `render_escapable()`. The element chaining system works by having each element implement a `render_head()`, and a `render_tail()` function.

If you want to implement your own custom `Elem` outside of this crate, you can safefully implement `Locked`. This crate does not expose an api that allows you to make an `Elem` that isnt locked. 


### What happened to the tagger crate?

I left the tagger crate alone and made this into a brand new crate because while it does have all
the functionality of tagger, it is more complicated. Some people might just like the simplicity of tagger. However, I recommend people choose hypermelon over tagger, because I think its a lot more flexible. The ability to pass around element chains like structs is really useful in my experience.

### Name origin?

So its not easy to find crate names these days. A lot of good ones are taken. This one started out as ht-melon because it has "html" in the name, but it just looked jarring in the code everywhere so I changed it to hypermelon.