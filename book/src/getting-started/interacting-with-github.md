# Interacting with GitHub

In the last section, we made a new change, built on top of `trunk`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:44:51}} 

Let's make a pull request for this.

In order to make a pull request, we need a branch. Well, GitHub wants a branch.
But we haven't made any branches! Did you notice that?

`jj` lets us work without naming any branches. That's great when we're working
locally, but when we interact with GitHub, it needs a branch name. To bridge
this gap, `jj` has a feature called "bookmarks". They're closer to `git` tags
than `git` branches, but since branches are a special type of tag, they'll work.

To create a bookmark, we can use `jj bookmark`:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:53:54}} 

`jj bookmark create` takes a name for the bookmark, and then we also pass a `-r` flag.
This is short for "revision," and it means we can pass in a change ID, a commit ID,
another bookmark name... lots of things. In this case, we pass `@-`, which means
"the parent of `@`."

Let's look at our log:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:57:63}} 

We can now see `goodbye-world` listed on the right. Great! Let's push that up
to GitHub:

```console
❯ jj git push
Changes to push to origin:
  Add bookmark goodbye-world to e2dd22df5f5d
remote: 
remote: Create a pull request for 'goodbye-world' on GitHub by visiting:
remote:      https://github.com/<YOUR USERNAME>/hello-world/pull/new/goodbye-world
remote: 
```

A `jj git push` will push up all of our changes. In this case, we have our new
bookmark, which has turned into a `git` branch. We could use that link to
create a pull request just like any other.

Because this is a tutorial repository, I won't be merging pull requests. But,
if this was a PR that eventually got merged, it's easy to pull the changes
down: 

```console
❯ jj git fetch
Nothing changed.
```

If there were changes, we'd have some output describing what happens.

With that, you have the basics down! Let's go over everything one more time,
just to double check what you've learned.