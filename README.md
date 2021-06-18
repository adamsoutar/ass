<h2 align="center">
Adam Soutar's Source compiler
</h2>
<h6 align="center">
An all-the-way-down handwritten C compiler, just for fun :)
</h3>

## What?

`ass` compiles programs written in C down to x86-64 assembly code.

It has a tokeniser, parser and codegen module. You can run it like this to generate assembly:

```
ass input.c > output.s
```

_Note:_ ass is not an assembler or linker. See `compile.sh` for assembling binaries

## Implemented features

 - **Variables** Stack based `int` variables (no other types at the moment)
 - **Functions** Functions + arguments can be defined and called
 - **StdLib** Call into standard library functions like `putchar`
 - **If statements** With optional `else`
 - **For loops** Including unusual loop declarations like `(;;)`

**Plus** while loops, binary operators, return statements and more

## Examples

See [./examples](https://github.com/adamsoutar/ass/tree/master/examples) for some example
code that `ass` can compile and run right now - including a FizzBuzz program!

_Is it even a compiler if it can't do FizzBuzz?_
