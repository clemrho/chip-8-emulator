# chip-8-emulator
> a simple chip-8 emulator written by Rust

#### Group name: 
ILoveRust123
#### Group member: 
Yanze Lu (yanzelu2) , Jiayi Yan (jiayi22)
#### Description: 
Chip-8 was a very simple programming language used for many of the 8-bit games from the 70s. This project is a simple implement of the chip-8 emulator. May include memory allocation, game loop, handling graphics. This is a very classic and charming project.
#### Technical Overview: 
One "chip-8" class:  Stores all the memory arrays and instances.
Rom-loading class : loads the contents of a ROM file. 
Fonts-loading class : convey the pixel character to the corresponding graphic images.
Random Number Generator : places a random number into a register.
The Main Loop : Game Loop (aka.the driver class) setup the callback, call the chip-8 class member functions, finally update and clear the display.
- Checkpoint 1: Rom-loading class finished
- Checkpoint 2: One "chip-8" class, Rom-loading class, Fonts-loading class, Random Number Generator.
- Final: All finished.
#### Possible Challenges:
 working with a UI in Rust; Parallelism & Multithreading ;time limit
#### References: 
>How to write an emulator (CHIP-8 interpreter)
> https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
>CHIP‐8 Technical Reference
>https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
>Cowgod's Chip-8 Technical Reference v1.0
>http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
>AUSTIN MORLAN's ABOUT CONTACT RSS
>https://austinmorlan.com/posts/chip8_emulator/


