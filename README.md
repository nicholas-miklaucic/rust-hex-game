
# Table of Contents

1.  [Purpose](#org685040e)
2.  [Warning](#org70f7014)
3.  [Features](#org227ba24)
4.  [Performance](#orgbe97803)


<a id="org685040e"></a>

# Purpose

`hex-game` is a crate that implements the board game [Hex](https://en.wikipedia.org/wiki/Hex_(board_game)), in Rust.


<a id="org70f7014"></a>

# Warning

**Warning: This is still in development and is not ready for use yet.**


<a id="org227ba24"></a>

# Features

-   Reading and writing Hex games in the human-readable [Smart Game Format](https://en.wikipedia.org/wiki/Smart_Game_Format) (SGF) and additionally
    reading and writing games to a more efficient binary format
-   Playing Hex games using boards of any given size, including determining victory
-   Visualizing Hex games and boards


<a id="orgbe97803"></a>

# Performance

Performance is a central goal of this crate: it will be used to train a neural network to play Hex,
and so at every point the goal is to make determining victory and playing moves as fast as possible.

