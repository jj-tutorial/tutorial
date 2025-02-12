# Throwing away changes & jj undo

We've talked a lot about how to make changes, but haven't talked about getting
rid of them!

Getting rid of changes is very easy in `jj`: we can do it with `jj abandon`.

If you remember, we're in the middle of some stuff:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:116:126}} 

If you are coming to this section fresh, just type `jj new` a few times to
give yourself some good changes. Done? Great. Let's throw them away.

### `jj restore` to reset contents

Let's say we don't like that "hello and goodbye world" stuff. We're not going
to pursue that further. Getting rid of it is as easy as:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:127:131}} 

By default, `jj restore` takes changes from your parent change, and puts them
into `@`. But there's `--from` and even `--into` flags you can pass as well.
Let's grab the diff from our first commit, and apply it to `@`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:133:138}} 

As you can see we aren't empty any more. Well, what does our code look like?
Let's use `jj diff` to see:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:139:144}} 

This format is different than `git`'s: the line number not being in the right
column but being in the left column means that this line is removed in this
diff.

If you want to get a `git` style diff instead, that is easy as well:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:146:155}} 

We only had one file that was changed, so we didn't *need* to pass the path, but
`jj restore` is mostly used with individual paths.  If we passed no arguments to
`jj restore`, it would `restore` every file, that is, move the entire diff from
your parent to `@`, effectively emptying out the change.

But what if we want to delete a change entirely?

### `jj abandon`

At any time, you can get rid of a change with `jj abandon`. It's tons of fun!
Let's try it:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:116:126}} 

You know what? Let's get rid of this "hello and goodbye world" nonsense. Sure,
we're standing on top of this change, but if you have a wrecking ball, isn't it
more fun to smash the thing down the middle? Let's get rid of `x`. What's the worst
that could happen?

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:157:171}} 

Oh. That doesn't look good. What did we do?

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:173:180}} 

As you can see, because we got rid of the commit we were standing on, instead of throwing
us away too, `jj` just re-parented us onto the abandoned commit's parent. And we have
some sort of problem with our diffs, `jj` ran into some issues trying to apply the contents
of our change onto our parent.

Thankfully, I have some good news.

### `jj undo`

There's a really useful subcommand that goes by `jj undo`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:182:188}} 

That's it! We're good again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:190:199}} 

Everything is back to where we put it. We can always `jj undo` to undo
*any* of our last operations, and `jj` will make things right again.
Don't underestimate how good this feels: you can really try out things
and not worry about messing up the state of the world, it's very freeing.

There is one funny thing about `jj undo` I feel compelled to mention,
though. What do you think would happen if we `jj undo`'d again right now?

Make your guess, and then give it a try:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:202:214}} 

That's right: the last thing you did was an `undo`, so an `undo` just
`undo`es that `undo`. Hilarious, but kind of frustrating. There's a
desire to let you go back an arbitrary number of `undo`s, but it's
a bit trickier than it sounds.

Regardless, we can fix this: there's no problem with `jj undo` that you
can't solve by throwing more `jj undo`s at it:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:218:222}} 

Whew. That's enough of that.

### Automatically abandoning changes

Having an empty change with no description is fine to have if it's
`@`, or if it has children. Here's a fun party trick:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:226:231}} 

That's right: `jj new` can take `--before` or `--after` flags to
squish a change in between others. (Yes, we're trying to make squish happen.)
And `--no-edit` means that we don't want to move our working copy
to the new change: `@` stays right where it is:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:232:242}} 

So that change is fine. But what if we destroy `@`? Well, that's
what `jj abandon` does by default:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:246:249}} 

Since we threw away `@`, what should our working copy be? `jj abandon`
will make us a new change on top of our old parent, that way we
wouldn't end up accidentally mutating any changes we weren't trying
to modify.

This is also true any time we move `@` away from a commit that's empty, has no
description, and no parents: `jj` will get rid of it for us. `jj new` liberally!
You won't litter up your history with a bunch of empty changes.