Yet another CHIP-8 emulator written in **Rust**, using **SDL2** for graphics and input.



```
.
├── chip-8-core/      # Emulator core (CPU, memory, opcodes)
├── frontend/         # SDL2-based frontend (window, input, render loop)
├── roms/             # CHIP-8 ROMs (not included)
└── README.md
```

* `chip-8-core` is platform-agnostic and reusable
* `frontend` handles SDL2 rendering and keyboard mapping

---


## Build & Run

```bash
cargo run  <path_to_rom>
```

Example:

```bash
cargo run roms/INVADERS
```

---

## Controls

CHIP-8 keypad mapping:

```
1 2 3 4        →    1 2 3 C
Q W E R        →    4 5 6 D
A S D F        →    7 8 9 E
Z X C V        →    A 0 B F
```

---

* Sound is currently represented as a console beep.
* Rendering uses a simple boolean framebuffer scaled via SDL2.


