

So the tagger crate is a nice and simple crate and might be good enough for your usecase.
hypermelon provides the same capability as tagger, but additionally
has an api that lets you use adaptors to keep chaining and appending 
elements together. Its a more complicated api, but more flexible.


# Which method to use?

You can append elements via building of long adaptor chains, or you can render
elements to a writer on the fly. There are pros and cons to both. With chaining,
you don't have to worry about handling errors because nothing actually gets written out
as you're chaining. A downside is that you can't build elements differently based on a condition
as you go. This is because if you have an if statement, for example, the types returned by each block have to be the same.
So you can't have one block return 2 elements, and another block return 3 elements.

Basically its a tradeoff between more flexibility in writing building blocks (if conditions/loops etc), and flexibility in passing
the build blocks around.


