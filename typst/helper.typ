 #let make1(name, caption) = {
     figure(
       supplement: none,
       image(name, width: 100%),
       caption: [#caption],
      )
 }

  #let make2(name, caption) = {
      figure(
        supplement: none,
        image(name, width: 100%),
       )
  }