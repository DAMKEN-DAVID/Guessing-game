# Guessing-game
Create a simple number guessing game in Rust. The program should:

Use a mutable variable to store a "secret" number (you can hard-code this).
Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
0 if the guess is correct
1 if the guess is too high
-1 if the guess is too low
In the main function:
Use a loop to repeatedly:
Set a mutable guess variable to a number of your choice (simulating user input)
Call the check_guess function
Use an if-else expression to print whether the guess was correct, too high, or too low
If the guess was correct, break the loop
After the loop ends, print how many guesses it took (you'll need to track this in a variable)
These assignments now strictly use only the concepts covered in the provided materials: variables, mutability, basic data types (integers, booleans), arrays, functions, if-else expressions, and the three types of loops (loop, while, for). They don't introduce any new concepts beyond what was explicitly mentioned in the content you provided.