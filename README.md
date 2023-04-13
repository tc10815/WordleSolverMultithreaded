# WordleHelper
## Written in Rust with multithreading. 

A Windows program that solves Wordle Puzzles by trying out every possibily
guess, sees how much each guess reduces the possible answer pool and recommends the
best world.

This technique gives good recommendations, but is very processor intense,
so it is written in Rust with multithreading. 

**Computes possibilities on 15 threads simulatiously since the computers I use it
on all support over 15 threads, and using over 15 threads counteracts optimization
techiques which actually makes it run slower.**

Code is found in main.rs
https://github.com/tc10815/WordleSolverMultithreaded/blob/main/src/main.rs
