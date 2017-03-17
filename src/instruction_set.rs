use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum RegisterSize {
    Bit32,
    Bit64,
    Segment,
}

#[derive(Debug)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSP,
    RBP,
    RSI,
    RDI,

    RIP,

    EAX,
    EBX,
    ECX,
    EDX,
    ESP,
    EBP,
    ESI,
    EDI,

    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
}

#[derive(Debug)]
pub enum ArgumentSize {
    Bit64,
    Bit32,
    Bit16,
    Bit8,
}

pub fn get_register_size(reg: &Register) -> ArgumentSize {
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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = format!("{:?}", self).to_lowercase();
        write!(f, "%{}", rep)
    }
}


#[derive(Debug)]
pub enum InstructionArgument {
    Immediate { immediate: i64 },
    Register { register: Register },
    EffectiveAddress {
        register: Register,
        displacement: i32,
    },
}

#[derive(Debug)]
pub struct InstructionArguments {
    pub first_argument: InstructionArgument,
    pub second_argument: Option<InstructionArgument>,
    pub opcode: Option<u8>,
    explicit_size: Option<ArgumentSize>,
}

impl InstructionArguments {
    pub fn new(argument: InstructionArgument) -> InstructionArguments {
        InstructionArguments {
            first_argument: argument,
            second_argument: None,
            opcode: None,
            explicit_size: None,
        }
    }

    fn second_argument(&mut self, second_argument: InstructionArgument) -> &mut InstructionArguments {
        self.second_argument = Some(second_argument);
        self
    }

    fn opcode(&mut self, opcode: u8) -> &mut InstructionArguments {
        self.opcode = Some(opcode);
        self
    }

    fn explicit_size(&mut self, explicit_size: ArgumentSize) -> &mut InstructionArguments {
        self.explicit_size = Some(explicit_size);
        self
    }

    pub fn new_one_argument(argument: InstructionArgument) -> InstructionArguments {
        InstructionArguments {
            first_argument: argument,
            second_argument: None,
            opcode: None,
            explicit_size: None,
        }
    }

    pub fn new_one_argument_opcode(argument: InstructionArgument,
                                   opcode: u8)
                                   -> InstructionArguments {
        InstructionArguments {
            first_argument: argument,
            second_argument: None,
            opcode: Some(opcode),
            explicit_size: None,
        }
    }

    pub fn new_two_arguments(first_argument: InstructionArgument,
                             second_argument: InstructionArgument)
                             -> InstructionArguments {
        InstructionArguments {
            first_argument: first_argument,
            second_argument: Some(second_argument),
            opcode: None,
            explicit_size: None,
        }
    }

    pub fn new_two_arguments_opcode(first_argument: InstructionArgument,
                                    second_argument: InstructionArgument,
                                    opcode: u8)
                                    -> InstructionArguments {
        InstructionArguments {
            first_argument: first_argument,
            second_argument: Some(second_argument),
            opcode: Some(opcode),
            explicit_size: None,
        }
    }

    pub fn assert_one_argument(&self) {
        match self.second_argument {
            Some(_) => panic!("Instruction accepts only one argument"),
            None => (),
        }
    }

    pub fn assert_two_arguments(&self) {
        match self.second_argument {
            Some(_) => (),
            None => panic!("Instruction requires two arguments"),
        }
    }

    pub fn size(&self) -> ArgumentSize {
        match self.second_argument {
            Some(_) => {
                panic!("argument size for two arguments not implemented");
            }
            None => {
                match self.first_argument {
                    InstructionArgument::Register { ref register } => get_register_size(register),
                    InstructionArgument::Immediate { .. } => ArgumentSize::Bit64,
                    InstructionArgument::EffectiveAddress { .. } => ArgumentSize::Bit64,
                }
            }
        }
    }
}

impl fmt::Display for InstructionArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.second_argument {
            Some(ref second_argument) => write!(f, "{},{}", self.first_argument, second_argument),
            None => write!(f, "{}", self.first_argument),
        }
    }
}

impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstructionArgument::Register { ref register } => write!(f, "{}", register),
            InstructionArgument::Immediate { immediate } => write!(f, "$0x{:x}", immediate),
            InstructionArgument::EffectiveAddress { ref register, displacement } => {
                if displacement < 0 {
                    write!(f, "-{:#x}({})", displacement.abs(), register)
                } else if displacement > 0 {
                    write!(f, "{:#x}({})", displacement, register)
                } else {
                    write!(f, "({})", register)
                }
            }
        }
    }
}
