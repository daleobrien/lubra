#import "template.typ": *

#import "@preview/algo:0.3.1": algo, i, d, comment, code
#import "@preview/cetz:0.1.0"

// Take a look at the file `template.typ` in the file panel
// to customize this template and discover how it works.
#show: project.with(
  title: "Maze"
)

// #let make(name, caption) = {
//     rect(width: 10mm, height: 5mm)[
//         #text(body)
//     ]
// }

= Shapes

#grid(
  columns: 3,

    figure(
    image("svgs/049-n45.svg", width: 100%),
    caption: [5-6],
  ),
  figure(
    image("svgs/043-n45-a14.svg", width: 100%),
    caption: [5-6],
  ),
  figure(
    image("svgs/044-n45-a15.svg", width: 100%),
    caption: [5-6/3-6],
  ),
  figure(
    image("svgs/042-n45-a14-b35.svg", width: 100%),
    caption: [5-6/3-5],
  ),
  figure(
    image("svgs/045-n45-a24-b15.svg", width: 100%),
    caption: [5-6/2-6],
  ),
  figure(
    image("svgs/046-n45-a24-b35.svg", width: 100%),
    caption: [5-6/1-5],
  ),
   figure(
    image("svgs/047-n45-a24.svg", width: 100%),
    caption: [5-6/1-5],
  ),
  figure(
    image("svgs/048-n45-a35.svg", width: 100%),
    caption: [5-6/1-5],
  ),
)


= Introduction

Maze is based upon a trianglar shaped tile with size nodes as shown. 

#cetz.canvas({
  import cetz.draw: *

  let (a, b, c) = ((0, 0), (1,1.732), (2,0))
 
  line(a,b, (2,0), close:true)
  
  fill(black)
  
  circle(a, radius: 0.1); content((-.3,-.3), [2])
  circle(c, radius: 0.1); content((2.3,-.3), [3])
  circle(b, radius: 0.1);  content((1,1.732+.3), [1])
  circle((1,0), radius: 0.1); content((1,-0.3), [6])
  circle((0.5,1.732/2), radius: 0.1);  content((0.5-0.3,1.732/2+0.15), [4])
  circle((1.5,1.732/2), radius: 0.1); content((1.5+0.3,1.732/2+0.15), [5])
})

TODO: Above, show posible tiles, remove the nodes ...

== Mapping

Each tile connects to each it's neighbours nodes such that the node indexes are all the same. 

#cetz.canvas({
  import cetz.draw: *

  let (a,b,c,d,e,f) = ((0, 0), (1,1.732), (2,0), (3,1.735),(4,0),(5,1.735))
 
 
  line(b,c,d, close:true, stroke: gray) 
  line(c,d,e, close:true, stroke: gray) 
  line(d,e,f, close:true, stroke:gray) 
  line(d,e,f, close:true, stroke: gray) 
  line(a,c,(1,-1.732), close:true,stroke: gray)
  line(e,c,(3,-1.732), close:true,stroke: gray)
  line(c,(1,-1.732),(3,-1.732), close:true,stroke: gray)
  
  line(a,b,c,close:true)

  fill(black)
  
  circle(a, radius: 0.1); content((-.3,-.3), [2])
  circle(c, radius: 0.1); content((2.3,-.3), [3])
  circle(b, radius: 0.1);  content((1,1.732+.3), [1])
  circle((1,0), radius: 0.1); content((1,-0.3), [6])
  circle((0.5,1.732/2), radius: 0.1);  content((0.5-0.3,1.732/2+0.15), [4])
  circle((1.5,1.732/2), radius: 0.1); content((1.5+0.3,1.732/2+0.15), [5])
  
  fill(gray)

  circle(d, radius: 0.1,stroke: gray); content((3, 1.735+.3), [2])

  circle(e, radius: 0.1,stroke: gray); content((4, -.3), [1])
  circle((2.5,1.732/2), radius: 0.1,stroke: gray); content((2.5+0.3,1.732/2+0.15), [6])
  circle((1,-1.732), radius: 0.1,stroke: gray); content((1,-1.732-.3), [1])
})

Constructing a graph of how each node is connected is thus much easier; however, the oriniation of each tile is now dependent on it's positions.

We have six posible tile orintations, ...

== Intra-tile connections

#cetz.canvas({
  import cetz.draw: *

  let (a, b, c) = ((0, 0), (1,1.732), (2,0))
 
  line(a,b, (2,0), close:true)
  
  fill(black)
  
  circle(a, radius: 0.1); content((-.3,-.3), [2])
  circle(c, radius: 0.1); content((2.3,-.3), [3])
  circle(b, radius: 0.1);  content((1,1.732+.3), [1])
  circle((1,0), radius: 0.1); content((1,-0.3), [6])
  circle((0.5,1.732/2), radius: 0.1);  content((0.5-0.3,1.732/2+0.15), [4])
  circle((1.5,1.732/2), radius: 0.1); content((1.5+0.3,1.732/2+0.15), [5])

  line((1,0), (1.5,1.732/2),stroke: (paint: red, thickness: 5pt))
  
})

TODO: Draw a vector on each tile to demostrate it's orination

Draw ...


Forumula for Rotations of a tile ...

...

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
