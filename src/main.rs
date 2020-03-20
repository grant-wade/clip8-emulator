// Crates used for this project //
extern crate rand;

// Standard Library Modules //
use std::io;
use std::fs::File;
use std::io::prelude::*;

// Modules From Crates.io //
use rand::Rng;


/// A representation of chip8 ram
struct ChipMemory {
    /// a vector representing the ram
    ram: Vec<u8>,
    /// program start location
    start: usize
}

impl ChipMemory {
    /// Init a chip8 memory structure 
    fn init() -> Self {
        ChipMemory {
            ram: vec![0; 4096], // Size of chip8 ram
            start: 512
        }
    }

    /// Load a binary into 
    /// 
    /// # Arguments
    /// 
    /// * `rom` - a Vec<u8> holding rom contents
    fn load_bytes(&mut self, rom: Vec<u8>) {
        let len = rom.len();
        for i in 0..len {
            self.ram[i + self.start] = rom[i]
        }
    }

    /// Set a byte in ram to a passed value
    /// 
    /// # Arguments
    /// 
    /// * `loc` - location to set
    /// * `val` - value to set with
    fn set_byte(&mut self, loc: u16, val: u8) {
        self.ram[loc as usize] = val;
    }

    fn get_byte(&self, loc: u16) -> u8 {
        self.ram[loc as usize]
    }

    /// Dump the Chip8 memory into the console as
    /// hex encoded strings. 
    fn dump_ram(&self) {
        let len = self.ram.len();
        for i in 0..len {
            if i % 2 == 0 {
                print!(" ");
            }
            if i % 32 == 0 {
                println!("");
            }
            print!("{:02x}", self.ram[i]);
        }
    }

    /// Load a file from disk and write its bytes into 
    /// the Chip8 memory. 
    /// 
    /// # Arguments
    /// 
    /// * `rom_file` - the filename to open and read from
    fn load_rom_file(&mut self, rom_file: &str) -> io::Result<()> {
        // Load bytes from file
        let mut file = File::open(rom_file)?;

        // Create vector to hold rom
        // Capacity of 3584 is max size of Chip8 rom
        let mut rom: Vec<u8> = Vec::with_capacity(3584);

        // Read rom into vector
        file.read_to_end(&mut rom)?;

        // Load bytes into chip8 ram
        self.load_bytes(rom);
        Ok(())
    }
}


/// A struct representing the chip8 registers
struct ChipRegisters {
    /// General purpose registers
    gp_reg: Vec<u8>, 
    /// Address call stack
    stack: Vec<u16>, 
    /// Register I, address storage
    i_reg: u16,      
    /// Delay timer register
    d_reg: u8,       
    /// Sound timer register
    s_reg: u8,       
    /// Program counter, current addr
    pc_reg: u16,     
    /// Stack pointer
    sp_reg: usize,      
}

impl ChipRegisters {
    /// Init a Chip8 register struct
    fn init() -> Self {
        let mut gp_reg = vec![0; 16];
        let mut stack = vec![0; 16];
        ChipRegisters {
            gp_reg,
            stack,
            i_reg: 0,
            d_reg: 0,
            s_reg: 0,
            pc_reg: 0,
            sp_reg: 0,
        }
    }

    /// Set a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which general purpose register
    /// * `value` - value to fill register with
    fn set_gp(&mut self, index: usize, value: u8) {
        self.gp_reg[index] = value;
    }

    /// Get the value of a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which register to get
    fn get_gp(&self, index: usize) -> u8 {
        self.gp_reg[index]
    }

    /// Add a value to a general purpose register
    /// 
    /// # Arguments
    /// 
    /// * `index` - which general purpose register
    /// * `value` - u8 value to add to register
    fn add_gp(&mut self, index: usize, value: u8) {
        self.gp_reg[index] += value;
    }

    /// Set the value of the I register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what to put in I register
    fn set_i(&mut self, value: u16) {
        self.i_reg = value;
    }

    /// Get the value of the I register
    fn get_i(&self) -> u16 {
        self.i_reg
    }

    /// Set the value in the pc register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what to put in pc register
    fn set_pc(&mut self, value: u16) {
        self.pc_reg = value;
    }

    /// Increment the value of the pc register by 2
    fn incr_pc(&mut self) {
        self.pc_reg += 2;
    }

    /// Get the value of the pc register
    fn get_pc(&self) -> u16 {
        self.pc_reg
    }

    /// Get the value of the delay register
    fn get_d(&self) -> u8 {
        self.d_reg
    }

    /// Set the value of the delay register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what value to put in the pc register
    fn set_d(&mut self, value: u8) {
        self.d_reg = value;
    }

    /// Get the value of the sound delay register
    fn get_s(&self) -> u8 {
        self.s_reg
    }

    /// Set the value of the sound delay register
    /// 
    /// # Arguments
    /// 
    /// * `value` - what value to put in the sound register
    fn set_s(&mut self, value: u8) {
        self.s_reg = value;
    }

    /// Decrement the delay register if value is not 0
    fn decr_d(&mut self) {
        if self.d_reg > 0 {
            self.d_reg -= 1;
        }
    }

    /// Decrement the sound register if vlaue is not 0
    fn decr_s(&mut self) {
        if self.s_reg > 0 {
            self.s_reg -= 1;
        }
    }

    /// Push a address onto the stack, increment stack pointer
    /// 
    /// # Arguments
    /// 
    /// * `addr` - address to push to the stack
    fn push_stack(&mut self, addr: u16) {
        self.stack[self.sp_reg as usize] = addr;
        self.sp_reg += 1;
    }

    /// Pop an address from the stack, decrementing sp
    fn pop_stack(&mut self) -> u16 {
        self.sp_reg -= 1;
        let addr = self.stack[self.sp_reg];
        return addr;
    }
}


/// A struct representing the chip8 display
struct ChipDisplay {
    /// A boolean vector representing the display
    display: Vec<bool>,
    /// String to divide display with
    divider: String
}

impl ChipDisplay {
    /// Initialize the chip8 display struct
    fn init() -> Self {
        let divider = match String::from_utf8(vec![b'-'; 64]) {
            Ok(s) => s,
            Err(_) => String::from("ERROR")
        };

        ChipDisplay {
            display: vec![false; 2048],
            divider
        }
    }

    fn clear_display(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                let pos: usize = y * 64 + x;
                self.display[pos] = false;
            }
        }
    }

    /// Draw the chip8 display in the terminal
    fn draw_display(&self) {
        println!("|{}|", self.divider);
        for x in 0..32 {
            print!("|");
            for y in 0..64 {
                let pos: usize = x * 64 + y;
                if self.display[pos] == true {
                    print!("#")
                }
                else {
                    print!(" ")
                }
            }
            println!("|");
        }
        println!("|{}|", self.divider);
    }
}


struct Opcode {
    h1: u16,
    v1: u16, 
    v2: u16, 
    v3: u16
}

impl Opcode {
    fn new(opcode: u16) -> Self {
        let h1 = (opcode >> 12) & 0xf;
        let v1 = (opcode >> 8) & 0xf;
        let v2 = (opcode >> 4) & 0xf;
        let v3 = (opcode) & 0xf;
        Opcode {h1, v1, v2, v3}
    }
}


struct ChipSystem {
    registers: ChipRegisters,
    display: ChipDisplay,
    ram: ChipMemory,
}

impl ChipSystem {
    fn new(reg: ChipRegisters, disp: ChipDisplay, ram: ChipMemory) -> Self {
        ChipSystem {
            registers: reg,
            display: disp,
            ram: ram
        }
    }

    fn random_byte() -> u8 {
        let mut rng = rand::thread_rng();
        let rand_byte: u8 = rng.gen();
        return rand_byte;
    }

    fn ex_opcode(&mut self, opcode: u16) {
        println!("Current Opcode: {:04x}", opcode);
        let comps = Opcode::new(opcode);

        match comps.h1 {
            0x0 => {
                match comps.v3 {
                    // CLS - Clear Display
                    0 => self.display.clear_display(),
                    // RET - Return from subroutine
                    14 => {
                        let pc: u16 = self.registers.pop_stack();
                        self.registers.set_pc(pc);
                    },
                    // Not a valid opcode
                    _ => panic!("Invalid Opcode: {:04x}", opcode)
                }
            },
            // JP - Jumps to address without modifying stack
            0x1 => {
                let pc: u16 = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_pc(pc);
            },
            // CALL - Jump to address with push to stack
            0x2 => {
                let pc: u16 = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.push_stack(pc);
            },
            // SE Vx, Byte - Skip instruction if Vx == Byte
            0x3 => {
                let comp_val: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                let reg_val: u8 = self.registers.get_gp(comps.v1 as usize);
                if comp_val == reg_val {
                    self.registers.incr_pc();
                }
            },
            // SNE Vx, Byte - Skip instruction if Vx != Byte
            0x4 => {
                let comp_val: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                let reg_val: u8 = self.registers.get_gp(comps.v1 as usize);
                if comp_val != reg_val {
                    self.registers.incr_pc();
                }
            },
            // SE Vx, Vy - Skip instruction if Vx == Vy
            0x5 => {
                let reg_x_val: u8 = self.registers.get_gp(comps.v1 as usize);
                let reg_y_val: u8 = self.registers.get_gp(comps.v2 as usize);
                if reg_x_val == reg_y_val {
                    self.registers.incr_pc();
                }
            },
            // LD Vx, Byte - Load byte value into Vx (Vx = Byte)
            0x6 => {
                let value: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                self.registers.set_gp(comps.v1 as usize, value);
            },
            // ADD Vx, Byte - Add byte value to Vx (Vx += Byte) no carry flag
            0x7 => {
                let value: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                self.registers.add_gp(comps.v1 as usize, value);
            },
            0x8 => {
                match comps.v3 {
                    // LD Vx, Vy - Store value of Vy in Vx (Vx = Vy)
                    0x0 => {
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        self.registers.set_gp(comps.v1 as usize, reg_y_val);
                    },
                    // OR Vx, Vy - Bitwise OR on Vx, Vy store in Vx (Vx = Vx | Vy)
                    0x1 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let value = reg_x_val | reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, value);
                    },
                    // AND Vx, Vy - Bitwise AND on Vx, Vy store in Vx (Vx = Vx & Vy)
                    0x2 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let value = reg_x_val & reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, value);
                    },
                    // XOR Vx, Vy - Bitwise XOR on Vx, Vy store in Vx (Vx = Vx ^ Vy)
                    0x3 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let value = reg_x_val ^ reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, value);
                    },
                    // ADD Vx, Vy - Add Vx, Vy if > 255 set Vf to 1 (Vx = Vx + Vy)
                    0x4 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        let holder: u16 = reg_x_val as u16 + reg_y_val as u16;
                        if holder > 255 { // Carry occurred
                            self.registers.set_gp(15, 1);
                        } else {
                            self.registers.set_gp(15, 0);
                        }
                        self.registers.set_gp(comps.v1 as usize, (holder & 0xff) as u8);
                    },
                    // SUB Vx, Vy - Subtract Vx, Vy if Vx < Vy set Vf to 0 (Vx = Vx - Vy)
                    0x5 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        if reg_x_val < reg_y_val {
                            self.registers.set_gp(15, 0);
                        } else {
                            self.registers.set_gp(15, 1);
                        }
                        let holder = reg_x_val - reg_y_val;
                        self.registers.set_gp(comps.v1 as usize, holder);
                    },
                    // SHR Vx, _ - Shift Vx right by 1, set Vf to LSB (Vx = Vx >> 1)
                    0x6 => {
                        let mut reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        self.registers.set_gp(15, reg_x_val & 0x01);
                        reg_x_val = reg_x_val >> 1; 
                        self.registers.set_gp(comps.v1 as usize, reg_x_val);
                    },
                    // SUBN Vx, Vy - Subtract Vy, Vx if Vy < Vx set Vf to 0 (Vx = Vy - Vx)
                    0x7 => {
                        let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                        if reg_y_val < reg_x_val {
                            self.registers.set_gp(15, 0);
                        } else {
                            self.registers.set_gp(15, 1);
                        }
                        let holder = reg_y_val - reg_x_val;
                        self.registers.set_gp(comps.v1 as usize, holder);
                    },
                    // SHL Vx, _ - Shift Vx left by 1, set Vf to MSB (Vx = Vx << 1)
                    0xE => {
                        let mut reg_x_val = self.registers.get_gp(comps.v1 as usize);
                        self.registers.set_gp(15, reg_x_val & 0x80);
                        reg_x_val = reg_x_val << 1;
                        self.registers.set_gp(comps.v1 as usize, reg_x_val);
                    },
                    _ => panic!("Invalid Opcode: {:04x}", opcode)
                }
            },
            // SNE Vx, Vy - Skip next instruction if Vx != Vy
            0x9 => {
                let reg_x_val = self.registers.get_gp(comps.v1 as usize);
                let reg_y_val = self.registers.get_gp(comps.v2 as usize);
                if reg_x_val != reg_y_val {
                    self.registers.incr_pc();
                }
            },
            // LD I, Addr (12bit) - Register I is set to the address
            0xA => {
                let value = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_i(value);
            },
            // JP V0, Addr (12bit) - Jump to the location Addr + V0
            0xB => {
                let reg_v0_val = self.registers.get_gp(0);
                let address = (comps.v1 << 8) + (comps.v2 << 4) + comps.v3;
                self.registers.set_pc(address + reg_v0_val as u16);
            },
            // RND Vx, Byte - Set Vx to Byte & Random byte
            0xC => {
                let byte_val: u8 = ((comps.v2 as u8) << 4) + comps.v3 as u8;
                let value = byte_val & ChipSystem::random_byte(); 
                self.registers.set_gp(comps.v1 as usize, value);
            },
            // DRW Vx, Vy, N - Draw a sprite coord (Vx, Vy) with height N
            // TODO: Implement
            0xD => {},
            0xE => {
                match (comps.v2 << 4) + comps.v3 {
                    // SKP Vx - Skip next instruction if key (0-15) is pressed
                    // TODO: Implement
                    0x9E => {},
                    // SKNP Vx - Skip next instruction if key (0-15) is not pressed
                    // TODO: Implement
                    0xA1 => {}
                    _ => panic!("Invalid Opcode: {:04x}", opcode)
                }
            },
            0xF => {
                match (comps.v2 << 4) + comps.v3 {
                    // LD Vx, DT - Set Vx to the value of the delay timer
                    0x07 => {
                        let delay_val = self.registers.get_d();
                        self.registers.set_gp(comps.v1 as usize, delay_val);
                    },
                    // LD Vx, K - Wait for keypress (halt), put key value in Vx
                    // TODO: Implement
                    0x0A => {},
                    // LD DT, Vx - Set the delay timer to the value in Vx
                    0x15 => {
                        let delay_val = comps.v1 as u8;
                        self.registers.set_d(delay_val);
                    },
                    // LD ST, Vx - Set the sound timer to the value in Vx
                    0x18 => {
                        let delay_val = comps.v1 as u8;
                        self.registers.set_s(delay_val);
                    },
                    // ADD I, Vx - Set register I to I + Vx
                    0x1E => {
                        let i_val = self.registers.get_i();
                        let value = i_val + comps.v1;
                        self.registers.set_i(value);
                    },
                    // LD F, Vx - Set I to the location of sprite (I = Vx * 5)
                    // TODO: Implement 
                    0x29 => {},
                    // LD B, Vx - Place the BCD of Vx in I (Hundreds), I+1 (Tens), I+2 (Ones)
                    0x33 => {
                        let reg_val = self.registers.get_gp(comps.v1 as usize);
                        let i_val = self.registers.get_i();
                        let ones = reg_val % 10;
                        let tens = (reg_val / 10) % 10;
                        let huns = (reg_val / 100) % 10;
                        self.ram.set_byte(i_val, huns);
                        self.ram.set_byte(i_val + 1, tens);
                        self.ram.set_byte(i_val + 2, ones);
                    },
                    // LD I, Vx - Stores V0 to Vx in memory starting at address I, then (I = I + x + 1)
                    0x55 => {
                        let i_val = self.registers.get_i();
                        let x_range = comps.v1;
                        let mut cur_reg: u8;
                        for loc in 0..x_range {
                            cur_reg = self.registers.get_gp(loc as usize);
                            self.ram.set_byte(i_val + loc, cur_reg);
                        }
                        let new_i = i_val + x_range + 1;
                        self.registers.set_i(new_i);
                    },
                    // LD Vx, I - Fills V0 to Vx with values from memory starting at address then (I = I + x + 1)
                    0x65 => {
                        let i_val = self.registers.get_i();
                        let x_range = comps.v1;
                        let mut cur_reg: u8;
                        for loc in 0..x_range {
                            cur_reg = self.ram.get_byte(loc);
                            self.registers.set_gp(i_val as usize + loc as usize, cur_reg);
                        }
                    },
                    _ => panic!("Invalid Opcode: {:04x}", opcode)
                }
            }
            _ => panic!("Invalid Opcode header: {:02x}", comps.h1)
        }
    }
}


fn init_chip8() -> ChipSystem {
    let ram = ChipMemory::init();
    let disp = ChipDisplay::init();
    let reg = ChipRegisters::init();
    ChipSystem::new(reg, disp, ram)
}



fn main() {
    let mut sys = init_chip8();
    let res = sys.ram.load_rom_file("roms/Trip8_Demo.ch8");
    match res {
        Ok(_) => println!("Rom file sucessfully read"),
        Err(e) => println!("Could not read rom file: {}", e)
    }
    // sys.ram.dump_ram();

    sys.display.draw_display();
}
