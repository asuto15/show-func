# Binary Symbol Function Reader in Rust
This program is a simple utility written in Rust that reads and displays the binary symbol function of a given binary. The tool call the command `objdump -d` inside, and showing how many instructions the each function have and showing the names of the child function which is called by the parent function.

## Features
- Displays the number of instruction in each function.
- Shows the connection of the functions.

## Usage
To use the program, simply compile it and then run:

```
> cargo run -- -i /bin/wc
// or
> cargo run -- -c /bin/wc
```
You must input either options `-i` or `-c`.

### Sample Output
The output of the `-i` option:
```
❯ cargo run -- -c test
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/show-func -c test`
_init:
        8 instructions
.plt:
        3 instructions
__cxa_finalize@plt:
        3 instructions
_start:
        14 instructions
deregister_tm_clones:
        11 instructions
register_tm_clones:
        16 instructions
__do_global_dtors_aux:
        17 instructions
frame_dummy:
        2 instructions
main:
        13 instructions
f:
        6 instructions
g:
        7 instructions
h:
        9 instructions
__libc_csu_init:
        36 instructions
__libc_csu_fini:
        2 instructions
_fini:
        4 instructions
```
The output of the `-c` option:
```
❯ cargo run -- -c test
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/show-func -c test`
_init:
.plt:
__cxa_finalize@plt:
_start:
deregister_tm_clones:
register_tm_clones:
__do_global_dtors_aux:
frame_dummy:
main:
f:
g:
h:
__libc_csu_init:
__libc_csu_fini:
_fini:
```
