
# Table of Contents

1.  [Purpose](#org49e64ee)
2.  [Warning](#org9ef7cb5)
3.  [Features](#org6d6a7d8)
4.  [Performance](#orgd63b357)
5.  [License](#orgc26c1cc)
6.  [Contribution](#org6a02328)


<a id="org49e64ee"></a>

# Purpose

`hex-game` is a crate that implements the board game [Hex](https://en.wikipedia.org/wiki/Hex_(board_game)), in Rust.


<a id="org9ef7cb5"></a>

# Warning

**Warning: This is still in development and is not ready for use yet.**


<a id="org6d6a7d8"></a>

# Features

-   Reading and writing Hex games in the human-readable [Smart Game Format](https://en.wikipedia.org/wiki/Smart_Game_Format) (SGF) and additionally
    reading and writing games to a more efficient binary format
-   Playing Hex games using boards of any given size, including determining victory
-   Visualizing Hex games and boards


<a id="orgd63b357"></a>

# Performance

Performance is a central goal of this crate: it will be used to train a neural network to play Hex,
and so at every point the goal is to make determining victory and playing moves as fast as possible.


<a id="orgc26c1cc"></a>

# License

Licensed under either of

-   [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT License](http://opensource.org/licenses/MIT)

at your option.


<a id="org6a02328"></a>

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

