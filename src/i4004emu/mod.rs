// Technically this processor is big endian, but I'm sticking to little endian cause it makes no difference at the end of the day
#[derive(Debug)]
pub struct CPU{ // we only have u8, u16, u32, u64, u128 to work with so the closest match will have to do
    pub ixr: [u8; 16], // Index registers consist of 16 registers of 4 bits each
    pub rom: [u8; 4096], // ROM consists of 4096 8-bit words (32768 bits total), 16 pages of 256 bits
    pub ram_d: [u8; 1024], // RAM Data "characters", consists of 1024 4-bit data "characters"
    pub ram_s: [u8; 256], // RAM Status "characters", consists of 256 4-bit status "characters"
    pub ram_bank: u8, // RAM Bank
    pub ram_addr: u8, // RAM Address register
    pub pc: u16, // 12-bit program counter
    pub stack: [u16; 3], // Subroutine stack contains 3 layers, each should store a 12-bit address
    pub stack_ptr: u8, // 4-bit Stack pointer
    pub acc: u8, // 4-bit accumulator
    pub carry: u8, // carry bit
    pub test: u8 // Test pin
    
}
impl CPU{
    pub fn execute(&mut self, max_cycle_count: i32){
        // Initialise
        self.pc = 0;
        // Todo rest
        let mut cycle = 0;

        while cycle < max_cycle_count{
            let op = self.rom[cycle as usize];
            // let op_instr_only = (self.rom[cycle as usize] & 0xF0)>>4;
            // let op_last_four = self.rom[cycle as usize] & 0xF;
            
            match (self.rom[cycle as usize] & 0xF0)>>4{
                0x0 => {cycle += 1}, // NOP
                0x1 => {
                    let next_op = self.rom[(cycle+1) as usize];
                    self.op_jcn(u16::from_ne_bytes([op, next_op]));
                    cycle += 2;
                },
                0x2 => {
                    if op & 0x1 == 0{
                        let next_op = self.rom[(cycle+1) as usize];
                        self.op_fim(u16::from_ne_bytes([op,next_op]));
                        cycle += 2;
                    } else {
                        self.op_src(op);
                        cycle += 1;
                    }
                },
                0x3 => {
                    if op & 0x1 == 0{
                        self.op_fin(op);
                        cycle += 1;
                    } else {
                        self.op_jin(op);
                        cycle += 1;  
                    }
                },
                0x4 => {
                    let next_op = self.rom[(cycle+1) as usize];
                    self.op_jun(u16::from_ne_bytes([op,next_op]));
                    cycle += 2;
                },
                0x5 => {
                    let next_op = self.rom[(cycle+1) as usize];
                    self.op_jms(u16::from_ne_bytes([op,next_op]));
                    cycle += 2;
                },
                0x6 => {
                    self.op_inc(op);
                    cycle += 1;
                },
                0x7 => {
                    let next_op = self.rom[(cycle+1) as usize];
                    self.op_isz(u16::from_ne_bytes([op,next_op]));
                    cycle += 2;
                },
                0x8 => {
                    self.op_add(op);
                    cycle += 1;
                },
                0x9 => {
                    self.op_sub(op);
                    cycle += 1;
                },
                0xA => {
                    self.op_ld(op);
                    cycle += 1;
                },
                0xB => {
                    self.op_xch(op);
                    cycle += 1;
                },
                0xC => {
                    self.op_bbl(op);
                    cycle += 1;
                },
                0xD => {
                    self.op_ldm(op);
                    cycle += 1;
                },
                0xF => {
                    match self.rom[cycle as usize] & 0xF{
                        0x0 => {
                            self.op_clb();
                            cycle += 1;
                        },
                        0x1 => {
                            self.op_clc();
                            cycle += 1;
                        },
                        0x2 => {
                            self.op_iac();
                            cycle += 1;
                        },
                        0x3 => {
                            self.op_cmc();
                            cycle += 1;
                        },
                        0x4 => {
                            self.op_cma();
                            cycle += 1;
                        },
                        0x5 => {
                            self.op_ral();
                            cycle += 1;
                        },
                        0x6 => {
                            self.op_rar();
                            cycle += 1;
                        },
                        0x7 => {
                            self.op_tcc();
                            cycle += 1;
                        },
                        0x8 => {
                            self.op_dac();
                            cycle += 1;
                        },
                        0x9 => {
                            self.op_tcs();
                            cycle += 1;
                        },
                        0xA => {
                            self.op_stc();
                            cycle += 1;
                        },
                        0xB => {
                            self.op_daa();
                            cycle += 1;
                        },
                        0xC => {
                            self.op_kbp();
                            cycle += 1;
                        },
                        0xD => {
                            self.op_dcl();
                            cycle += 1;
                        }
                        _=>{panic!()}
                    }
                }
                _=>{panic!()}
            };
        }
    }

    fn op_jcn(&mut self, instr: u16){ // This is a 2 word instruction, hence u16 instead of u8
        /*  
            If c1 = 0, Do not invert jump condition
            If c1 = 1, Invert jump condition
            If c2 = 1, Jump if accumulator content is zero
            If c3 = 1, Jump if the carry/link content is 1
            If c4 = 1, Jump if test signal is zero
        */
        let words = instr.to_ne_bytes(); // instr.to_ne_bytes() splits a 16 bit int into two 8 bit ints
        let c1 = words[0] & 0x8; 
        let c2 = words[0] & 0x4;
        let c3 = words[0] & 0x2;
        let c4 = words[0] & 0x1;

        if (c1 == 1 && ((c2 == 1 && self.acc != 0) || (c3 == 1 && self.carry == 1) || (c4 == 1 && self.test == 0))) || (c1 == 0 && !((c2 == 1 && self.acc != 0) || (c3 == 1 && self.carry == 1) || (c4 == 1 && self.test == 0))){
            self.pc = u16::from_ne_bytes([words[1],0xFF]); // Need to test, unsure
        } else {
            self.pc += 2;
        }
    }

    fn op_fim(&mut self, instr: u16){
        // In the first word, the last three bytes (exluding the tailing 0) refers to the index register pair in which the data is to be stored
        let words = instr.to_ne_bytes();
        let index_reg_pair = words[0] & 0xE;
        
        println!("{}",words[1]);

        self.ixr[index_reg_pair as usize] = (words[1] & 0xF0)>>4;
        self.ixr[(index_reg_pair+1) as usize] = words[1] & 0x0F;


        self.pc += 2;

    }

    fn op_fin(&mut self, instr: u8){
        // ROM[<Address from index reg pair 0>] is copied to index pair register number supplied
        
        let index_reg_pair = instr & 0xE;
        let page_num = self.pc/255; 
        let ixr_val_u16: u16 = self.ixr[0].into();

        if self.pc % 255 == 0 && self.pc != 0{
            let rom_addr: u16 = (page_num+1)*255 + ixr_val_u16;

            self.ixr[index_reg_pair as usize] = (self.rom[rom_addr as usize] & 0xF0)>>4;
            self.ixr[(index_reg_pair+1) as usize] = self.rom[rom_addr as usize] & 0x0F;
            
        } else {

            let rom_addr: u16 = page_num*255 + ixr_val_u16;

            self.ixr[index_reg_pair as usize] = (self.rom[rom_addr as usize] & 0xF0)>>4;
            self.ixr[(index_reg_pair+1) as usize] = self.rom[rom_addr as usize] & 0x0F;
        }

        self.pc += 1;

    }

    fn op_jin(&mut self, instr: u8){
        // Jump indirect, to address stored in index registers
        let index_reg_pair = instr & 0xE;
        let page_num: u16 = self.pc/255;
        let pc_out: u16 = ((self.ixr[index_reg_pair as usize])<<4 & self.ixr[(index_reg_pair+1) as usize]).into();

        if self.pc % 255 == 0{
            self.pc = pc_out+(page_num+1)*255;
            
        } else {
            self.pc = pc_out+page_num*255;
    
        }

    }

    fn op_jun(&mut self, instr: u16){
        // Jump directly to rom address
        let address = instr & 0xFFF;
        self.pc = address;
    }

    fn op_jms(&mut self, instr:u16){
        // Jump to subroutine ROM address, and save old address (PC) in the stack
        self.stack[self.stack_ptr as usize] = self.pc;

        if self.stack_ptr == 2 {
            self.stack_ptr = 0;
        } else {
            self.stack_ptr+=1;
        }

        self.pc = instr;

    }

    fn op_inc(&mut self, instr:u8){
        // Increment register RRRR
        let index_addr = instr & 0xF;
        if self.ixr[index_addr as usize] < 15{
            self.ixr[index_addr as usize] = self.ixr[index_addr as usize]+1;
        } else {
            self.ixr[index_addr as usize] = 0;
        }

        self.pc += 1;
    }

    fn op_isz(&mut self, instr:u16){
        // Increment register RRRR and jump to address supplied in ROM
        let words = instr.to_ne_bytes();
        let index_addr = words[0] & 0xF;

        if self.ixr[index_addr as usize] < 15{
            self.ixr[index_addr as usize] = self.ixr[index_addr as usize]+1;
        } else {
            self.ixr[index_addr as usize] = 0;
        }

        self.pc = words[1].into();
    }

    fn op_add(&mut self, instr:u8){
        // Add value in register to accumulator with carry
        let index_addr = instr & 0xF;

        let result = self.acc + self.ixr[index_addr as usize];

        if result <= 15 {
            self.acc = result;
            self.carry = 0;
        } else {
            self.acc = 0;
            self.carry = 1;
        }

        self.pc += 1;

    }

    fn op_sub(&mut self, instr:u8){
        // Subtract value in register to accumulator with carry
        let index_addr = instr & 0xF;

        let result = self.acc - self.ixr[index_addr as usize];

        if result <= 15 && result > 0{
            self.acc = result;
            self.carry = 1;
        } else {
            self.acc = 0;
            self.carry = 0;
            
        }

        self.pc += 1;

    }

    fn op_ld(&mut self, instr: u8){
        // Load contents of register RRRR into accumulator
        let index_addr = instr & 0xF;
        self.acc = self.ixr[index_addr as usize];

        self.pc += 1;
    }

    fn op_xch(&mut self, instr: u8){
        // Exchange contents of index register and accumulator
        let index_addr = instr & 0xF;
        let acc_temp = self.acc;
        let reg_temp = self.ixr[index_addr as usize];

        self.ixr[index_addr as usize] = acc_temp;
        self.acc = reg_temp;

        self.pc += 1;
    }

    fn op_bbl(&mut self, instr: u8){
        // Move down one level in the stack, dump pc in there and dump DDDD in accumulator
        self.acc = instr & 0xF;
        self.pc = self.stack[self.stack_ptr as usize];

        if self.stack_ptr > 0{
            self.stack_ptr -= 1;
        } else {
            self.stack_ptr = 0;
        }
        
        self.pc += 1;
    }

    fn op_ldm(&mut self, instr: u8){
        // Load DDDD into accumulator
        self.acc = instr & 0xF;
        self.pc += 1;
    }

    fn op_clb(&mut self){
        // Clear accumulator and carry
        self.acc = 0;
        self.carry = 0;
        self.pc += 1;
    }

    fn op_clc(&mut self){
        // Clear carry
        self.carry = 0;
        self.pc += 1;
    }

    fn op_iac(&mut self){
        // Increment accumulator
        let result = self.acc + 1;

        if result <= 15 {
            self.acc = result;
            self.carry = 0;
        } else {
            self.acc = 0;
            self.carry = 1;
        }

        self.pc += 1;
    }

    fn op_cmc(&mut self){
        // Complement carry
        self.carry = !self.carry;
        self.pc += 1;
    }

    fn op_cma(&mut self){
        // Complement accumulator
        self.acc = !self.acc;
        self.pc += 1;
    }

    fn op_ral(&mut self){
        // Rotate left 
        let new_acc = (self.acc << 1 & 0xF) | self.carry;
        self.carry = (self.acc & 0x8) >> 3;
        self.acc = new_acc;

        self.pc += 1;

    }

    fn op_rar(&mut self){
        // Rotate right 
        let new_acc = (self.acc >> 1 & 0xF) | self.carry;
        self.carry = self.acc & 0x1;
        self.acc = new_acc;

        self.pc += 1;

    }

    fn op_tcc(&mut self){
        // transfer carry to accumulator
        self.acc = self.carry;
        self.carry = 0;

        self.pc += 1;
    }

    fn op_dac(&mut self){
        // Decrement accumulator
        let result = self.acc - 1;

        if result <= 15{
            self.acc = result;
            self.carry = 1;
        } else {
            self.acc = 0;
            self.carry = 0;
        }

        self.pc += 1;
    }

    fn op_tcs(&mut self){
        if self.carry == 1{
            self.acc = 0x9;
        } else {
            self.acc = 0xA;
        }
        self.carry = 0;
        self.pc += 1;
    }

    fn op_stc(&mut self){
        // Set carry to 1
        self.carry = 1;
        self.pc += 1;
    }

    fn op_daa(&mut self){
        if (self.acc > 9 || self.carry == 1) && self.acc <= 15{
            self.acc += 6;
        } else if (self.acc > 9 || self.carry == 1) && self.acc > 15{
            self.acc = 0;
        }

        self.pc += 1;
    }

    fn op_kbp(&mut self){
        match self.acc{
            0x0 => {
                self.acc = 0x0;
            },
            0x1 => {
                self.acc = 0x1;
            },
            0x2 => {
                self.acc = 0x2;
            },
            0x4 => {
                self.acc = 0x3;
            },
            0x8 => {
                self.acc = 0x4;
            },
            _=>{
                self.acc = 0xF;
            }
        };
    }

    fn op_dcl(&mut self){
        // todo
    }

    fn op_src(&mut self, instr:u8){
        // see data sheet
        let index_reg_pair = instr & 0xE;
        self.ram_addr = self.ixr[index_reg_pair as usize] & (self.ixr[(index_reg_pair+1) as usize])<<4;
        self.pc += 1;
    }

    fn op_wrm(&mut self){
        self.ram_d[self.ram_addr as usize] = self.acc;
        self.pc += 1;
    }

}
