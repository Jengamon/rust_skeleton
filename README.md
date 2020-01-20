# Rust Skeleton for MIT Pokerbots 2020

This is skeleton code in Rust for the MIT Pokerbots competition.
The team only actually support C/++, Python, and Java, but I thought changing
things up would be kind of fun.

Rust can be a surprising pain to compile offline in a manner of seconds, but
when it actually does compile, it's nice.

This crate provides:
  - A multithreaded runner for your bots. (might have a few issues, but it should work)
    - [yes, it does technically follow competition rules, because it blocks on read, and panics when it can't write]
  - A non-string representation of Cards and Actions, which you can freely Copy all over the place
    - [the card values do follow regular poker ordering when directly comparing them]
  - A hand calculation engine that can calculate hands for arbitrary orderings
    - [my piece-de-resistance. this was the most fun to code. guess which part wasn't]

This code does use 4 relatively light dependencies:
  - approx
  - bitflags
  - itertools [only required if you want to use the hand calculation engine]
  - log [to replace my very hacky debug_println code]

These compile in a manner of 2-3 seconds, so that should be fun~

## What happened during the competiton?

The skeleton code is designed to not get in your way when trying to compile,
so it should be build in minimal time.

This code will build in anywhere from 6 seconds [debug] to 13 seconds [release, fully optimized].
These times include all the dependencies, from building their source, optimizing them, and linking them into objects.
The competition only gives 10 seconds. With such a wide variance on build time depending
solely on optimization, I personally relied on:

```toml
[profile.release]
opt-level = 0
```

but that isn't a good story for runtime performance. I could go up to opt-level 2,
while taking about 13 seconds, but my build step would always time out...

To be honest, multithreading the runner was the worst idea ever. But I've done it,
and I worked too hard on it to want to change it. The biggest problem is that
the format the server sends is very order dependent, as you can see from the
"PreservedOrdering" enum. About the only thing that you can do as soon as you see it
is change the game clock. This is really nice for the server, because that means
it can play a game and send just what happened, but for multithreading the code,
that means I have to guarantee the ordering of updates. I ran into a bunch of
problems when I didn't do that, but luckily, I had already written my bot, and
it worked on the synchronous runner (although not exactly the best), so I knew
it wasn't problems with the bot, but with state updates. However, it is pretty nice,
because I/O is done on separate threads, so I can manage somewhere around 113 rounds/s
at opt-level 2, even if the ThreadPool is configured to use a single thread. I can't
say anything on performance on a single-core machine, but then the effects of the ThreadPool
might be negligible for such a difficult task.

In my opinion now, Rust is a great language for correctness and dependency, but
it's compilation speed leaves much to be desired. This hopefully changes in the future,
but for now, I'm out~

As a side note: Yes, the runner panics a lot, but if I handled those errors,
I have no idea how I would handle panics in the bot. I know I'm lazy, but panics
are stupid easy, and you have to handle Results. Maybe I'll rewrite the Runner
more robustly and only panic if the bot actually returns an Err, but for
now, this is good enough.
