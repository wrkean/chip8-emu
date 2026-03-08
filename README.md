# Description

A chip8 emulator that runs on the web. Fetching, decoding, and executing instructions is done in Rust through WebAssembly. Rendering is done through HTML, CSS, and JavaScript.

# Run

Clone the repo

```
git clone --depth=1 https://github.com/wrkean/chip8-emu.git
cd chip8-emu
```

Start a server (Example here uses `live-server` from npm).

```
live-server
```

or

- Download the ZIP
- Extract the contents
- Start a server

Drag and drop roms from your file manager or click the button in the UI to browse for roms. You need to own the roms, you can look them up in the internet by simply searching `chip8 roms`.

# Controls

Keys are mapped like this:

```
1 2 3 C → 1 2 3 4
4 5 6 D → Q W E R
7 8 9 E → A S D F
A 0 B F → Z X C V
```
