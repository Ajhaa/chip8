import * as wasm from "chip8-wasm";

let chip = wasm.Chip.new();
for (const val of chip.mem_dump(0x200, 0x20A)) {
  console.log(val);
}
let display_string = chip.display_as_str();

let canvas = document.getElementById("chip8-canvas");

canvas.textContent = display_string;
