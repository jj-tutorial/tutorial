# Cloning a repository

Let's create a pull request! We're going to do that on GitHub, but these
instructions should be easy to adopt to any of the various similar code forges.

## Forking and cloning

I've got a sample repository located here:
[https://github.com/jj-tutorial/hello-world][]. We'll be using this repo as
an example for this first part of the tutorial. Click "fork" to create
a fork of your own.

Next, let's clone down our fork. Go to the directory where you'd like to create
your clone, in my case, that's `~/src`. And then type this:

```bash
$ jj git clone --colocate git@github.com:<YOUR USERNAME>/hello-world
Fetching into new repo in "/home/<YOUR USERNAME>/src/hello-world"
bookmark: trunk@origin [new] untracked
Setting the revset alias "trunk()" to "trunk@origin"
Working copy now at: snwusnyo 3ea00cda (empty) (no description set)
Parent commit      : qvryknuz 5a15ed3b trunk | Hello, world!
Added 4 files, modified 0 files, removed 0 files
```

Just like `git clone`, `jj git clone` will clone a remote repository
to your local disk. However, we are passing a certain flag to this
command, `--colocate`. `jj` supports two different kinds of repositories:
colocated, and non-colocated. What's the difference? Well, let's take a look
at our repository:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:10:19}} 

We have both a `.jj` and a `.git` directory at the top level. This means both
jj's information as git's information are co-located: they're next to each
other. A non-colocated directory stores `.git` inside of `.jj`. For your
first foray into `jj`, I strongly recommend a colocated repository, as it
allows you to still easily run `git` commands as well as `jj`'s. This can
help ease you into things. It also means tooling that expects to see a `.git`
at the root of the repository will still work.

## Taking a look around

Let's see what our repository looks like:

{{#trycmdinclude tests/tests/cmd/getting-started.trycmd:22:27}} 

This looks a bit different than `git log`, but it's the same general idea: we
can see where we our in our history.

There's a lot I could say about this output, but I'd rather show you how to get
work done first. Let's make our first change.