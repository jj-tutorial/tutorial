# Making a new change

`jj` is a very flexible tool, but in this section, we're going to show the
simplest possible workflow. If you're a fan of building up small commits via the
`git` index, we'll learn how to do that in the next chapter. Baby steps!

If you remember from the end of the last section, we're on an empty change.
You can double check with `jj status`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:29:32}} 

`jj st` is an alias to make that a bit easier.

## Changes and commits

If you look closely in the output we just saw, you'll see four identifiers:

```text
wsxvoskz 3471e1d7
snwusnyo 308fcf55
```

In `git`, we often talk about commits and their ID numbers. The
`3471e1d7` and `308fcf55` bits in the above output are in fact commit IDs. What
about `wsxvoskz` and `snwusnyo` though? Those are called "change ID"s. `jj` has
a concept called a "change", and it's like a commit that evolves over time. This
change ID will remain stable over the life of the change, but every time we
modify it, we'll see a new commit ID. In a sense, a change represents the
evolution of a commit over time.

Let's see how that works by modifying a file. Our change ID is `wsxvoskz` and our
commit is `3471e1d7`. Let's modify a file and see what happens.

## Modifying a file

Here's the contents of `src/main.rs`. Don't worry, you won't need to actually
know Rust to complete this tutorial, we just want some code to work with:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's change that to something else:

```rust
fn main() {
    println!("Goodbye, world!");
}
```

A bit fatalistic, but it works. Let's run `jj st` again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:35:40}} 

We can see we've modified `src/main.rs`. Whenever we run a `jj` command,
it updates the contents of `@`, the commit that represents the working copy,
to contain all of the changes we've made.

We still see our change ID `wsxvoskz`, but now, instead of `3471e1d7`, we have
`3a23a995`. Our change ID is stable, but now that we've evolved it by making
changes to our working directory, we get a new commit that represents this
latest state. This is a bit of a mental shift from `git`! In `git`, once we
have a commit, it's "done," unless we decide to modify it later. With `jj`,
changes aren't just for finished work: they're also for tracking in-progress
work. We'll talk more about mutable vs immutable changes eventually, but for
now, just know that changes and commits are two different things, and that
a change represents the evolution of a commit over time, giving it a stable
identifier that we can talk about.

## Using `jj commit`

Let's say we're happy with the contents of this change. We want to
finish up this work, and start something else. To do that, we can use `jj commit`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:41:43}} 

To see our changes in context, let's look at `jj log` again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:45:52}} 

We can see that `@` is now on a new, empty change. And we have `wsxvoskz`
as its parent.

In the next section, we'll make a pull request for this change!