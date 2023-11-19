mod exec;

use cfg::{
    opcode::{arity, is_push},
    Block,
};
use proptest::{
    prelude::any,
    test_runner::{Config, TestCaseError, TestError, TestRunner},
};

#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    /// variant of PUSH(N) for every case
    Concrete(usize),
    /// value cannot be known at compile time
    Symbolic,
    /// value is already on the stack
    Unknown,
}

#[derive(Debug, Clone)]
pub struct JBlock {
    /// the location it's jumping to
    jump: JType,
    block: Block,
}

#[derive(Debug, Clone)]
pub struct BytecodeRunner {
    test_runner: TestRunner,
    jcfg: Vec<JBlock>,
    covered: usize,
}

// TODO use ranges of pcs for easier merging
#[derive(Debug, Clone)]
pub struct CoverageReport {
    pcs: Vec<usize>,
}

impl BytecodeRunner {
    // TODO constructor data ?
    fn new(cfg: Vec<Block>) -> Self {
        let config: Config = Config {
            max_shrink_iters: 100,
            ..Default::default()
        };
        let test_runner = TestRunner::new(config);
        let jcfg = Self::glue(cfg);

        Self {
            test_runner,
            jcfg,
            covered: 0,
        }
    }

    /// turn a cfg into a jcfg (jumping cfg)
    fn glue(cfg: Vec<Block>) -> Vec<JBlock> {
        cfg.into_iter()
            .map(|block| JBlock {
                jump: Self::get_jump(&block),
                block,
            })
            .collect()
    }

    pub fn max_instr(&self) -> usize {
        self.jcfg
            .iter()
            .map(|cfg| cfg.block.instructions.len())
            .sum()
    }

    fn get_jump(block: &Block) -> JType {
        let mut loc_arity = 0;
        for instr in block.instructions.iter().rev() {
            loc_arity += arity(instr);
            if loc_arity >= 1 {
                if is_push(instr) {
                    return JType::Concrete(0); // TODO we should get the location pc here
                } else {
                    return JType::Symbolic;
                }
            }
        }

        JType::Unknown
    }

    fn permutations(&self, num: usize) {
        if num == 0 {
            panic!("smallish!");
        }

        todo!()
    }

    fn single_fuzz(data: Vec<u8>) -> Result<Option<CoverageReport>, TestCaseError> {
        // if we hit more coverage, minimize the input and store it

        Ok(Some(CoverageReport { pcs: Vec::new() }))
    }

    fn execute(&mut self) -> Result<(), TestError<Vec<u8>>> {
        let strategy = proptest::collection::vec(any::<u8>(), 0..1000);
        self.test_runner.run(&strategy, |data| {
            let report = BytecodeRunner::single_fuzz(data)?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{BytecodeRunner, JType};
    use cfg::{opcode::Opcode, Block};

    #[test]
    fn cfg_to_jcfg() {
        let cfg = vec![
            Block {
                start: 0,
                end: 15,
                #[rustfmt::skip]
                instructions: vec![
                    Opcode::PUSH0, Opcode::CALLDATALOAD, Opcode::PUSH1, Opcode::SHR,
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

        let jcfg = BytecodeRunner::glue(cfg);

        // all jumps should be concrete here!
        assert!(!jcfg
            .iter()
            .any(|jblock| !matches!(jblock.jump, JType::Concrete(_))));

        let jump_pcs: Vec<_> = jcfg
            .iter()
            .filter(|jblock| matches!(jblock.jump, JType::Concrete(_)))
            .map(|jblock| {
                if let JType::Concrete(pc) = jblock.jump {
                    pc
                } else {
                    unreachable!()
                }
            })
            .collect();

        assert_eq!(jump_pcs, vec![16, 27, 38]);
    }
}
