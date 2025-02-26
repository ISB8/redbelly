## Usage
To run a script with redbelly use:
```
$ ./redbelly [Script]
```
Alternatively access the interactive prompt by invoking redbelly with no args:
```
$ ./redbelly
> println("Hello World");
Hello World
```


## Syntax


### Comments
Anything after "//" will be ignored.


### Variables
Declare a variable with the "let" keyword. A variable can be a number, a utf8 string, true, false or nil.
```
let foo = "bar";
```
To assign to a variable use the "=" operator:
```
let foo = "bar";
foo = "baz";
println(foo); // Outputs baz
```


### Loops
Loops use C style syntax:
```
while(true) {
   // Loops forever
}
```
There is no break statement, however a similar effect can be achieved with the following:
```
let break = false;
while(!break) {
   // do stuff
   break = true; // ends the loop
}
```


For loops are also supported:
```
for(let i = 0; i < 10; i = i + 1) {
   // Loops 10 times
}
```
### Functions
Declare a function with:
```
func foo(bar) {
   // function
}
```


Returning from a function is supported:
```
func foo(bar) {
   return bar;
}
```


## Builtins
Redbelly provides several builtins to help with programming.


### clock
Returns the number of seconds since the Unix Epoch.


### print
Prints the given value to stdout.


### println
Prints the given value to stdout, and appends a newline.


### input
Prompts the user for input, then returns it as a string.


### exit
Exits the program with a given exit code.


### str_to_num
Takes a given string and converts it to a number. Returns nil if the conversion fails.


### round
Returns a given number rounded to the nearest whole integer. Returns nil if not given a number


### abs
Returns the absolute value of a given number. Returns nil not given a number


