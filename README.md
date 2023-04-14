# WordleHelper
## Written in Rust with multithreading. 

A program that assists players by analyzing Wordle Puzzles, identifying possible answers and recommending guesses. A binary is compiled here for Windows, but naturally this source code can be compiled for any operating system (see bottom of README for Linux instructions).
It works by trying out every possible guess with every possible answer scenario, ultimately recommending the guess which leads to the 
smallest possible answer pool on average, after the result of all possible guesses applied to all answers scenarios are averaged together.

This technique gives good recommendations, but is very processor intensive,
so it is written in Rust with multithreading. 

**Computes possibilities on 15 threads simulatiously since the computers I use it
on all support over 15 threads, and using over 15 threads counteracts optimization
techiques which actually makes it run slower.**
(https://raw.githubusercontent.com/tc10815/WordleSolverMultithreaded/main/wordlesolver_screenshot.png)

![screenshot](https://raw.githubusercontent.com/tc10815/WordleSolverMultithreaded/main/wordlesolver_screenshot.png)

In this screen shot the user has guessed REACH as the first guess and TOILS as the second. R is yellow and O is green.
This shows all 9 possible answers given that response from Wordle, it shows 3 equally good responses that cannot be 
the correct answer (that is, will shrink the remaining answer pool the most on average, but cannot possibly be the correct answer), and the best guess which is in the valid answer pool (gourd). 
 
*Instructions: Enter Wordle's response to guess in 10 characters, where before
each letter you indicate the color of the square, ' ' for blank '*' for yellow,
and '=' for green (e.g.' t=e\*s t s') All remaining possible words will appear.
Other options: 'r' gets best possible word. 'c' clears guesses. 'w' generates
Wordles response to a guess. 'b' removes most recent guess from guess list.
'q' quits the program*


Code for program is found in main.rs here:
[https://github.com/tc10815/WordleSolverMultithreaded/blob/main/src/main.rs](https://github.com/tc10815/WordleSolverMultithreaded/blob/main/src/main.rs)

Executable windows binaries are found in "compiled" (must have data folder in same directory as binary). This source code can be compiled on other operating systems.

**To compile successfully on Linux (and most likely Mac OS although I haven't tested it) change line 9 of main.rs to:**

    let answer_words_file = String::from("./data/words_answers.txt");

It is compiled with "cargo build", executed in the home directory of the project.
