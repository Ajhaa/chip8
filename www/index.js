import * as wasm from "chip8-wasm";

wasm.init_panic_hook()
let fibonacci = [
  0x6501, // LD V5 1 ; add this to I (memory pointer) later
  0xA400, // LD I 0x400 ; set memory pointer to 0x400

  0x6400, // LD V4 0 ; loop counter

  0x6100, // LD V1 0 ; init fibonacci seq
  0x6201, // LD V2 1

  0x8320, // LD V3 V2 ; save previous fibonacci number to V3
  0x8214, // ADD V2 V1 ; calculate next fibonacci number to V2
  0x8130, // LD V1 V3 ; write previous fibonacci number to V1

  0x7401, // ADD V4 1 ; increment loop counter

  0x8020, // LD V0 V2 ; load latest fibonacci to V0
  0xF055, // LD [I], 0 ; save V0 to memory pointed in I

  0xF51E, // ADD I V5 ; increment I by 1
  0x3407, // SE V4 07 ; if loop counter is 7, skip next instruction
  0x120A, // JP 0x20A ; jump to the sixth instruction
];

let picture = [
  0x6205, // load x coordinate to register 2
  0x6302, // load y coordinate to register 3
  0xA400, // set memory pointer to 0x400
  0x60FE, // load first part of sprite to register 0
  0x6101, // load second part of sprite to register 1
  0xF155, // load registers 0 and 1 into memory at memory pointer
  0xD232, // display 2 byte sprite at coordinates V2, V3
];

let letter = [
  0x620F, // load x coordinate to register 2
  0x6302, // load y coordinate to register 3
  0x6007, // Load value 0x0A to register 0
  0xF029, // Move I to digit pointed by register 0
  0xD235, // display 5 byte sprite at coordinates V2, V3
]

let chip = wasm.Chip.new();
chip.load_instructions(letter)

while (chip.pc - 0x200 < letter.length * 2) {
  chip.trigger_cycle();
}

for (const val of chip.mem_dump(0x400, 0x40A)) {
  console.log(val);
}
let display_string = chip.display_as_str();

let canvas = document.getElementById("chip8-canvas");

canvas.textContent = display_string;

