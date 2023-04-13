An Windows program that solves Wordle Puzzles by trying out every possibily
guess, sees how much it reduces the possible answer pool and recommends the
best world.

This technique gives good recommendations, but is very processor intense,
so it is written in Rust with multithreading. 