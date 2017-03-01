use instruction_set::{InstructionArgument, Register};

use cpu::CPU;

pub enum ArgumentSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}


impl CPU {
    pub fn first_argument_size(&self, arg: &InstructionArgument) -> ArgumentSize {
        match *arg {
            InstructionArgument::OneRegister { ref register, .. } => get_register_size(register),
            InstructionArgument::TwoRegister { ref register1,
                                               ref register2,
                                               reverse_direction,
                                               .. } => {
                if reverse_direction {
                    get_register_size(register2)
                } else {
                    get_register_size(register1)
                }
            }
            InstructionArgument::Immediate8 { .. } => ArgumentSize::Bit8,
            InstructionArgument::Immediate32 { .. } => ArgumentSize::Bit32,
            InstructionArgument::Immediate8BitRegister { .. } => ArgumentSize::Bit8,
            InstructionArgument::Immediate32BitRegister { .. } => ArgumentSize::Bit32,
        }
    }

    pub fn second_argument_size(&self, arg: &InstructionArgument) -> ArgumentSize {
        match *arg {
            InstructionArgument::OneRegister { .. } => panic!("Only one argument available"),
            InstructionArgument::TwoRegister { ref register1,
                                               ref register2,
                                               reverse_direction,
                                               .. } => {
                if reverse_direction {
                    get_register_size(register1)
                } else {
                    get_register_size(register2)
                }
            }
            InstructionArgument::Immediate8 { .. } => panic!("Only one argument available"),
            InstructionArgument::Immediate32 { .. } => panic!("Only one argument available"),
            InstructionArgument::Immediate8BitRegister { ref register, .. } => {
                get_register_size(register)
            }
            InstructionArgument::Immediate32BitRegister { ref register, .. } => {
                get_register_size(register)
            }
        }
    }

    pub fn first_argument_i8(&self, arg: InstructionArgument) -> i8 {
        panic!("Not implemented");
    }

    pub fn first_argument_i16(&self, arg: InstructionArgument) -> i16 {
        panic!("Not implemented");
    }

    pub fn first_argument_i32(&self, arg: InstructionArgument) -> i32 {
        match arg {
            InstructionArgument::OneRegister { register, .. } => {
                self.get_register_value_i32(register)
            },
            InstructionArgument::TwoRegister { register1, .. } => {
                self.get_register_value_i32(register1)
            },
            InstructionArgument::Immediate8 { immediate, .. } => immediate as i32,
            InstructionArgument::Immediate32 { immediate, .. } => immediate as i32,
            InstructionArgument::Immediate8BitRegister { immediate, .. } => immediate as i32,
            InstructionArgument::Immediate32BitRegister  { immediate, .. } => immediate as i32,
        }
    }

    pub fn first_argument_i64(&self, arg: InstructionArgument) -> i64 {
        match arg {
            InstructionArgument::OneRegister { register, .. } => {
                self.get_register_value_i64(register)
            },
            InstructionArgument::TwoRegister { register1, .. } => {
                self.get_register_value_i64(register1)
            },
            InstructionArgument::Immediate8 { immediate, .. } => immediate as i64,
            InstructionArgument::Immediate32 { immediate, .. } => immediate as i64,
            InstructionArgument::Immediate8BitRegister { immediate, .. } => immediate as i64,
            InstructionArgument::Immediate32BitRegister  { immediate, .. } => immediate as i64,
        }
    }


    pub fn second_argument_i8(&self, arg: InstructionArgument) -> i8 {
        panic!("Not implemented");
    }

    pub fn second_argument_i16(&self, arg: InstructionArgument) -> i16 {
        panic!("Not implemented");
    }

    pub fn second_argument_i32(&self, arg: InstructionArgument) -> i32 {
        panic!("Not implemented");
    }

    pub fn second_argument_i64(&self, arg: InstructionArgument) -> i64 {
        panic!("Not implemented");
    }


    // register operations
    fn get_register_value_i32(&self, register: Register) -> i32 {
        panic!("Not implemented");
    }

    fn get_register_value_i64(&self, register: Register) -> i64 {
        panic!("Not implemented");
    }

    fn set_register_value_i32(&self, register: Register, value: i32) {
        panic!("Not implemented");
    }

    fn set_register_value_i64(&self, register: Register, value: i32) {
        panic!("Not implemented");
    }


    // stack operations
    pub fn stack_push(&self, data: Vec<u8>) {
        panic!("Not implemented");
    }

}

pub fn convert_i8_to_u8vec(value: i32) -> Vec<u8> {
    vec![]
}

pub fn convert_i16_to_u8vec(value: i32) -> Vec<u8> {
    vec![]
}

pub fn convert_i32_to_u8vec(value: i32) -> Vec<u8> {
    vec![]
}

pub fn convert_i64_to_u8vec(value: i64) -> Vec<u8> {
    vec![]
}


fn get_register_size(reg: &Register) -> ArgumentSize {
    match *reg {
        Register::RAX | Register::RBX | Register::RCX | Register::RDX | Register::RSP |
        Register::RBP | Register::RSI | Register::RDI | Register::RIP => ArgumentSize::Bit64,
        Register::EAX | Register::EBX | Register::ECX | Register::EDX | Register::ESP |
        Register::EBP | Register::ESI | Register::EDI => ArgumentSize::Bit32,
        Register::ES | Register::CS | Register::SS | Register::DS | Register::FS | Register::GS => {
            ArgumentSize::Bit16
        }
    }
}
