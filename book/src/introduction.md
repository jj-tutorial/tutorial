# Introduction

<div class="warning">
Please note this is an in-progress version of my tutorial.

You can find a more complete version [over here](https://steveklabnik.github.io/jujutsu-tutorial/).
</div>

Hi there! My name is [Steve](https://github.com/steveklabnik). This is my
tutorial on [jj](https://jj-vcs.github.io/jj/latest/), a version control system.
It is not the official tutorial, but so many folks liked the first version that
I wrote, that it may become the official one someday. We'll see!

If you're already sold on jj, feel free to skip to the next section. But if
you're checking out this tutorial because you're curious about jj, but you're
not familiar with what it is all about, keep reading.

Distributed version control became popular in the mid 2000s. Specifically, two
projects, released just twelve days apart, really took off: git and mercurial.
I won't speculate as to why git "won" in the open source world, but it surely
did: StackOverflow's 2022 Developer Survey reported that [94% of folks used git
as their primary version control
tool](https://survey.stackoverflow.co/2022/#section-version-control-version-control-systems),
with just 1% for Mercurial.

JJ is a next-generation version control system. It takes lessons from both of
these projects, and has made a version control system that is as powerful as
git, but as easy to use as Mercurial. But here's the truly great thing about
jj: you can use jj with any git repository, and it'll just work. This means you
can give jj a try, but nobody else needs to even know. And if you decide to not
continue to use jj, you can stop, and you won't lose any of the work you did
during your time with it, because it'll still be there, in your upstream git
repository.
