pub mod opcode;

use self::opcode::{byte_as_opcode, is_push, is_terminating, push_size, Opcode};

pub struct Bytecode(Vec<u8>);

impl From<Vec<u8>> for Bytecode {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&str> for Bytecode {
    fn from(value: &str) -> Self {
        if value.len() % 2 != 0 {
            panic!("non-even nibbles!");
        }

        let value = value.trim_start_matches("0x");

        // chunks() is not available in std!
        let mut bytes = Vec::new();
        let mut i = 0usize;
        while i < value.len() {
            let byte = &value[i..(i + 2)];
            bytes.push(byte.parse().expect("invalid char!"));
            i += 2;
        }

        bytes.into()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub start: usize,
    pub end: usize,
    pub instructions: Vec<Opcode>,
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Cfg {
//     blocks: Vec<Block>,
// }

impl Bytecode {
    pub fn as_mnemonics(&self) -> Vec<Opcode> {
        let bytes = &self.0;
        let mut instructions = Vec::new();
        let mut i = 0usize;
        while i < bytes.len() {
            let opcode = if let Some(opcode) = byte_as_opcode(bytes.get(i).unwrap()) {
                if let Some(size) = push_size(&opcode) {
                    i += size;
                }

                opcode
            } else {
                Opcode::INVALID
            };
            instructions.push(opcode);
            i += 1;
        }

        instructions
    }

    pub fn as_sparse_cfg(&self) -> Vec<Block> {
        let mnemonics = self.as_mnemonics();
        Self::mnemonics_to_cfg(mnemonics)
    }

    pub fn mnemonics_to_cfg(mnemonics: Vec<Opcode>) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut pc = 0usize;
        let mut block = Self::make_block(pc);
        let mut instructions = Vec::new();
        let mut mnemonics = mnemonics.into_iter();
        let last_instruction = mnemonics.next_back();
        for opcode in mnemonics {
            instructions.push(opcode);
            if is_terminating(&opcode) {
                block.end = pc;
                block.instructions = instructions;
                blocks.push(block);

                pc += 1;

                block = Self::make_block(pc);
                instructions = Vec::new();
            } else {
                pc += 1;
            }

            if is_push(&opcode) {
                pc += push_size(&opcode).unwrap();
            }
        }

        block.end = pc;
        if let Some(instruction) = last_instruction {
            instructions.push(instruction);
        }
        block.instructions = instructions;
        blocks.push(block);

        blocks
    }

    fn make_block(pc: usize) -> Block {
        Block {
            start: pc,
            end: 0,
            instructions: Vec::new(),
        }
    }
}

// impl Cfg {
//     type Error = String;

//     fn fac(n: usize) -> usize {
//         if n == 0 || n == 1 {
//             return n;
//         }
//         let mut ret = 0;
//         for i in 2..n {
//             ret *= i;
//         }
//         return ret;
//     }

//     /// return the max permutations going from 2 to num
//     fn max_perm(num: usize) -> Result<Vec<usize>, Self::Error> {
//         if num < 2 {
//             panic!("too low!");
//         }

//         (2..num).into_iter().map(|n| Self::fac(n)).collect()
//     }

//     fn perms_of(&self, n: usize) -> Vec<Block> {
//         Vec::new()
//     }

//     pub fn vec_permutations(&self, num: usize) -> Vec<Block> {
//         (2..num)
//             .into_iter()
//             .map(|n| self.perms_of(n))
//             .flatten()
//             .collect()
//     }

//     // pub fn permutations(&self, num: usize) -> Result<Box<Iterator<Item = Block>, Self::Error>> {
//     //     if num < 2 {
//     //         Err(String::from("too low!"))
//     //     } else {
//     //         let mut iter = Box::new(std::iter::empty()); // TODO should be an iterator
//     //         let iter = if num > 2 {
//     //             iter.chain(self.permutations(num - 1)?.flatten().into())
//     //         } else {
//     //             iter
//     //         };

//     //         Ok(iter)
//     //     }
//     // }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mnemonics() {
        let code = vec![0x5F, 0x5F, 0x01];
        let bytecode = Bytecode::from(code);
        let mnemonics = bytecode.as_mnemonics();
        assert_eq!(mnemonics, vec![Opcode::PUSH0, Opcode::PUSH0, Opcode::ADD]);
    }

    #[test]
    fn flat_cfg() {
        #[rustfmt::skip]
        let mnemonics = vec![
            Opcode::PUSH0, Opcode::CALLDATALOAD, Opcode::PUSH1, Opcode::SHL,
            Opcode::DUP1, Opcode::PUSH4, Opcode::EQ, Opcode::PUSH2, Opcode::JUMPI,
            Opcode::DUP1, Opcode::PUSH4, Opcode::EQ, Opcode::PUSH2, Opcode::JUMPI,
            Opcode::DUP1, Opcode::PUSH4, Opcode::EQ, Opcode::PUSH2, Opcode::JUMPI,
            Opcode::PUSH0, Opcode::PUSH0, Opcode::REVERT
        ];

        let cfg = vec![
            Block {
                start: 0,
                end: 15,
                #[rustfmt::skip]
                instructions: vec![
                    Opcode::PUSH0, Opcode::CALLDATALOAD, Opcode::PUSH1, Opcode::SHL,
                    Opcode::DUP1, Opcode::PUSH4, Opcode::EQ, Opcode::PUSH2, Opcode::JUMPI,
                ],
            },
            Block {
                start: 16,
                end: 26,
                #[rustfmt::skip]
                instructions: vec![
                    Opcode::DUP1, Opcode::PUSH4, Opcode::EQ, Opcode::PUSH2, Opcode::JUMPI,
                ],
            },
            Block {
                start: 27,
                end: 37,
                #[rustfmt::skip]
                instructions: vec![
                    Opcode::DUP1, Opcode::PUSH4, Opcode::EQ, Opcode::PUSH2, Opcode::JUMPI,
                ],
            },
            Block {
                start: 38,
                end: 40,
                #[rustfmt::skip]
                instructions: vec![
                    Opcode::PUSH0, Opcode::PUSH0, Opcode::REVERT
                ],
            },
        ];

        assert_eq!(Bytecode::mnemonics_to_cfg(mnemonics), cfg);
    }
}
