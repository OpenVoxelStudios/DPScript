# Compilation

DPScript will use a simple method, where it tokenizes the code into a very basic list
of tokens, and then it will form it into an AST (Abstract Syntax Tree), validating the
code's structure in the process. Then, it will validate the AST and perform type
inference and checking, make sure things exist, and finally output the `mcfunction` code.

# IR

DPScript has its own IR (Intermediary Representation), which allows people to build their
own custom frontend for DPIR and make their own syntax. If they want the syntax to look
like python, they can! This flexibility allows for DPScript to be a lot more than just a
nice looking language, but a full-on compiler backend with incredibly advanced features.

# Generated Structure

Essentially, everything happens at compile time. Variables are just references to data
storages and keys (as well as their types), and everything is validated at compile time.
The only exception to this is `for ... in ...` loops, since those will always be for
selectors and will be simply replaced with `execute as [selector] run function [func]`.

(Soon, however, I'll add suppoort for loop unrolling and iterating through arrays!)

Constant variables (or static variables) are compile-time variables that are substituted
for their actual values during compilation.

# Modules

Modules are pure syntactic sugar, they do nothing. They're just there to make sure that
IDs don't clash and you can have multiple functions with the same name, as when they're
generated they will have separate names.
