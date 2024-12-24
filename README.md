# rsmsx
 a MSX emulator written in rust, a port from gomsx(https://github.com/pnegre/gomsx). Working in debian linux and windows.

## Why?
 There's no MSX emulator written in rust and I want to learn more about rust and emulation. It's for the educational purpose and did not targeted for 100% perfect emulator.

How to RUN it
-------------

Make sure you have the following libraries installed:
    - libsdl2
    - libsdl2-image
    - libsdl2-ttf

Bundled with the emulator is the C-BIOS rom file. It provides a free implementation of the
BIOS routines of the MSX. No BASIC.

If you want to run BASIC, you can find a MSX1.ROM system file elsewhere.

The file "softwaredb.xml" is useful in aiding the emulator to apply the correct memory mapper
for the MSX cartridge games. It's not required, but usually you can't play games without it.

To run it:

    $ ./rsmsx --cart game.rom

Help:

    $ ./rsmsx -h

Compilation
-----------

So, you want to check the source, eh? First, you'll need a rust installation. I recommend [rustup](https://rustup.rs/).

Next, get the source and the dependencies if necessary:

    $ git clone https://github.com/ghtalpo/rsmsx
    $ sudo apt install sdl2 libsdl2-dev

And, you just build the program:

    $ cd yourworkspace/rsmsx
    $ cargo build --release

Happy hacking!!!