
**THIS IS STILL WORK-IN-PROGRESS**

# Rustorio Circuit Toolkit

A Factorio circuit toolkit that contains a circuit compiler and simulator.


## To Do

 - [_] **Important** - Support comments
 - [_] Proper diagnostics
 - [_] Implement simulator
 - [_] Parse blueprints to IR


## Overview

### Compiler

The main feature of the toolkit is the circuit compiler. It works in 2 stages:

 1. Parse input circuit description (`*.fc` files) to IR (intermediate representation).
 2. Compile IR to factorio blueprint.
 
Refer to the compiler's help message for usage instructions:

```
# cargo run --bin rustorio-circuits -- compile --help

rustorio-circuits-compile 0.1.0

USAGE:
    rustorio-circuits compile [OPTIONS] [--] [inputs]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -I, --import <imports>...    Paths to look into for imports
    -i, --ir <ir-out>            Output intermediate representation to given path
    -o, --output <output>        Output blueprint to path. If unspecified, prints the blueprint to STDOUT
    -P, --param <params>...      Parameters for instantiation of root module
    -r, --root <root>            Which module to use as root [default: main]

ARGS:
    <inputs>...    Input source files
```


### Circuit description language

A circuit description can be composed of multiple files, called *units*. *Units* can be either passed individually to
the compiler as input files, or alternatively you can specify `import` statements at the start of an *unit*:

```
import utils.clock;
```

The import path is seperated with a period (`.`) character. In this example the compiler will look for a file named
`clock.fc` under a directory `utils`. In which path the compiler looks for these files, can be specified specified with
the `-I` option when invoking the compiler. By default the compiler will look in the current directory (i.e. the
 directory in which the current file is in). 

#### Modules

After the `import` statements you can declare multiple *modules*:

```
mod clock {
  [...]
}
```

A *module* is a self-contained collection of combinators that fulfills a certain purpose. Modules can be parameterized
over either numeric values or signal IDs:

```
mod clock_generic<#seconds, $(time), $(enable)> {
    [...]
}
```

The *module* `clock_generic` takes 3 parameters, 1 numeric constant, and 2 signal ID constants. These are available in
the module and will be evaluate at compile-time.

#### Ports

Modules first start with *port* declarations. *Ports* are connection points with which signals can be passed in and out 
the module. Specifically they are named pairs of red and green *wires*. They can be specified in 2 formats:

```
port red clock_enable;
port (red time, green other);
```

The first variant declares a port `clock_enable` which is connected via only a red cable. The red *wire* is available in
the module under the same name (`clock_enable`).

The second variant declares a port with both a red and green wire connected to it. Since wires are always either one or
the other color, the wires need to be named individually. In this form the red cable always comes first and the colors
can be omitted. Thus the following port declaration is equivalient:

```
port (time, other);
```

#### Wires

After the *port* declarations, you can declare *wires*. As mentioned with in the previous section, *wire* declarations give
them names that can be used inside the *module*:

```
wire green loopback;
```

Keep in mind that we also created *wires* when we declared the *ports* in the previous section. Thus we have the following
wires available:

 - `clock_enable` (red)
 - `time` (red)
 - `other` (green)
 - `loopback` (green)
 
Only the first 3 are available from outside the *module* through *ports*. More on that later.

#### Statements

At the end of the module you can specify combinators, sub-modules and their connections. Before we discuss how those are
specified, we must take a look at how constants and signal IDs can be specified.

##### Numeric Constants 

A numeric constant is just a number that can be used a parameter for a constant combinator or sub-module. You can
specify them directly as decimal (e.g. `42`), use parameters as declared in the modules parameter list (e.g. `#seconds`)
or construct expressions: `(((42 - (4 / 2)) * 10) % #some_param)`. In most context such expressions must be parenthesized.

#### Signal IDs

As in Factorio *signal IDs* are either items, fluids or virtual. You can specify them as literals:

 - `$i(iron-plate)`
 - `$f(water)`
 - `$v(signal-pink)`
 
Although the *Everything*, *Anything* and *Foreach* signals still exist as virtual signals with their respective
internal names, there are short-hands available: `$all`, `$any` and `$each`.

Furthermore virtual signals that are letters and digits have short-hands, e.g. `$F` will expand to `$v(signal-F)`.

Signal parameters are referred to with the following syntax: `$(time)`. Where the identifier in the parenthesis is a
module parameter declared earlier.

##### Constant Combinator

They can be given a list of signal, constant pairs:

```
other <- {
    $v(signal-pink) = 42,
    $i(copper-plate) = #seconds
} 
```

The identifier left of the arrow (`<-`) specifies the output wire.

##### Arithmetic combinator

```
loopback[$i:iron-plate] <- time[$a] * other[$b] then 1;
```

This will take `signal-A` from the `time` wire and multiply it with the `signal-B` from the `other` wire. Note that
`time` was declared red and `other` green and thus both can be connected to the combinator. Note that the compiler
doesn't ensure that no mixing of signals takes places (e.g. if there was also a `signal-A` present on `other`). Also
note that this only works if one cable is red and the other is green, as the compiler will not join cables. 

##### Decider Combinator

```
loopback[$i:copper-plate] <- if clock_enable[$(enable)] > 0 then 1;
```

This time we specify the output wire and signal on the left side of the arrow: `loopback[$(time)]`, where `loopback`
refers to the wire and the portion in brackets `[$(time)]` tells the compiler to output as the signal `copper-plate`.

On the right side of the arrow we write `if LHS < RHS then OUTPUT_MODE`, where LHS and RHS are either of the form
`wire[signal]`, or a constant value. The comparision operators are `<`, `>`, `=`, `<=`, `>=`, `!=`.

After the `then` we specify the output mode, which is either `1` or `_`. With `1` one will be output on the specified
output signal (e.g. `copper-plate`) and with `_` the `copper-plate` output will be passed-through from input.  

##### Sub-modules

TODO

#### Full example

This is a clock circuit. It takes as module parameters:

 - `#seconds`: After how many seconds the clock will reset.
 - `$(time)`: The signal on which to output the time in ticks.
 - `$(enable)`: This signal on wire `clock_enable` will enable the clock.

It declares the ports:

 - `clock_enable`: On which it accepts the `$(enable)` signal on a red wire.
 - `time_output`: On which it outputs the time in ticks on a red wire.

```
mod clock<#seconds, $(time), $(enable)> {
    port red clock_enable;
    port red time_output;

    wire green tmp;

    tmp[$(time)] <- if clock_enable[$(enable)] > 0 then 1;

    (time_output, tmp)[$(time)] <- if tmp[$(time)] < (#seconds * 60) then _;
}
```

This can be compiles with:

```bash
cargo run --bin rustorio-circuits -- compile rustorio-circuits/examples/clock-generic.fc -r clock -P 42 -P "item-copper-plate" -P "fluid-water"
```
