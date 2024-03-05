#import "template.typ": *

#import "@preview/algo:0.3.1": algo, i, d, comment, code

// Take a look at the file `template.typ` in the file panel
// to customize this template and discover how it works.
#show: project.with(
  title: "Maze",
  authors: (
    "Whale",
  ),
)

// We generated the example code below so you can see how
// your document will look. Go ahead and replace it with
// your own content!

= Introduction
How it works ...

== In this paper
#lorem(20)

=== Contributions
#lorem(40)

= Related Work
#lorem(500)



#algo(
  title: "Fib",
  parameters: ("n",)
)[
  if $n < 0$:#i\        // use #i to indent the following lines
    return null#d\      // use #d to dedent the following lines
  if $n = 0$ or $n = 1$:#i #comment[you can also]\
    return $n$#d #comment[add comments!]\
  return #smallcaps("Fib")$(n-1) +$ #smallcaps("Fib")$(n-2)$
]