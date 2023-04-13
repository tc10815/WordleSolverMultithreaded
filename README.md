# WordleHelper
## Written in Rust with multithreading. 

A Windows program that assists players by analyzing Wordle Puzzles and recommending guesses.
It works by trying out every possibily guess, sees how much each guess reduces the possible 
answer pool and recommends the best world.

This technique gives good recommendations, but is very processor intense,
so it is written in Rust with multithreading. 

**Computes possibilities on 15 threads simulatiously since the computers I use it
on all support over 15 threads, and using over 15 threads counteracts optimization
techiques which actually makes it run slower.**
(https://raw.githubusercontent.com/tc10815/WordleSolverMultithreaded/main/wordlesolver_screenshot.png)
Code is found in main.rs

![screenshot](https://raw.githubusercontent.com/tc10815/WordleSolverMultithreaded/main/wordlesolver_screenshot.png)

In this screen shot the user has guessed REACH as the first guess and TOILS as the second. R is yellow and O is green.
This shows all 9 possible answers given that response from Wordle, it shows 3 equally good answers that cannot be 
the correct answer (5 letter words not in valid answer pool), and the best guess which is in the valid answer pool (gourd).
