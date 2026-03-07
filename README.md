# Description

A chip8 emulator that runs on the web. Fetching, decoding, and executing instructions is done in Rust through WebAssembly. Rendering is done through HTML, CSS, and JavaScript.

# Run

Clone the repo

```
git clone --depth=1 https://github.com/wrkean/chip8-emu.git
cd chip8-emu
```

Start a server (Example here uses `live-server` from npm)

```
live-server
```

Drag and drop roms from your file manager or click the button in the UI to browse for roms. You need to own the roms, you can look them up in the internet.
