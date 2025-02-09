# Interacting with GitHub

Breifly, all of these instructions should work with any of the various
`git` forges out there, but since GitHub is very popular, I'm going
to talk about the topic with GitHub as the specific example. No shade to the
other hosts, I just feel that trying to speak about this in the abstract
would make it harder to understand.

Let's take one last look at that `jj log` output:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:43:50}} 

Do you see that little `trunk` over on the right there? That is a *bookmark*
in `jj`, and it's how `jj` understands git branches. `trunk` is the name of
the default branch of the git repository we pulled down from GitHub. (I prefer
`trunk` to `main` or `master` these days, so that's why I did this, but those
are of course popular as well.) `jj` doesn't really have "named branches" like
`git` does, but it does have bookmarks, and `jj` will create a bookmark for each 
branch in the underlying `git` repository.

We made our change on top of `trunk`, but we never created a branch! `jj`
allows our branches to be anonymous. That's great when we're working
locally, but when we interact with GitHub, it needs a branch name.

To create a bookmark, we can use `jj bookmark`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:52:53}} 

`jj bookmark create` takes a name for the bookmark, and then we also pass a `-r` flag.
This is short for "revision," which is a sort of catch-all name for the various kinds
of IDs we can use to refer to changes. We could have also used `t`, for example, which is
the change ID. In this case, we pass `@-`, which means "the parent of `@`."

Let's look at our log:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:55:62}} 

We can now see `goodbye-world` listed on the right. Great! Let's push that up
to GitHub:

```console
$ jj git push --allow-new
Changes to push to origin:
  Add bookmark goodbye-world to e2dd22df5f5d
remote: 
remote: Create a pull request for 'goodbye-world' on GitHub by visiting:
remote:      https://github.com/<YOUR USERNAME>/hello-world/pull/new/goodbye-world
remote: 
```

A `jj git push` will push up all of our changes. In this case, we have our new
bookmark, which has turned into a `git` branch. Doing a `jj git push` is kind
of like doing a `git push --force-with-lease`, and so `jj` wants us to confirm
that we intend to create a new branch by passing `--allow-new`.

Because this is a tutorial repository, I won't be merging any pull requests you
send me. Feel free to do so anyway if you want!

Let's pretend that this PR was merged. What exactly that looks like depends on
if your upstream does a merge, a rebase, or a squash merge. Let's talk about the
default case for now, a merge. 

The first thing we need to do is add a new remote. Right now, we have one: origin.
I like to use the name "upstream" for the repositories I've forked, but you can
name them whatever you'd like. We can do that like this:

```console
$ jj git remote add upstream https://github.com/jj-tutorial/hello-world
```

No output. That's vaguely ominous, but I promise, you're fine. Let's pull in
the changes:

```console
$ jj git fetch --remote upstream
remote: Enumerating objects: 1, done.
remote: Counting objects: 100% (1/1), done.
remote: Total 1 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
bookmark: trunk@upstream [new] untracked
```

It tells us we have a new bookmark: `trunk@upstream`, and it's not tracked.
What does that mean? If something changes in upstream, and we do another
fetch, we won't see the changes. By default, `jj` doesn't automatically
track branches. So let's tell it we care about this bookmark:

```console
$ jj bookmark track trunk@upstream
Started tracking 1 remote bookmarks.
```

Great! Let's see what our history looks like now:

```console
❯ jj log
@  kozrnusy steve@steveklabnik.com 2025-02-09 11:48:43 a1be631e
│  (empty) (no description set)
│ ○  oltlpuxu steve@steveklabnik.com 2025-02-09 11:42:09 trunk* 82008a30
╭─┤  (empty) Merge pull request #4 from steveklabnik/goodbye-world
○ │  zourloqr steve@steveklabnik.com 2025-02-09 11:40:46 goodbye-world git_head() b8f74a89
├─╯  Goodbye, world
◆  ptrqnyzv steve@steveklabnik.com 2024-09-23 19:43:36 trunk@origin 0c72abbb
│  Hello, world!
~
```

That's quite different! We can see that `o` is a merge commit, because it
has two parents, `z` and `p`. Our working copy is still on top of `z`.

If we take a closer look at `trunk`, you'll notice it's changed to `trunk*`.
Since we have a local branch named `trunk`, and we fetched `trunk@upstream`,
it has updated our bookmark to match. We can see `trunk@origin` is where
`trunk` used to be. That `*` is there to remind us that our local `trunk`
doesn't line up with `trunk@origin`, so we may want to push it. Let's
do that:

```console
$ jj git push -b trunk
Changes to push to origin:
  Move forward bookmark trunk from 0c72abbb8365 to 82008a304090
remote:
```

`-b` is short for bookmark. Let's look at our history now:

```console
❯ jj log
@  kozrnusy steve@steveklabnik.com 2025-02-09 11:48:43 a1be631e
│  (empty) (no description set)
│ ◆  oltlpuxu steve@steveklabnik.com 2025-02-09 11:42:09 trunk 82008a30
╭─┤  (empty) Merge pull request #4 from steveklabnik/goodbye-world
│ │
│ ~
│
◆  zourloqr steve@steveklabnik.com 2025-02-09 11:40:46 goodbye-world git_head() b8f74a89
│  Goodbye, world
~
```

We have a new bit of output in the log, a tilde (`~`). It's been at the bottom of our
log this whole time, but you probably only really noticed it now, once the second one
appeared under our merge there. This tilde means that there are
changes that are currently not being shown. `jj log` tries to be helpful, and by default
will show you... well, it's a bit complex for now, let's just say that it tries to show
you helpful context around where you're working currently. You can always pass it options
to show something else, but I don't want to get further sidetracked here. Our working
copy is still based off of where we were before we did all this remote stuff. Let's
create a new change on top of `trunk`:

```console
$ jj new trunk
Working copy now at: rzownqqx 569855c9 (empty) (no description set)
Parent commit      : oltlpuxu 82008a30 trunk | (empty) Merge pull request #4 from steveklabnik/goodbye-world
```

Don't worry about our old change; because it was empty and had no description, `jj`
automatically abandons it, so you won't have a bunch of empty stuff literring up your
repository.

We'll talk about it more in the next section, but we're using `new` here in a way similar
to "checking out a branch" in other DVCSes. 

Let's take a look at our log:

```console
@  rzownqqx steve@steveklabnik.com 2025-02-09 12:29:32 569855c9
│  (empty) (no description set)
◆  oltlpuxu steve@steveklabnik.com 2025-02-09 11:42:09 trunk git_head() 82008a30
│  (empty) Merge pull request #4 from steveklabnik/goodbye-world
~
```

Where'd everything go?!? Well, everything is in harmony: since our local `trunk`
agrees with both `origin` and `upstream`, `jj` only bothers to show us `trunk`.
Our `goodbye-world` bookmark still exists:

```console
$ jj bookmark list
goodbye-world: zourloqr b8f74a89 Goodbye, world
trunk: oltlpuxu 82008a30 (empty) Merge pull request #4 from steveklabnik/goodbye-world
```

But since it's behind `trunk`, `jj log` doesn't bother to show it.

What happens when our `upstream`'s `trunk` changes. The next time we `fetch`:

```console
❯ jj git fetch --remote upstream
remote: Enumerating objects: 7, done.
remote: Counting objects: 100% (7/7), done.
remote: Compressing objects: 100% (2/2), done.
remote: Total 4 (delta 1), reused 0 (delta 0), pack-reused 0 (from 0)
bookmark: trunk@upstream [updated] tracked
```

It'll pull in those changes. Here's our `log` now:

```console
$ jj log
@  rzownqqx steve@steveklabnik.com 2025-02-09 12:29:32 569855c9
│  (empty) (no description set)
│ ○  wuxwwlxm steve@steveklabnik.com 2025-02-09 12:38:31 trunk* 9fbe724a
├─╯  Update main.rs
◆  oltlpuxu steve@steveklabnik.com 2025-02-09 11:42:09 trunk@origin git_head() 82008a30
│  (empty) Merge pull request #4 from steveklabnik/goodbye-world
~
```

This is the same as when our pull request got merged, except since `w` isn't a
merge commit, things look simpler.

One last thing: in this sort of scenario, where you have `origin` as a fork
of `upstream`, it's common to configure `jj` so that `jj git fetch` pulls
from `upstream` by default, and `jj git push` pushes to `origin` by default.

We can do that via `jj config`. `jj` has per-repository settings as well as
per-user settings. I think this one is more appropriate for this repsitory
only, so:

```console
$ jj config edit --repo
```

This will open up your local configuration in an editor. It should have this
in it:

```toml
[revset-aliases]
"trunk()" = "trunk@origin"
```

We can ignore this for now. You can add this below that stuff:

```toml
[git]
fetch = "upstream"
push = "origin"
```

And now we have default remotes for `jj git fetch` and `jj git push`. If you
work on your fork from multiple computers, you may want to do this instead:

```toml
[git]
fetch = ["upstream", "origin"]
push = "origin"
```

And now it'll fetch from both. Up to you.

With that, you have the basics down! There's a lot more to talk about of course: how
to respond to pull request feedback, what to do if your upstream squash merges,
and rebasing your work on top of an updated upstream all come to mind. We'll get
there. But you now know how to do the simplest possible version of working with
GitHub.

In the next section, we'll talk about what
we just did in a bit more depth, so you'll be prepared to learn some more
advanced topics. 