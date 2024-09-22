# Functions
Functions are reusable blocks of code designed to perform specific tasks. They take input (known as parameters or arguments), process that input, and return a result (though not all functions return a value). Functions help with code organization, modularity, and reusability, making programs easier to maintain and scale.

We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments

In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. 

## Statements and Expressions
Function bodies are made up of a series of statements optionally ending in an expression.
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.
Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions.
- Statements do not return values. Therefore, you can’t assign a let statement to another variable
- Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
- Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11. Expressions can be part of statements
- Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression
- Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.