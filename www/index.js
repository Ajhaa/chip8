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
  0x600F, // Load value 0x0A to register 0
  0xF029, // Move I to digit pointed by register 0
  0xD235, // display 5 byte sprite at coordinates V2, V3
];

let countUp = [
  0x613C, // load delay timer (60) to register 1 
  0x620F, // load x coordinate to register 2
  0x6302, // load y coordinate to register 3
  0x6000, // Load value 0x00 to register 0

  0xF029, // Move I to digit pointed by register 0
  0xF115, // load register 1 to delay timer
  0x00E0, // clear screen
  0xD235, // display 5 byte sprite at coordinates V2, V3
  0XF407, // Load the value of delay timer to register 4
  0x3400, // Skip next instruction if delay timer is 0
  0x1210, // Jump two lines back

  0x7001, // Add 1 to register 0
  0x1208, // Jump to instruction 5
];

let ikiloop = [
  0x613C,
  0x1200,
]

let canvas = document.getElementById("chip8-canvas");

let chip = wasm.Chip.new();
chip.load_instructions(countUp)

const wasmCycle = () => {
  chip.trigger_cycle();
  let display_string = chip.display_as_str();

  canvas.textContent = display_string;
  requestAnimationFrame(wasmCycle);

}

wasmCycle()

