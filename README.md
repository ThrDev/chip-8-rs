# chip-8-rs
A simple chip-8 emulator written in rust.

Uses minifb for the display, and is multi-threaded. One thread updates the logic and runs opcodes while the other thread renders the display.

To run, simply: ./chip-8-rs ./file.ch8

Debugging support needs to be added, and while I had a prototype for stepping through each instruction, I have since
removed that to try to get the multi-threading to be supported better.

### Examples

![Example](https://shotr.dev/J3W6tn.png)
![Example1](https://shotr.dev/l92Rpv.png)
![Example2](https://shotr.dev/A48KmI.png)