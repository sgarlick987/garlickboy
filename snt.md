# Goals

* Learn rust
* Learn how gameboy software and lesser extent hardware works
* Learn how to do multi media with sdl2
* Get practice in with lower level programming
* Rely heavily on published docs and avoid looking directly source code for other emulators

# Why Gameboy?

Lots of good documentation especially in last few years. Strong community and active developers. There
are also a large number of tests roms written by the developers for some of the most compatible emulators.

Simple enough to atleast to get initial results and decent accuracy but complex enough to
allow for long term development.

I played a lot of gameboy growing up.

# Why Rust

Buzz. Basic requirement was to avoid garbage collected languages and to avoid languages I was already familiar with (c, c++).

Rust is said to help avoid common programming mistakes around manual memory management and lends it self well to places
where c or c++ might've been used in the past. I can say so far I've not run into any memory management errors at runtime
in this project but it required a greater initial effort to learn how rust achieves this at compile time.

Technically sdl is still c++, but I'm using rust bindings for it. I wanted to try one of the bigger names here but
I don't expect it would be too hard to swap out as my needs aren't that high.

Rust has most of the typical things you'll find in most languages, wide number of primitive types, strings, structs,
generics, modules, closures, testing, etc but is not oop. Everything in rust requires you to be explicitly, from variables
being immutable by default, to requiring you to explicitly enable your various data to allow copying, hash codes,
equals, heap vs stack allocation, etc. One thing you won't see in most languages is the borrow checker which rust uses
to help determine rules for when memory is freed.  We'll see some of these as we look at our examples.

Lastly my experience so far is that I would definely use rust againt in places where I need a non gc lower level language, but
would personally stick to jvm, golang, etc for business applications.

# Gameboy basics

We need to discuss a few basic things, but remember that while keeping it simple here a lot of these
tend to get much more complicated as you aim for higher accuracy.

## Pandocs

https://gbdev.io/pandocs/ contains most this information in detail.

As we won't be able to go into much detail, if you are interested in
how specific parts works I recommend reading more there.

## CPU

Our cpu doesn't have modern features. There's no instruction pipelining, minimal 8/16 bit registers and instruction set.

It has a clock speed of 4.19 MHz but as every instruction takes atleast 4 or multiples of 4 cycles and
no instruction pipelining, we can treat it effectively as 1.05 MHz and can time our emulator off this. We
refer to these 4 MHz cycles as tcycles, and the 1 MHz as mcycles. Its most common for us to talk and code in mcycles.

All interactions with the gameboy happen through the cpu. The cpu interacts with other components like the
PPU, joypad through the address bus.

A program counter is maintained which is a 16 bit register holding the address of the next instruction to execute.
This doesn't have to only be rom, but can be any location in ram, which we can see used with sprites.

All programs are written in assembly.

## Address bus

The address bus is 16 bits and gives the cpu ways to interact with rest of the hardware via read/writes.

This means that certain things like joypad state (0xFF00) have known addresses the programmer can access.

## PPU

This pushes pixels to the screen. It runs at 160x144 resolution and 60fps. Doesnt work on pixel's but "tiles" 8x8 sets of pixels.

## Sound

I've not gotten here yet.

# Bootrom

This lives in the gameboy itself and is the scrolling of the nintendo logo and ping sound. When mapped
it lives in the first 256 bytes of the gameboys memory.

Right now I'm using the official bootrom for the original gameboy (DMG) and emulating it,
but a common feature especially as you add more gameboy types (color, super, etc)
is to just skip the bios and initialize your emulator to match the state once a boot rom is executed.

Once this finishes executing it writes to 0xFF50 and unmaps itself, allowing for the carts initial
values to be accessible.

# Tetris

Tetris is considered one of the most simpliest games on the gameboy. It has no extra memory chips on its cart,
doesnt require high cpu accuracy, and uses minimal graphics features. To give you an idea of where I actually am,
another game that is considered an easy target is Dr Mario, and we can take a look and see how that is handled
while tetris is running pretty well.

Tetris also has a number of disassemblys out there that I've used to help trace my code execution to verify
various fetch, jp, call, ret, etc statements properly incremented the program counter.

# Specific examples

## Main loop and sync

* src/emu/mod.rs

The gameboy runs as 60 fps, and the cpu as 1.05 MHz, which means that we have 1.05MHz / 60 mcycles per frame.
Using this the main loop keeps track of how many mcycles is used by the instruction execution loop.

At the start of each frame (cycle 0, first 1/60 frame) we handle both input for our controller, and sdl event
loop (eg esc to quit).

## Instruction fetching and execution

* src/gameboy/instructions
* src/gameboy/mod.rs
* src/gameboy/instructions/shift/rla.rs

Each cpu instruction has a byte value, or for prefixed instructions 0xCB and the following byte. We map everyone
of these byte values to an enum using a large switch statement. As most instructions are duplicates based on
different registers/pointers/etc we can reduce these greatly in terms of what we must implement.

Each instruction takes 1 or more mcycles, and we break these down into fuctions that can be executed in our main
loop to keep our timing.

example run rla tests

## Interrupts

* src/gameboy/interrupts

There are a number of events that can trigger cpu interrupts, and run code installed at specific addresses in our rom.

The main one tetris relies on is the vblank interrupt. This is triggered once the ppu has finished rendering the last visible
line on the screen, and will then continue to render 10 more off screen. This time is referred to as vblank, and when it is
entered the vblank interrupt is fired, and we jump to address 0x0040 which then typically jumps to where the games interrupt
routing is.

Tetris uses this to time and handle screen progressions. It does this by writing to a programmer chosen during vblank
and checking in the code if it has been changed to 1 to know its interrupt handler ran.

example disable interrupt handlers

## Joypad input

* src/emu/controller/mod.rs
* src/gameboy/bus/mod.rs
* src/gameboy/bus/joypad.rs

As mentioned joypad input can be accessed at 0xFF50 and is a single byte. The basics of how the joypad worked are as follows

* Bits 7/8 are unused
* Bits 1-4 are used for either the direction buttons or action buttons
* The programmer can use bits 5/6 to set wether action or direction buttons will be returned in lower bits on next read

One quirk is that the unpressed state is 1 and pressed is 0. Same for selecting button types, a write of 0 is selecting not 1.

For actual input, we are just using a keyboard implementation of our Controller trait (interface). Sdl provides the keyboard state for us, and we
map that to our struct.

Our goal here is two things. First we need to take our controller state, and convert it to the binary expected by gameboy games, and make our bus
properly return when that address is read. Second is keep track of writes to our register to know which set of buttons state to return on next read.

## Div register and tetris "rng"

* src/gameboy/mod.rs
* src/gameboy/bus/mod.rs

When I first got tetris working, only the square block was dropping. I dug through the assemelby and finally realized it was reading from a value
that it'd never written to to determine the next block to drop. Looking at the pandocs I realized that this address was a special register that
the game expected the gameboy to be populating.

This register simply counts up increasing every 64 mcycles and can be reset by writing to it. This means that rng in tetris is more or less based
on how long your gameboy is on.

Once I started returning the counter I setup from that address, next blocks became "random" as expected.

example break rng

# Graphics

There are quite a number of graphics modes that you need to implement for high accuracy, but tetris only requires
a few parts to work. We'll look into how graphics are rendered next.

## Palette

* src/gameboy/gpu/palette.rs

The gameboy has a 4 color palette, effectively black, white, light gray, dark gray. The original gameboy this
was shades of green, but gameboy pocket more gray.

The gameboy can remap these 4 colors pallete, and is used in the boot rom to effectively make 3 colors black and one white.
Originally before I implmented updating palletes the nintendo logo was actaully grey not black.

## Tiles

Tiles are what the gameboy uses to display graphics and are typically 8x8 blocks of pixels represented by
two bytes for each row, 16 bytes per tile.

Each line is two bytes are &ed bit by bit, each combination of 00, 01, 10, 11 are mapped to the 4 color pallete.

These values get copied into a special space of vram dedicated to tiles.

## Background maps

This is another section of vram that explains how it wants to lay out tiles on a a 256x256 space, or 32x32 tiles. This
is populated by writing indexes for tiles into the address space which then gets displayed by the ppu. As the gameboy
is actually 160x144 resolution, the fully 256x256 area isnt displayed. This is then used with the x and y scroll registers
on the gpu to give the illusion of scrolling like in our boot rom.

## Sprites

Sprites againt use tiles to be displayed. 40 sprites can be loaded into a special space of memory called oam. In this memory
each sprite is represented by 4 bytes, x, y, tile index, and attributes. Using these we can decribe where on the screen a tile
is displayed and any special effects (eg flipping) to be applied.

As sprites have special attributes that can be applied, they live in the special oam memory. To get them copied over typically
dma is used. To do this, a programmer can take the upper byte of a 16bit address and write to the dma register. Once
done this kicks off a process where the next 160 bytes (40x4 byte sprites) are copied from the supplied address into oam.

During this time reads from all except an address space called HRAM are effectively unreadable. This is why its important
that the gameboy can execute instructions not just from rom, but also ram, HRAM in this case. Most games will install
a small busy loop into HRAM and jump back once dma is done since not much else can happen during that time.

## Rendering

Originally I was rendering the window using sdl canvas and using their draw pixel function. This way we didnt even hit 60 fps
which obviously wasnt going to workout for our timing. To fix this I ended up switching to using a texture, which I could
still write to pixel by pixel, but in a must faster manner.

When going to render we copy the texture all at once quickly displaying out image. This also benefited that I could use
gpu scaling to handle rendering the game at various scales while still only working with the 160x144 pixels in the code.

Right now I have zero opimizations going on for rendering. Each frame I'm redrawing everything pixel by pixel from "vram", but
the gameboy itself is not actaully rewriting any of that memory a lot of times. Tetris loads its backgrounds into memory
once per transistion. Due to this a lot of stuff like abstracting tiles out likely into their own textures and generating them
only when they are updated by the game will happen at some point.
