# Making a new change

`jj` is a very flexible tool, but in this section, we're going to show the
simplest possible workflow. If you're a fan of building up small commits via the
`git` index, we'll learn how to do that in the next chapter. Baby steps!

If you remember from the end of the last section, we're on an empty change.
You can double check with `jj status` (or `jj st`):

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:29:32}} 

So what is a change, anyway? It is the core primitive you'll be working with in
`jj`. We'll talk about that actually means in Part 2. For now, you can think of
a change as a commit. There are some differences, but now is not the time for
talking, but for action! Let's do some work.

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

We can see we've modified `src/main.rs`. Whenever we run a `jj` command, `jj`
will snapshot all of the changes that we've made to any files in our repository
and add those differences to the change we're working on. If you're a `git`
user, you may be already wondering about something like `git`'s index.  Don't
worry, even though `jj` always adds the contents of files into your changes, you
don't lose the staging area. We'll talk about that soon.  Also, you can turn
this off, but I strongly suggest you give it a try. I thought I would hate it,
but now I love it.

## Using `jj commit`

Let's say we're happy with the contents of this change. We're done, and we want
to start working on something else. To do that, we can use `jj commit`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:41:43}} 

Easy enough! Our working copy is now on a fresh new change, and its parent
is our "Goodbye, world!" change that we just committed.

To see our changes in context, let's look at `jj log` again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:45:52}} 

You can see that we're currently working on an empty change. It has `x` as a
change ID, but there's also a little `@` there: `@` is an alias for the working
copy, that is, whatever change `@` is on is the one that we're currently working
on.

Its parent is our "Goodbye, world!" change, `t`. Its parent is `p`, the "Hello, world"
change I included in the repository we cloned down.

In the next section, we'll make a pull request for this change!