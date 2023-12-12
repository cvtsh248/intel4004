# intel4004
A WIP Intel 4004 emulator written in Rust. All instructions have been implemented apart from WPM (E3) so far, but very few have been fully tested.

I used the following: http://e4004.szyc.org/asm.html for assembling MCS-4 assembly code.

# Usage
To compile programs, visit http://e4004.szyc.org/asm.html and paste the assembled data into a file.

To configure the program, edit ``config/settings.toml``. Note that the step function does not work right now.

Within ``settings.toml`` you may see a few parameters:
* ``filepath`` refers to the path to the assembled file containing the program you wish to run. 

* ``maxcycles`` refers to the maximum number of CPU cycles you want the emulator to run for. 

* ``outputpath`` refers to the location in which you want to dump all the simulation date, including registers, contents of RAM, etc. Note that each file is named as such: ``<cycle count>_<program counter content>.log``

To run and compile the program, run ``cargo run``.
