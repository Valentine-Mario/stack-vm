pub mod vm_mod{
    use crate::stack::stack_mod;
    pub const CODE_TYPE_DATA_POSITIVE: u8 = 0;
    pub const CODE_TYPE_DATA_NEGATIVE: u8 = 3; //11

    pub const CODE_TYPE_INSTRUCTION: u8 = 1;

    
    //Stack VM will have its internal state structure to store current program counter, vm state, runtime stack, and program memory
    
    pub struct VM {
        //stack internal state;
        running: bool,
        //program counter
        pc: usize,
    
        //typ of value read
        type_info: u8,
        //data
        data: i32,
    
        stack: stack_mod::VMStack,
        //code area
        program_memory: Vec<u32>,
    }

    impl VM{
        pub fn new(stack_size: usize) -> VM {
            VM {
                program_memory: Vec::new(),
                pc: 0,
                type_info: 0,
                data: 0,
                running: true,
                stack: stack_mod::VMStack::new(stack_size),
            }
        }

        //Loads the user program from file to VM program_memory
        pub fn load_program(&mut self, instructions: &Vec<u32>) {
            println!("Loading program...");
            //Insert magic bits to beginning of the program memory, so that we can start pc from 1
            self.program_memory.push(0xBADC0DE);
            for instruction in instructions {
                self.program_memory.push(*instruction);
            }
            println!("Memory content : {:?}", self.program_memory);
        }

        fn exec(&mut self) {
            if self.current_instruction_type() == CODE_TYPE_DATA_POSITIVE || self.current_instruction_type() == CODE_TYPE_DATA_NEGATIVE {
                println!("Instruction type Data ({} = {} ) ", self.current_instruction_type(), self.current_data());
                self.stack.push(self.data);
            } else {
                //execute instruction
                println!("Instruction type Operation ({}) , ", self.current_instruction_type());
                self.do_primitive();
            }
        }

          /*main thread which executes VM modules
     */
    pub fn run(&mut self) {
        println!("Memory content : {:?}", self.program_memory);

        println!("Executing instructions...");
        self.set_running(true);
        while self.is_running() {
            self.fetch();
            self.decode();
            self.exec();
        }
        println!(" \nExecution completed");
    }

        fn do_primitive(&mut self) {
            match self.current_data() & 0xCfffff {
                0 => {
                    println!("[ HALT ] : Stopping VM ");
                    self.set_running(false);
                    return;
                }
                1 => {
                    let top_1 = self.stack.pop();
                    let top_2 = self.stack.pop();
                    println!("[ ADD ] : {} {} ", top_1, top_2);
                    self.stack.push(top_1 + top_2);
                }
                2 => {
                    let top_1 = self.stack.pop();
                    let top_2 = self.stack.pop();
                    println!("[ SUB ] : {} {} ", top_1, top_2);
                    self.stack.push(top_1 - top_2);
                }
                3 => {
                    let top_1 = self.stack.pop();
                    let top_2 = self.stack.pop();
                    println!("[ MULT ] : {} {} ", top_1, top_2);
                    self.stack.push(top_1 * top_2);
                }
                4 => {
                    let top_1 = self.stack.pop();
                    let top_2 = self.stack.pop();
    
                    println!("[ DIV ] : {} {} ", top_1, top_2);
                    self.stack.push(top_1 / top_2);
                }
                _ => {
                    panic!("[ exec ] : Undefined instruction ");
                }
            }
    
            println!(" TOS now : {}", self.stack.peek());
        }

        fn decode(&mut self) {
            let word = self.program_memory[self.pc];
            self.data = VM::get_data(word);
            self.type_info = VM::get_type(word);
        }

        fn fetch(&mut self) {
            if self.pc < self.program_memory.len() {
                 self.pc += 1;
             } else {
                 panic!("Incomplete code execution, memory boundary reached   without reading HALT instruction");
             }
         }

         fn is_running(&self) -> bool { self.running }
         fn set_running(&mut self, state: bool) { self.running = state }

         fn current_data(&self) -> i32 { self.data }
         fn current_instruction_type(&self) -> u8 { self.type_info }

         pub fn get_type(instruction: u32) -> u8 {
            ((instruction & 0xC0000000_u32) >> 30) as u8//2 msb
        }
        pub fn get_data(instruction: u32) -> i32 {
            (instruction & 0xffffffff) as i32
        }
    }
    
}

#[cfg(test)]
mod test_vm {
    use super::*;

    #[test]
    fn test_get_type() {
        assert_eq!(0, vm_mod::VM::get_type(0x0));
        assert_eq!(vm_mod::CODE_TYPE_INSTRUCTION, vm_mod::VM::get_type(1073741825));
        assert_eq!(vm_mod::CODE_TYPE_DATA_POSITIVE, vm_mod::VM::get_type(22));
        let neg = -100;
        assert_eq!(vm_mod::CODE_TYPE_DATA_NEGATIVE, vm_mod::VM::get_type(neg as u32));
    }

    #[test]
    fn test_get_data() {
        assert_eq!(0, vm_mod::VM::get_data(0));
        assert_eq!(1, vm_mod::VM::get_data(1));
        let num = -1;
        assert_eq!(-1, vm_mod::VM::get_data(num as u32));
    }
}