<div align=center>
<h1>ram_simulator_rust</h1>
So, basically, I wanted to learn Rust and this is what came out of it.
<br>&nbsp;

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Kotlin](https://img.shields.io/badge/BuK-%237F52FF.svg?style=for-the-badge&logoColor=white)

</div>

## What is this?
`ram_simulator_rust` is a simple [Register Machine](https://en.wikipedia.org/wiki/Register_machine)
simulator written in Rust. It supports hardcoded programs and executing files via its cli. By 
default, it outputs all registers (up to the highest that was used) after executing every instruction.

The instruction set is the one described in `lib/ram_simulator/src/instruction/mod.rs`. (Basically
the one from my 
[Computability and Complexity](https://online.rwth-aachen.de/RWTHonline/pl/ui/$ctx/WBMODHB.wbShowMHBReadOnly?pKnotenNr=335994&pOrgNr=14194) course with one minor addition.)

## Program syntax

For the instruction set, see `lib/ram_simulator/src/instruction/mod.rs`.

Here's an example program that computes `floor(ld(c(1)))` (the floor of the log_2 of the
value that is in the 1st register):

```asm
INIT 256        ; Initialize the 1st register with 256
CLOAD 0         ; this is line 1 (not 2, the initialisation is **not** counted)
STORE 2
CLOAD 1
STORE 3
LOAD 3
SUB 1
IF c(0) > 0 THEN GOTO 15
LOAD 2
CADD 1
STORE 2
LOAD 3
CMULT 2
STORE 3
GOTO 4
LOAD 2          ; even though this is technically line 16, GOTO 15 jumps here
CSUB 1
STORE 1
END
```

The `INIT` instruction is my own addition. It initializes the first `n` registers with the given values.
For example, to set `c(1)=5, c(2)=3, c(3)=2`, you can use `INIT 5 3 2`. Every program must start with this,
even if it's not used/empty.

<b>Note:</b> For hardcoded programs, look at `src/examples/example{1,2}.rs`.

### On the topic of syntax
- Empty lines between instructions are currently not supported. 
- Comments are supported on any line via any delimiter 
    (e.g. `#`, `//`, `;`, `--`, `/* */`, `--[[ ]]--`, etc.),
    except on `INIT`, as long as there's at least one space between the comment and the instruction.
- `IF`'s syntax is matched using a regex and is intentionally not very strict. The
    number of whitespaces doesn't matter either. The following get parsed as the exact same instruction:
    ```asm
    IF c(0)>0   THEN GOTO 15
    IF c(0) > 0 THEN GOTO 15                ; if you write code like this,
    IF c(0)> 0  THEN GOTO 15                ; you're a menace to society though
    IF c(0) >0  THEN GOTO 15
    IF c(0) >  0   THEN    GOTO         15
    ```

## Usage

### Building from source
1. Build the project with `cargo build --release`. The binary will be in `target/release/rscli`.
2. (optional) Install the binary
    - Linux: `sudo install -m 755 target/release/rscli /usr/local/bin/rscli`
    - Windows: i don't know, figure it out
    - MacOS: ¯\\\_(ツ)\_/¯
3. Execute a program via `rscli /path/to/program.s`.