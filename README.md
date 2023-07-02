Build xml / html / svg programmatically by chaining structs together or by closures. Instead of using a templating engine, write data/markup that 'looks like' rust. User has control over formatting via the `inline()` function.

You can find hypermelon on [github](https://github.com/tiby312/hypermelon) and [crates.io](https://crates.io/crates/hypermelon).
Documentation at [docs.rs](https://docs.rs/hypermelon)


### Adaptor Example:

```rust
use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let a = build::elem("a1");
    let b = build::elem("b1");
    let c = build::elem("c1");
    let it = build::from_iter((0..5).map(|i| build::elem(format_move!("x1:{}", i)).inline()));
    let all = a.append(b.append(c.append(it)));

    hypermelon::render(all, hypermelon::stdout_fmt())
}

```

### Output Text:
```
<a1>
    <b1>
        <c1>
            <x1:0></x1:0>
            <x1:1></x1:1>
            <x1:2></x1:2>
            <x1:3></x1:3>
            <x1:4></x1:4>
        </c1>
    </b1>
</a1>
```

## Stack Example

```rust
use hypermelon::build;
use hypermelon::prelude::*;

fn main() -> std::fmt::Result {
    let all = build::from_stack(|stack| {
        let a = build::elem("a2");
        let b = build::elem("b2");
        let c = build::elem("c2");

        let mut stack = stack.push(a)?.push(b)?.push(c)?;

        for i in 0..5 {
            let e = build::elem(format_move!("x2:{}", i)).inline();
            stack.put(e)?;
        }
        stack.pop()?.pop()?.pop()
    });

    hypermelon::render(all, hypermelon::stdout_fmt())
}

```

### Output Text:
```html
<a2>
    <b2>
        <c2>
            <x2:0></x2:0>
            <x2:1></x2:1>
            <x2:2></x2:2>
            <x2:3></x2:3>
            <x2:4></x2:4>
        </c2>
    </b2>
</a2>
 ```




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
User can bypass this using the `raw_escpapable()` or `from_stack_escapable()` functions. This returns the only element type that doesnt implement `elem::Locked`.
`render()` requires that the chained together element implements `Locked`. If the user chains in a raw element, the whole
chain will not implement `Locked`. Instead the user would have to use `render_escapable()`. The element chaining system works by having each element implement a `render_head()`, and a `render_tail()` function.

### What happened to the tagger crate?

I left the tagger crate alone and made this into a brand new crate because while it does have all
the functionality of tagger, it is more complicated. Some people might just like the simplicity of tagger. However, I recommend people choose hypermelon over tagger, because I think its a lot more flexible. The ability to pass around element chains like structs is really useful in my experience.

### Name origin?

So its not easy to find crate names these days. A lot of good ones are taken. This one started out as ht-melon because it has "html" in the name, but it just looked jarring in the code everywhere so I changed it to hypermelon.