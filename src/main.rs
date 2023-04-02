use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read_from_file(filename: &str) -> [usize; 256]{
    let mut memory = [0; 256];
    let mut counter = 0;
    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let tmp = line.unwrap();
        println!("{}", tmp);
        if tmp.split(" ").nth(0).unwrap() == "start" {
            let pos = tmp.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            for j in counter..=pos {
                memory[j] = 0;
                
            }
            counter = pos;
            continue;
        }else{
            // from string to hex to usize
            if tmp.contains("x") {
                memory[counter] = usize::from_str_radix(&tmp.split("x").nth(1).unwrap(), 16).unwrap();
            }else{
                memory[counter] = tmp.parse::<usize>().unwrap();
            }
        }
        counter += 1;
    }
    memory
}

fn main() {
    let mut pc = 0;
    let mut addr = read_from_file("memory.txt");
    
    println!("pc: {}, addr: {:?}\n", pc, addr);
    loop {
        // fetch
        let opcode = addr[pc];
        match opcode {
            0x1 => {
                // add
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] + addr[b];
                pc += 4;
            }
            0x2 => {
                // sub
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] - addr[b];
                pc += 4;
            }
            0x3 => {
                // mul
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] * addr[b];
                pc += 4;
            }
            0x31 => {
                // fibonaci
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] + addr[b];
                addr[a] = addr[b];
                addr[b] = addr[c];
                pc += 4;

            }
            0x4 => {
                // div
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] / addr[b];
                pc += 4;
            }
            0x5 => {
                // mod
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] % addr[b];
                pc += 4;
            }
            0x6 => {
                // mov
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                addr[b] = addr[a];
                pc += 3;
            }
            0x7 => {
                // jmp
                let a = addr[pc + 1];
                pc = addr[a];
            }
            0x8 => {
                // jz (jump if zero)
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                if addr[a] == 0 {
                    pc = addr[b];
                } else {
                    pc += 3;
                }
            }
            0x9 => {
                // jnz (jump if not zero)
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                if addr[a] != 0 {
                    pc = addr[b];
                } else {
                    pc += 3;
                }
            }
            _ => {
                println!("unknown opcode: {}", opcode);
                println!("pc = {}", pc);
                break;
        }
    }
        println!("pc: {}, addr: {:?}\n", pc, addr);
        // wait for 500 ms
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
