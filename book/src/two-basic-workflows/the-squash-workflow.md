# The Squash Workflow

So, in the introduction to this tutorial, I made the claim that `jj` is simpler
yet just as powerful as `git` is. But I also just told you that `jj` does not have
an index. Well, the thing is, we can still do what the index lets us do with `git`,
it's just that it's a workflow rather than a feature. Let's give it a try.

If you recall, we are currently sitting on an empty change:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:57:64}} 

Earlier, we used `jj commit -m` to give a commit message to `@` and create a new
change. Let's do that again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:67:70}} 

Now we have a change that represents our work, and `@` is a new change with that
one as its parent.

We can make a change to `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
    println!("Goodbye, world!");
}
```

We're treating `@` kind of like a index in this sense. Our work isn't going
into our feature just yet. Let's do that now:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:74:76}} 

This command moves the contents of `@` into our parent. You can see that our
current commit is now empty, and our parent has some changes, that is, the
`(empty)` isn't there any more.

If you like doing `git add -p` to include only parts of the diff into the index,
you can `jj squash -i`, and it'll bring up a little terminal editor you can use
to select which parts of the diff you'd like to move. You can even click on the
menus!

With this, we've shown how you can have the equivalent of a index if you want
one, without needing "the index" as a distinct concept. 

### Why does this matter?

Basically, `jj` has managed to provide equal functionality, while removing the
concept of "the index" as an independent feature entirely. This may not seem like a
huge deal, and while it's not a massive one, it does have a number of other
implications that aren't immediately obvious.

Because we don't have a index, we can do that auto-snapshotting of your
working directory, removing the distinction between the working copy and a change.
This makes it easy to not accidentally lose work. You know how people say it's
hard to cause data loss in git? They're not wrong, but with `jj`, it's even
moreso. Having the working copy, index, and commit contents be three
different things, where only one of them is permanently stored, means you can
run commands that end up losing data. Have you ever run `git reset --hard` and
realized that there was something in your index you forgot to save first?
It's now gone forever. with `jj`, 

Because the index is just a commit, we can use all of the tools that
we use to transform commits onto our index. `jj squash` is not fundamentally
about the index pattern: it's about moving portions of the diff one change
represents into another change. Later in this book, we'll use `jj squash` for a
similar, but different purpose.

Because we don't have a index, various commands don't need to account for
it in their behavior. `git reset` has `--soft`, `--mixed`, and `--hard` modes
(and a few more...), all of which do different things. This is because `git reset`,
as a command that modifies the current branch head, needs to know what to do with
the working copy and index, and so you need flags for each permutation of
behavior someone may want to make use of.