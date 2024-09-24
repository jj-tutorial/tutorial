# A summary of the basics

Let's go over what we've learned in this chapter:

We can clone a repository with `jj git clone`. The `--colocated` flag allows us
to keep using our normal `git` tooling if we wish.

`jj` has both changes and commits. A change represents the evolution of a commit
over time.

We can look at the history of our repository with `jj log`, and the state of
the current change with `jj st`.

`@` is a special name for the change representing the working directory. Every
time we run a `jj` command, `@` is updated with the latest snapshot of the
working directory.

When we're done with building up our changes in `@`, we can use `jj commit -m`
with a message to describe our change with, and then start a new change.

Bookmarks are `jj`'s feature to work with `git`'s branches, and `jj bookmark set
<name> -r <revision>` will set a bookmark named `<name>` to the given revision.

`jj git push` will push any changes in our local repository to our `git` remote,
and `jj git fetch` will synchronize our local repository with any changes that
have been made on the remote that we don't have yet.

That's a bunch of stuff! With these steps, you have the basics down, but there's
still a bunch to learn. In the next chapter, we'll learn more about the two most
popular workflows for getting real work done with `jj`.