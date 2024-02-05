# Brainflub-rs

Have a look at the [Brainfuck wikipedia page](https://en.wikipedia.org/wiki/Brainfuck) to see what brainfuck is.

The major difference between this and normal Brainfuck is that this uses Unicode Scalar instead of ASCII, so code that is based off the ASCII version will not work as expected!

## Example Hello World
This code work with this and the [copy.sh](https://copy.sh/brainfuck) interpreter
```brainfuck
++++++++++[>++++++++++<-]>++++.->++++++++++[>++++++++++<-]>+.->++++++++++[>++++++++++<-]>++++++++..->++++++++++[>++++++++++<-]>+++++++++++.->++++++++++++++++[>++<-]>.->++++++++++[>++++++++++<-]+++++++++[>++<-]+>[<+>-]<.-<<<+.->>>>-++++++++++[>++++++++++<-]>++++++++++++++++++++++++.>++++++++++[>++++++++++<-]>++++++++.->++++++++++[>++++++++++<-]>.->++++++++++++++++[>++<-]>+.-.-
```

## Import things to note
If you interpet code the char 'b' will dump the memory to stdout as a debugging tool
