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
  0x673C, // load delay timer to register 7 
  0x6600, // Load value 0x00 to register 6

  0xF715, // load register 7 to delay timer
  0x00E0, // clear screen

  0x680F, // load x coordinate to register 8
  0x6902, // load y coordinate to register 9

  0xF029, // Move I to digit pointed by register 0
  0xD895, // display 5 byte sprite at coordinates V8, V9

  0x7808, // add two to register 8 

  0xF129, // Move I to digit pointed by register 1
  0xD895, // display 5 byte sprite at coordinates V8, V9

  0x7808, // add two to register 8 

  0xF229, // Move I to digit pointed by register 2
  0xD895, // display 5 byte sprite at coordinates V8, V9

  0XF407, // Load the value of delay timer to register 4
  0x3400, // Skip next instruction if delay timer is 0
  0x121C, // Jump two lines back

  0xA200, // Set I to 200
  0x7601, // Add 1 to register 6
  0xF633, // Load the decimal representation of V6 to memory
  0xF265, // load the digits to registers 0-2
  0x1204, // Jump to instruction 3
];


let canvas = document.getElementById("chip8-canvas");

let chip = wasm.Chip.new();
chip.load_instructions(countUp)

const wasmCycle = () => {
  chip.cycle_until_draw();
  let display_string = chip.display_as_str();

  canvas.textContent = display_string;
  requestAnimationFrame(wasmCycle);

}

wasmCycle()

