pub mod intel4004{
    // Technically this processor is big endian, but I'm sticking to little endian cause it makes no difference at the end of the day
    #[repr(u8)]
    #[derive(PartialEq, Eq)]
    pub enum Instruction{ // Refer to data sheet http://datasheets.chipdb.org/Intel/MCS-4/datashts/intel-4004.pdf
        NOP = 0x0,
        JCN = 0x1,
        FIMSRC = 0x2, // SRC if only one word, FIM if two words
        FINJIN = 0x3, // FIN if last bit is 0, JIN if last bit is 1
        JUN = 0x4,
        JMS = 0x5,
        INC = 0x6,
        ISZ = 0x7,
        ADD = 0x8,
        SUB = 0x9,
        LD = 0xA,
        XCH = 0xB,
        BBL = 0xC,
        LDM = 0xD,
        CLB = 0xF0,
        CLC = 0xF1,
        IAC = 0xF2,
        CMC = 0xF3,
        CMA = 0xF4,
        RAL = 0xF5,
        RAR = 0xF6,
        TCC = 0xF7,
        DAC = 0xF8,
        TCS = 0xF9,
        STC = 0xFA,
        DAA = 0xFB,
        KBP = 0xFC,
        DCL = 0xFD,
        WRM = 0xE0,
        WMP = 0xE1,
        WRR = 0xE2,
        WPM = 0xE3,
        WR0 = 0xE4,
        WR1 = 0xE5,
        WR2 = 0xE6,
        WR3 = 0xE7,
        SBM = 0xE8,
        RDM = 0xE9,
        RDR = 0xEA,
        ADM = 0xEB,
        RD0 = 0xEC,
        RD1 = 0xED,
        RD2 = 0xEE,
        RD3 = 0xEF
        
    }
    #[derive(Debug)]
    pub struct cpu{ // we only have u8, u16, u32, u64, u128 to work with so the closest match will have to do
        pub ixr: [u8; 16], // Index registers consist of 16 registers of 4 bits each
        pub rom: [u8; 4096], // ROM consists of 4096 8-bit words (32768 bits total), 16 pages of 256 bits
        pub ram_d: [u8; 1024], // RAM Data "characters", consists of 1024 4-bit data "characters"
        pub ram_s: [u8; 256], // RAM Status "characters", consists of 256 4-bit status "characters"
        pub pc: u16, // 12-bit program counter
        pub stack: [u16; 3], // Subroutine stack contains 3 layers, each should store a 12-bit address
        pub stack_ptr: u8, // 4-bit Stack pointer
        pub acc: u8, // 4-bit accumulator
        pub carry: u8, // carry bit
        pub test: u8 // Test pin
        
    }
    impl cpu{
        pub fn execute(&mut self, max_cycle_count: i32){
            // Initialise
            self.pc = 0;
            // Todo rest
            let mut cycle = 0;

            while cycle < max_cycle_count{
                let op = self.rom[cycle as usize];
                let op_instr_only = (self.rom[cycle as usize] & 0xF0)>>4;
                
                match op_instr_only{
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
                            // SRC
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
                    }
                    _=>{panic!()}
                };

                println!("{}",self.pc);

                
            }
        }

        pub fn op_jcn(&mut self, instr: u16){ // This is a 2 word instruction, hence u16 instead of u8
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

        pub fn op_fim(&mut self, instr: u16){
            // In the first word, the last three bytes (exluding the tailing 0) refers to the index register pair in which the data is to be stored
            let words = instr.to_ne_bytes();
            let index_reg_pair = words[0] & 0xE;
            
            println!("{}",words[1]);

            self.ixr[index_reg_pair as usize] = (words[1] & 0xF0)>>4;
            self.ixr[(index_reg_pair+1) as usize] = words[1] & 0x0F;


            self.pc += 2;

        }

        pub fn op_fin(&mut self, instr: u8){
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

        pub fn op_jin(&mut self, instr: u8){
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

        pub fn op_jun(&mut self, instr: u16){
            // Jump directly to rom address
            let address = instr & 0xFFF;
            self.pc = address;
        }

        pub fn op_jms(&mut self, instr:u16){
            // Jump to subroutine ROM address, and save old address (PC) in the stack
            self.stack[self.stack_ptr as usize] = self.pc;

            if self.stack_ptr == 2 {
                self.stack_ptr = 0;
            } else {
                self.stack_ptr+=1;
            }

            self.pc = instr;

        }

        pub fn op_inc(&mut self, instr:u8){
            // Increment register RRRR
            let index_addr = instr & 0xF;
            if self.ixr[index_addr as usize] < 15{
                self.ixr[index_addr as usize] = self.ixr[index_addr as usize]+1;
            } else {
                self.ixr[index_addr as usize] = 0;
            }

            self.pc += 1;
        }

        pub fn op_isz(&mut self, instr:u16){
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

        pub fn op_add(&mut self, instr:u8){
            // Add value in register to accumulator with carry
            let index_addr = instr & 0xF;

            let result = self.acc + self.ixr[index_addr as usize];

            if result <= 15 {
                self.acc = result;
            } else {
                self.acc = 0;
                self.carry = 1;
            }

            self.pc += 1;

        }

        pub fn op_sub(&mut self, instr:u8){
            // Subtract value in register to accumulator with carry
            let index_addr = instr & 0xF;

            let result = self.acc - self.ixr[index_addr as usize];

            if result <= 15 && result > 0{
                self.acc = result;
            } else {
                self.acc = 0;
                self.carry = 1;
            }

            self.pc += 1;

        }

        pub fn op_ld(&mut self, instr: u8){
            // Load contents of register RRRR into accumulator
            let index_addr = instr & 0xF;
            self.acc = self.ixr[index_addr as usize];

            self.pc += 1;
        }

        pub fn op_xch(&mut self, instr: u8){
            // Exchange contents of index register and accumulator
            let index_addr = instr & 0xF;
            let acc_temp = self.acc;
            let reg_temp = self.ixr[index_addr as usize];

            self.ixr[index_addr as usize] = acc_temp;
            self.acc = reg_temp;

            self.pc += 1;
        }


    }

}