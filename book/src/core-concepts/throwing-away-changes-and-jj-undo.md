# Throwing away changes & jj undo

We've talked a lot about how to make changes, but haven't talked about getting
rid of them!

Getting rid of changes is very easy in `jj`: we can do it with `jj abandon`.

If you remember, we're in the middle of some stuff:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:114:118}}

If you are coming to this section fresh, just type `jj new` a few times to
give yourself some good changes. Done? Great. Let's throw them away.

### `jj restore` to reset contents

Let's say we don't like that "hello and goodbye world" stuff. We're not going
to pursue that further. Getting rid of it is as easy as:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:121:124}}

By default, `jj restore` takes changes from your parent change, and puts them
into `@`. But there's `--from` and even `--into` flags you can pass as well.
Let's grab the diff from our first commit, and apply it to `@`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:127:130}}

As you can see we aren't empty any more. Well, what does our code look like?
Let's use `jj diff` to see:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:133:137}}

This format is different than `git`'s: we have red and green to indicate
what's changed, for example.

If you want to get a `git` style diff instead, that is easy as well:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:140:149}}

We only had one file that was changed, so we didn't *need* to pass the path to
`jj restore`, but `jj restore` is mostly used with individual paths.  If we
passed no arguments to `jj restore`, it would `restore` every file, that is,
move the entire diff from your parent to `@`, effectively emptying out the
change.

But what if we want to delete a change entirely?

### `jj abandon`

At any time, you can get rid of a change with `jj abandon`. It's tons of fun!
Let's try it:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:152:157}}

This throws away our current change. We abandoned `opqvmvrn`, and since that
was the same as `@`, `jj` makes a new change for us, in this case, called
`nvnlxpxw`.

But what if we abandon something that's not `@`? Like, let's say, `t`, the
change that we're currently on top of. What's the worst that could happen?

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:160:167}}

So what happened here?

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:170:175}}

As you can see, because we got rid of the commit we were standing on, instead of throwing
us away too, `jj` just re-parented us onto the abandoned commit's parent. We're still on
change `nvnlxpxw`, but now our parent is `ptrqnyzv`, not `tnmounps`.

But what if that was a mistake? What if we didn't actually want to throw away `tnmounps`,
and we regret our actions here?

I have good news.

### `jj undo`

There's a really useful subcommand that goes by `jj undo`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:178:182}}

That's it! We're good again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:185:192}}

Everything is back to where we put it. We can always `jj undo` to undo
*any* of our last operations, and `jj` will make things right again.
Don't underestimate how good this feels: you can really try out things
and not worry about messing up the state of the world, it's very freeing.
It even brought our bookmark back!

There is one funny thing about `jj undo` I feel compelled to mention,
though. What do you think would happen if we `jj undo`'d again right now?

Make your guess, and then give it a try:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:195:201}}

That's right: the last thing you did was an `undo`, so an `undo` just
`undo`es that `undo`. Hilarious, but kind of frustrating. There's a
desire to let you go back an arbitrary number of `undo`s, but it's
a bit trickier than it sounds.

Regardless, we can fix this: there's no problem with `jj undo` that you
can't solve by throwing more `jj undo`s at it:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:204:210}}

Whew. That's enough of that.

### Automatically abandoning changes

Having an empty change with no description is fine to have if it's
`@`, or if it has children. Here's a fun party trick:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:213:217}}

That's right: `jj new` can take `--before` or `--after` flags to
squish a change in between others. (Yes, we're trying to make squish happen.)
And `--no-edit` means that we don't want to move our working copy
to the new change: `@` stays right where it is:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:220:229}}

So that change is fine. But what if we move away from these changes?
Let's make a new change on top of `trunk`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:232:235}}

We had two empty commits on top of `goodbye-world` before, but
what about now?

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:238:247}}

Our empty change `nvnlxpxw` was discarded, automatically. You don't have to
worry about `jj new` littering up your repository, empty changes will end up
abandoned.
