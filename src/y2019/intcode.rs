use num_enum::TryFromPrimitive;

pub type Value = i64;
pub type Program = Vec<Value>;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i64)]
enum OpCode {
    Add = 1,
    Mult,
    Read,
    Write,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelBase,

    Halt = 99,
}

pub struct IntCode {
    memory: Program,
    inst_ptr: usize,
    inputs: Vec<Value>,
    outputs: Vec<Value>,
    relative_base: Value,
}

impl IntCode {
    pub fn parse(program: &str) -> Program {
        let memory: Program = program
            .trim()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        return memory;
    }

    pub fn load(program: Program) -> IntCode {
        IntCode {
            memory: program,
            inst_ptr: 0,
            inputs: vec![],
            outputs: vec![],
            relative_base: 0,
        }
    }

    pub fn set_inputs(&mut self, inputs: Vec<Value>) {
        self.inputs = inputs;
        self.inputs.reverse();
    }

    pub fn get_outputs(&mut self) -> &Vec<Value> {
        return &self.outputs;
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    fn opcode(&self) -> OpCode {
        let ptr = self.inst_ptr;
        let instruction = self.memory[ptr];

        match (instruction % 100).try_into() {
            Ok(opcode) => opcode,
            _ => panic!("Unknown opcode"),
        }
    }

    pub fn halted(&self) -> bool {
        return self.opcode() == OpCode::Halt;
    }

    fn _get(&mut self, index: usize) -> Value {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        return self.memory[index];
    }
    fn _set(&mut self, index: usize, value: Value) {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }

    pub fn get(&mut self, index: usize, mode: Value) -> Value {
        let index = match mode {
            0 => self._get(index) as usize,
            1 => index,
            2 => (self.relative_base + self._get(index)) as usize,
            _ => panic!("Unknown mode"),
        };
        return self._get(index);
    }

    pub fn set(&mut self, index: usize, mode: Value, value: Value) {
        let index = match mode {
            0 => self._get(index) as usize,
            1 => index,
            2 => (self.relative_base + self._get(index)) as usize,
            _ => panic!("Unknown mode"),
        };
        self._set(index as usize, value);
    }

    pub fn step(&mut self) -> bool {
        let ptr = self.inst_ptr;
        let instruction = self.get(ptr, 1);
        let opcode = (instruction % 100).try_into();
        let mode1 = (instruction / 100) % 10;
        let mode2 = (instruction / 1000) % 10;
        let mode3 = (instruction / 10000) % 10;
        match opcode {
            Ok(OpCode::Add) => {
                let value = self.get(ptr + 1, mode1) + self.get(ptr + 2, mode2);
                self.set(ptr + 3, mode3, value);
                self.inst_ptr += 4;
            }
            Ok(OpCode::Mult) => {
                let value = self.get(ptr + 1, mode1) * self.get(ptr + 2, mode2);
                self.set(ptr + 3, mode3, value);
                self.inst_ptr += 4;
            }
            Ok(OpCode::Read) => {
                if let Some(value) = self.inputs.pop() {
                    self.set(ptr + 1, mode1, value);
                    self.inst_ptr += 2;
                } else {
                    return false;
                }
            }
            Ok(OpCode::Write) => {
                let value = self.get(ptr + 1, mode1);
                self.outputs.push(value);
                self.inst_ptr += 2;
            }
            Ok(OpCode::JumpIfTrue) => {
                let cond = self.get(ptr + 1, mode1);
                let value = self.get(ptr + 2, mode2);
                if cond != 0 {
                    self.inst_ptr = value as usize;
                } else {
                    self.inst_ptr += 3;
                }
            }
            Ok(OpCode::JumpIfFalse) => {
                let cond = self.get(ptr + 1, mode1);
                let value = self.get(ptr + 2, mode2);
                if cond == 0 {
                    self.inst_ptr = value as usize;
                } else {
                    self.inst_ptr += 3;
                }
            }
            Ok(OpCode::LessThan) => {
                let cond = self.get(ptr + 1, mode1) < self.get(ptr + 2, mode2);
                self.set(ptr + 3, mode3, cond as Value);
                self.inst_ptr += 4;
            }
            Ok(OpCode::Equals) => {
                let cond = self.get(ptr + 1, mode1) == self.get(ptr + 2, mode2);
                self.set(ptr + 3, mode3, cond as Value);
                self.inst_ptr += 4;
            }
            Ok(OpCode::AdjustRelBase) => {
                let value = self.get(ptr + 1, mode1);
                self.relative_base += value;
                self.inst_ptr += 2;
            }
            Ok(OpCode::Halt) => return false,
            Err(_) => panic!("Unknown OpCode"),
        }

        return true;
    }

    pub fn run(&mut self) -> Value {
        while self.step() {}
        return self.memory[0];
    }
}
