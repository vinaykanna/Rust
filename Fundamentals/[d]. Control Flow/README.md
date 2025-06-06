# Control Flow
The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

## If Expressions
An if expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

Optionally, we can also include an else expression to give the program an alternative block of code to execute if the condition evaluate to false. If you don’t provide an else expression and the condition is false, the program will just skip the if block and move on to the next bit of code.

It’s also worth noting that the condition must be a bool. If the condition isn’t a bool, we’ll get an error. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.

## Repetition with Loops
It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning.
Rust has three kinds of loops: loop, while, and for.

The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

Fortunately, Rust also provides a way to break out of a loop using code. You can place the ```break``` keyword within the loop to tell the program when to stop executing the loop. We can also use ```continue```, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, if, else, and break; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a while loop.
This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer. While a condition evaluates to true, the code runs; otherwise, it exits the loop.

You can also use the while construct to loop over the elements of a collection, such as an array. As a more concise alternative, you can use a for loop and execute some code for each item in a collection. Using the for loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array. The safety and conciseness of for loops make them the most commonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times