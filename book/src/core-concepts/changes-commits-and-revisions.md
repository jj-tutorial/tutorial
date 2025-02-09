# Changes, commits, and revisions

If there's one concept that's at the heart of `jj`, that would be the *change*.
Let's talk about them, as well as two related concepts: *commit*s and
*revision*s.

## Changes

Changes serve the same pupose, conceptually, that commits do in `git`: they're a
snapshot of your project, and the graph of these changes forms a sort of
timeline of the history of the project.

So why not call them commits? Well, there's a big difference between changes in
`jj` and commits in `git`, and that's because `git`'s commits are immutable, whereas
(by default) changes are mutable. Let's talk about how we use these tools to
compare and contrast the two.

### Creating Changes vs Commits

I really like this description from [Pro Git]:

> The basic Git workflow goes something like this:
> 
> * You modify files in your working tree.
> * You selectively stage just those changes you want to be part of your next
>   commit, which adds only those changes to the staging area.
> * You do a commit, which takes the files as they are in the staging area and
>   stores that snapshot permanently to your Git directory.

"Creating a commit" is an action that you do after you've got things set up the
way you want them to be saved. You can use the index to do this incrementally
if you'd like.

If we were to talk about `jj` in a similar way, I'd say this:

> The basic `jj` workflow goes something like this:
> 
> * You create a new change
> * You modify files in your working tree.
> * There is no step 3.

We are always working within the context of some change. `jj` refers to the
change we're working on as `@`. Whenever you run a `jj` command, it will sync up
your working tree and `@`. You can also ask [Watchman] to do this
synchronization every time you save a file.

This means that in some sense, you work "backwards" in `jj` from `git`'s
perspective: it's not "modify files in your working tree, produce a commit"
it's "make a new change, modify files in your working tree your working
tree."

### Descriptions

We haven't really talked about descriptions yet. We've used `jj commit`
to give some of our changes descriptions, but as you've seen, `jj new`
is totally fine with creating a new change without one. We can use
`jj describe` to give `@` a description. This will open up your `$EDITOR`
and works very similar to `git commit`. If you give a lengthy description
to a commit, `jj` will use its first line as the description in `jj log`.
Just as with `git`, it's recommended that you create a relatively short
first line for this reason.

### Change IDs

You may have noticed change IDs use purely letters instead of letters and
numbers. This is deliberate, and in fact, doesn't include `a` through `f`,
so you cannot confuse a change ID for a commit ID.

### Immutable Changes

While changes are mutable, there are some instances when you'd prefer not to
mutate one. `jj` considers some commits immutable, and if you try to mutate
one, it will refuse unless you pass `--ignore-immutable` as an argument.

Let's look at `jj log`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:81:90}} 

We have the green `@`, but its parent instead has a `○`. This change
is mutable. But what about `p`? It has a `◆`. This change is immutable.

`jj` considers a change immutable if it's part of your `trunk`/`main`/`master`
branch, if it's got a `git` tag, if it's an untracked remote bookmark.
You can override this, but I don't recommend it.

Oh, and the root change is also immutable. Let's talk about that!

### The root change

One interesting thing about `jj` is that there's a special change that
exists in every repository, the root change. While `git` allows for
multiple root commits, `jj` does not: every change other than the root
change must have a parent change. This simplifies a lot of the various
algorithms used to modify history, but the specifics aren't important
to us right now.

We can use `jj show` to show information about changes. The root change
has a change ID of all `z`s, let's check it out:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:93:99}} 

Pretty fun. Wait, what's that `Commit ID` doing there? Okay, let's
talk about commits.

[Pro Git]: https://git-scm.com/book/en/v2/Getting-Started-What-is-Git%3F
[Watchman]: https://facebook.github.io/watchman/

## Commits

So we've been talking about how changes are different than `git`'s commits...
but `jj` also has commits. Let me explain. Let's use `jj st` to look at our
current change:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:76:79}} 

Do you see how we have four identifiers there?

```text
skrvmwst f1d3d6c2
xlpymuzl af847aae
```

The two on the left are change IDs, but the two on the right are commit IDs.
What are commits for? Well, whenever you modify a change, that has to be stored
in `git` somehow. And since `git`'s commit IDs are based on (among other things)
the contents of the commit, when you create a new git commit, you're also going
to get a different ID. Here, let's give it a try: modify `src/main.rs`, in any
way that you'd like. Then we'll run `jj st` again:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:103:107}} 

`s` used to have a commit ID of `f1d3d6c2`, but now it's `df7c31c8`. Our change
ID remains stable, but the commit ID will change over time.

This is very powerful! Part 4 of the tutorial is titled "Fixing Problems," and
a lot of the stuff we will talk about there is powered by commits. We can
use `jj evolog`, the "evolution log," to see how a change has evolved over time:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:109:114}} 

There are a lot of flags to control `jj evolog`'s output. I've chosen the
`summary` flag here to show which files we changed.

## Revisions

The term *revision* is sometimes used as well. "revision" is a synonym for "commit."
It's a slightly more generic term.

The `jj` project is still working out which terminology is best, and so sometimes,
you'll find things like this. In particular, there's a desire to turn these three
concepts into only two, but it's not clear which word ends up being best, so for
now, there's three of them.