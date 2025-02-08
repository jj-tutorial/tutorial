# Introduction

<div class="warning">
Please note this is an in-progress version of my tutorial.

You can find a more complete version [over here](https://steveklabnik.github.io/jujutsu-tutorial/).
</div>

Hi there! My name is [Steve](https://github.com/steveklabnik). This is my
tutorial on [jj](https://jj-vcs.github.io/jj/latest/), a version control system.
It is not the official tutorial, but so many folks liked the first version that
I wrote, that it may become the official one someday. We'll see!

`jj` is the name of the CLI for Jujutsu. Jujutsu is a DVCS, or "distributed
version control system." You may be familiar with other DVCSes, such as `git`,
and we will sometimes compare and contrast with it, since it is the most popular
VCS in current use. You don't need to be a `git` expert to read this tutorial.

So why should you care about `jj`? Well, it has a property that's pretty rare in
the world of programming: it is both simpler and easier than other DCVSes, but
at the same time, it is more powerful. We're often
taught, correctly, that there exist tradeoffs when we make choices. And
"powerful but complex" is a very common tradeoff. That power has been worth it,
and so people flocked to git over its predecessors.

I know that sounds like a huge claim, but I believe that the rest of this
tutorial will show you why.

`jj`'s design is about having a smaller number of essential concepts, and ensure
that they fit together nicely. Once you learn the basics, the learning curve
to advanced usage is very smooth.

There's one other reason you should be interested in giving `jj` a try: it has a
`git` compatible backend, and so you can use `jj` on your own, without requiring
anyone else you're working with to convert too. This means that there's no real
downside to giving it a shot; if it's not for you, you're not giving up all of
the history you wrote with it, and can go right back to `git` with no issues.