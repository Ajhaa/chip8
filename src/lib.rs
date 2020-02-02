mod utils;
mod chip;

use wasm_bindgen::prelude::*;

use chip::Chip;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub fn chip_test() {
    let mut system = Chip::new();
    let fibonacci = vec![
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


    system.load_instructions(&fibonacci);
    
    while system.pc - 0x200 < fibonacci.len() * 2 {
        system.cycle();
    }

    log("Memory:");
    for mem in system.mem_dump(0x400, 0x40A) {
        log(&mem);
    }
}
