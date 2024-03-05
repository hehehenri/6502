use memory::{Bus, Memory};

mod memory;

pub struct CPU {
    memory: Box<dyn Bus>,

    accumulator: u8,
    x: u8,
    y: u8,
    program_counter: u16,
    stack_pointer: u8,
    status: u8,
}

enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Break,
    Overflow,
    Negative,
}

impl Flag {
    fn position(&self) -> u8 {
        match self {
            Self::Carry => 0,
            Self::Zero => 1,
            Self::InterruptDisable => 2,
            Self::Decimal => 3,
            Self::Break => 4,
            Self::Overflow => 6,
            Self::Negative => 7,
        }
    }
}

impl CPU {
    fn set_status(&mut self, flag: Flag, value: bool) {
        let value = match value {
            true => 1,
            false => 0,
        };

        self.stack_pointer = self.status | (value << flag.position());
    }

    pub fn new() -> Self {
        let memory = Memory::empty();

        Self {
            memory: Box::new(memory),
            program_counter: 0xFFFC,
            stack_pointer: 0x00FF,
            accumulator: 0,
            x: 0,
            y: 0,
            status: 0,
        }
    }
}
