# This project will soon be superseded by [Tydi-lang2](https://github.com/twoentartian/tydi-lang-2), which uses CHISEL as backend to generate synthesizable FPGA code

# Tydi-lang

## what is Tydi-lang?

Tydi-lang is designed to be a FPGA accelerator language, integrating [Tydi-spec](https://ieeexplore.ieee.org/document/9098092) to map complex and dynamiclly sized data structures to hardware streams.

## What is the language syntax?
A short [cheat sheet](./cheat_sheet.md) is available.

Some "hello world" examples are also available [here](./CookBook).

Notice that examples #9~#14 illustrate how to convert SQL queries to Tydilang code. The build results are in the "build" folder. VHDL code are in "4_vhdl/proj".

"[12_tpch_sql3](./CookBook/12_tpch_sql3/)" provides a full compile output at [here](./CookBook/12_tpch_sql3/build/).

## How to compile the code?
Compile this [Rust binary](./tydi_compiler/src/main.rs) with cargo.


## What does the Tydi-lang complier do?

![tydi-lang_front_end](./img/tydi_frontend.drawio.jpg)

## How Tydi-lang helps accelerator designs?
![tydi-lang_accelerator](./img/SQL_Tydi.drawio.jpg)

## Related works:

- [til-vhdl](https://github.com/matthijsr/til-vhdl): a backend to generate VHDL fromm Tydi-IR.

- [Fletcher](https://github.com/abs-tudelft/fletcher): a tool to generate hardware interface to access Apache Arrow data on memory via PCIE (not yet integrated).