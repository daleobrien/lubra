#set page("a4")
#let project(title: "", authors: (), body) = {

  set document(author: authors, title: title)
  set page(numbering: "1", number-align: center)

  let body-font = "New Computer Modern"
  let sans-font = "Fira Code"

  set text(font: body-font, lang: "en")
  show math.equation: set text(weight: 400)
  show heading: set text(font: sans-font)

  align(center)[
    #block(text(font: sans-font, weight: 700, 1.75em, title))
  ]

  pad(
    top: 0.5em,
    bottom: 0.5em,
    x: 2em,
    grid(
      columns: (1fr,) * calc.min(3, authors.len()),
      gutter: 1em,
      ..authors.map(author => align(center, strong(author))),
    ),
  )

  set par(justify: true)

  body
}