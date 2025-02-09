# Interacting with GitHub

Let's take one last look at that `jj log` output:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:45:52}} 

Do you see that little `trunk` over on the right there? That is a *bookmark*
in `jj`, and it's how `jj` understands git branches. `trunk` is the name of
the default branch of the git repository we pulled down from GitHub.
Bookmarks are semantically closer to `git` tags than `git` branches, but since
branches and tags are very similar in `git`, it works just fine.

We made our change on top of `trunk`, but we never created a branch! `jj`
allows our branches to be anonymous. That's great when we're working
locally, but when we interact with GitHub, it needs a branch name.

To create a bookmark, we can use `jj bookmark`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:54:55}} 

`jj bookmark create` takes a name for the bookmark, and then we also pass a `-r` flag.
This is short for "revision," which is a sort of catch-all name for the various kinds
of IDs we can use to refer to changes. We could have also used `t`, for example, which is
the change ID. In this case, we pass `@-`, which means "the parent of `@`."

Let's look at our log:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:57:64}} 

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
send me. Feel free to do so anyway if you want! But, if this was a PR that
eventually got merged, it's easy to pull the changes down: 

```console
$ jj git fetch
Nothing changed.
```

If there were changes, we'd have some output describing what happens.

With that, you have the basics down! In the next section, we'll talk about what
we just did in a bit more depth, so you'll be prepared to learn some more
advanced topics. 