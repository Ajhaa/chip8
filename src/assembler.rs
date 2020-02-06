//TODO the whole thing

pub fn assemble(symbols: &Vec<&str>) -> Vec<u16> {
    let mut program: Vec<u16> = Vec::new();
    for command in symbols {
        program.push(translate(command));
    } 
    program
}

fn translate(command: &str) -> u16 {
    let parts: Vec<&str> = command.split(" ").collect();
    match parts[0] {
        "LD" => {
            return load(parts);
        },
        _ => {
            panic!("could not translate instruction {}", command);
        }
    }

    0
}

fn load(parts: Vec<&str>) -> u16{
    let target = parts[1];
    if &target[0..1] == "V" {
        let reg = target[1..2].parse::<u16>().unwrap();
        let value = parts[2];
        let number = value.parse::<u8>();


        if let Ok(x) = number {
            let lower = 0x8000 + (reg << 8);
        }
        
    }
    0
}