# Assembler made for Zirc16 architecture developed by Kaktus14
[Zirc16 Emulator made by Kaktus14](https://github.com/Kaktus14/zirc16)

## Features
- Compile time statements
    Ops:
    - `+` add
    - `-` subtract
    - `*` multiply
    - `/` divide
    - `&` bitwise and
    - `|` bitwise or
    - `^` bitwise xor
    - `<<` bitshift left
    - `>>` bitshift right
    ```
    lim R2, ((5 + 5) << 2)
    ```
  NOTE: you can manipulate label addresses
- Sub-labels (simmilar to any other assembler)
    ```
    main:
      lim R1, 5
    .loop:
      limb R2, .loop

    func:
      add R1, .loop
    .loop:
      limb R1, main.loop
    ```
- flexible instruction set and registers
  #### just look at common.rs inside components folder and see for yourself
