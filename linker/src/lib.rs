mod exec;

use cfg::{
    opcode::{arity, is_push, Opcode},
    Block,
};
use proptest::{
    prelude::any,
    test_runner::{Config, TestCaseError, TestError, TestRunner},
};

#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    /// block not jumping
    No,
    /// variant of PUSH(N) for every case
    Concrete(Vec<u8>),
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
        let jump = block.instructions.last().unwrap().opcode == Opcode::JUMP;
        let jumpi = block.instructions.last().unwrap().opcode == Opcode::JUMPI;
        for instr in block.instructions.iter().rev() {
            loc_arity += arity(&instr.opcode);
            if !(jumpi || jump) {
                return JType::No;
            } else if (loc_arity >= 1 && jumpi) || jump {
                if is_push(&instr.opcode) {
                    return JType::Concrete(instr.pushes.clone());
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
    use cfg::{op, opcode::Opcode, Block, Operation};

    #[test]
    fn cfg_to_jcfg() {
        let cfg = vec![
            Block {
                start: 0,
                end: 15,
                #[rustfmt::skip]
                instructions: vec![
                    op!(PUSH0), op!(CALLDATALOAD), op!(PUSH1), op!(SHR),
                    op!(DUP1), op!(PUSH4), op!(EQ), op!(PUSH2, [0, 16]), op!(JUMPI),
                ],
            },
            Block {
                start: 16,
                end: 26,
                #[rustfmt::skip]
                instructions: vec![
                    op!(DUP1), op!(PUSH4), op!(EQ), op!(PUSH2, [0, 27]), op!(JUMPI),
                ],
            },
            Block {
                start: 27,
                end: 37,
                #[rustfmt::skip]
                instructions: vec![
                    op!(DUP1), op!(PUSH4), op!(EQ), op!(PUSH2, [0, 38]), op!(JUMPI),
                ],
            },
            Block {
                start: 38,
                end: 40,
                #[rustfmt::skip]
                instructions: vec![
                    op!(PUSH0), op!(PUSH0), op!(REVERT)
                ],
            },
        ];

        let jcfg = BytecodeRunner::glue(cfg);

        // all jumps should be concrete here!
        assert!(!jcfg[..jcfg.len() - 1]
            .iter()
            .any(|jblock| !matches!(jblock.jump, JType::Concrete(_))));

        let jump_pcs: Vec<_> = jcfg
            .iter()
            .filter(|jblock| matches!(jblock.jump, JType::Concrete(_)))
            .map(|jblock| {
                if let JType::Concrete(mut pc) = jblock.jump.clone() {
                    let mut new = if pc.len() <= 8 {
                        vec![0; 8 - pc.len()]
                    } else {
                        unimplemented!();
                    };
                    new.append(&mut pc);
                    let arr: [u8; 8] = new.try_into().unwrap();
                    usize::from_be_bytes(arr)
                } else {
                    unreachable!()
                }
            })
            .collect();

        assert_eq!(jump_pcs, vec![16, 27, 38]);
    }
}
