import * as wasm from "chip8-wasm";

let chip = wasm.Chip.create();
let display_string = chip.get_display_string();

let canvas = document.getElementById("chip8-canvas");

canvas.textContent = display_string;
