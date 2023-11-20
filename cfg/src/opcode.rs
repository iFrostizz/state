#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Opcode {
    #[default]
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    KECCAK256,
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    BASEFEE,
    BLOBHASH,
    BLOBBASEFEE,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    TLOAD,
    TSTORE,
    MCOPY,
    PUSH0,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    LOG0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

pub fn byte_as_opcode(byte: &u8) -> Option<Opcode> {
    match byte {
        0x00 => Some(Opcode::STOP),
        0x01 => Some(Opcode::ADD),
        0x02 => Some(Opcode::MUL),
        0x03 => Some(Opcode::SUB),
        0x04 => Some(Opcode::DIV),
        0x05 => Some(Opcode::SDIV),
        0x06 => Some(Opcode::MOD),
        0x07 => Some(Opcode::SMOD),
        0x08 => Some(Opcode::ADDMOD),
        0x09 => Some(Opcode::MULMOD),
        0x0A => Some(Opcode::EXP),
        0x0B => Some(Opcode::SIGNEXTEND),
        0x10 => Some(Opcode::LT),
        0x11 => Some(Opcode::GT),
        0x12 => Some(Opcode::SLT),
        0x13 => Some(Opcode::SGT),
        0x14 => Some(Opcode::EQ),
        0x15 => Some(Opcode::ISZERO),
        0x16 => Some(Opcode::AND),
        0x17 => Some(Opcode::OR),
        0x18 => Some(Opcode::XOR),
        0x19 => Some(Opcode::NOT),
        0x1A => Some(Opcode::BYTE),
        0x1B => Some(Opcode::SHL),
        0x1C => Some(Opcode::SHR),
        0x1D => Some(Opcode::SAR),
        0x20 => Some(Opcode::KECCAK256),
        0x30 => Some(Opcode::ADDRESS),
        0x31 => Some(Opcode::BALANCE),
        0x32 => Some(Opcode::ORIGIN),
        0x33 => Some(Opcode::CALLER),
        0x34 => Some(Opcode::CALLVALUE),
        0x35 => Some(Opcode::CALLDATALOAD),
        0x36 => Some(Opcode::CALLDATASIZE),
        0x37 => Some(Opcode::CALLDATACOPY),
        0x38 => Some(Opcode::CODESIZE),
        0x39 => Some(Opcode::CODECOPY),
        0x3A => Some(Opcode::GASPRICE),
        0x3B => Some(Opcode::EXTCODESIZE),
        0x3C => Some(Opcode::EXTCODECOPY),
        0x3D => Some(Opcode::RETURNDATASIZE),
        0x3E => Some(Opcode::RETURNDATACOPY),
        0x3F => Some(Opcode::EXTCODEHASH),
        0x40 => Some(Opcode::BLOCKHASH),
        0x41 => Some(Opcode::COINBASE),
        0x42 => Some(Opcode::TIMESTAMP),
        0x43 => Some(Opcode::NUMBER),
        0x44 => Some(Opcode::DIFFICULTY),
        0x45 => Some(Opcode::GASLIMIT),
        0x46 => Some(Opcode::CHAINID),
        0x47 => Some(Opcode::SELFBALANCE),
        0x48 => Some(Opcode::BASEFEE),
        0x49 => Some(Opcode::BLOBHASH),
        0x4A => Some(Opcode::BLOBBASEFEE),
        0x50 => Some(Opcode::POP),
        0x51 => Some(Opcode::MLOAD),
        0x52 => Some(Opcode::MSTORE),
        0x53 => Some(Opcode::MSTORE8),
        0x54 => Some(Opcode::SLOAD),
        0x55 => Some(Opcode::SSTORE),
        0x56 => Some(Opcode::JUMP),
        0x57 => Some(Opcode::JUMPI),
        0x58 => Some(Opcode::PC),
        0x59 => Some(Opcode::MSIZE),
        0x5A => Some(Opcode::GAS),
        0x5B => Some(Opcode::JUMPDEST),
        0x5C => Some(Opcode::TLOAD),
        0x5D => Some(Opcode::TSTORE),
        0x5E => Some(Opcode::MCOPY),
        0x5F => Some(Opcode::PUSH0),
        0x60 => Some(Opcode::PUSH1),
        0x61 => Some(Opcode::PUSH2),
        0x62 => Some(Opcode::PUSH3),
        0x63 => Some(Opcode::PUSH4),
        0x64 => Some(Opcode::PUSH5),
        0x65 => Some(Opcode::PUSH6),
        0x66 => Some(Opcode::PUSH7),
        0x67 => Some(Opcode::PUSH8),
        0x68 => Some(Opcode::PUSH9),
        0x69 => Some(Opcode::PUSH10),
        0x6A => Some(Opcode::PUSH11),
        0x6B => Some(Opcode::PUSH12),
        0x6C => Some(Opcode::PUSH13),
        0x6D => Some(Opcode::PUSH14),
        0x6E => Some(Opcode::PUSH15),
        0x6F => Some(Opcode::PUSH16),
        0x70 => Some(Opcode::PUSH17),
        0x71 => Some(Opcode::PUSH18),
        0x72 => Some(Opcode::PUSH19),
        0x73 => Some(Opcode::PUSH20),
        0x74 => Some(Opcode::PUSH21),
        0x75 => Some(Opcode::PUSH22),
        0x76 => Some(Opcode::PUSH23),
        0x77 => Some(Opcode::PUSH24),
        0x78 => Some(Opcode::PUSH25),
        0x79 => Some(Opcode::PUSH26),
        0x7A => Some(Opcode::PUSH27),
        0x7B => Some(Opcode::PUSH28),
        0x7C => Some(Opcode::PUSH29),
        0x7D => Some(Opcode::PUSH30),
        0x7E => Some(Opcode::PUSH31),
        0x7F => Some(Opcode::PUSH32),
        0x80 => Some(Opcode::DUP1),
        0x81 => Some(Opcode::DUP2),
        0x82 => Some(Opcode::DUP3),
        0x83 => Some(Opcode::DUP4),
        0x84 => Some(Opcode::DUP5),
        0x85 => Some(Opcode::DUP6),
        0x86 => Some(Opcode::DUP7),
        0x87 => Some(Opcode::DUP8),
        0x88 => Some(Opcode::DUP9),
        0x89 => Some(Opcode::DUP10),
        0x8A => Some(Opcode::DUP11),
        0x8B => Some(Opcode::DUP12),
        0x8C => Some(Opcode::DUP13),
        0x8D => Some(Opcode::DUP14),
        0x8E => Some(Opcode::DUP15),
        0x8F => Some(Opcode::DUP16),
        0x90 => Some(Opcode::SWAP1),
        0x91 => Some(Opcode::SWAP2),
        0x92 => Some(Opcode::SWAP3),
        0x93 => Some(Opcode::SWAP4),
        0x94 => Some(Opcode::SWAP5),
        0x95 => Some(Opcode::SWAP6),
        0x96 => Some(Opcode::SWAP7),
        0x97 => Some(Opcode::SWAP8),
        0x98 => Some(Opcode::SWAP9),
        0x99 => Some(Opcode::SWAP10),
        0x9A => Some(Opcode::SWAP11),
        0x9B => Some(Opcode::SWAP12),
        0x9C => Some(Opcode::SWAP13),
        0x9D => Some(Opcode::SWAP14),
        0x9E => Some(Opcode::SWAP15),
        0x9F => Some(Opcode::SWAP16),
        0xA0 => Some(Opcode::LOG0),
        0xA1 => Some(Opcode::LOG1),
        0xA2 => Some(Opcode::LOG2),
        0xA3 => Some(Opcode::LOG3),
        0xA4 => Some(Opcode::LOG4),
        0xF0 => Some(Opcode::CREATE),
        0xF1 => Some(Opcode::CALL),
        0xF2 => Some(Opcode::CALLCODE),
        0xF3 => Some(Opcode::RETURN),
        0xF4 => Some(Opcode::DELEGATECALL),
        0xF5 => Some(Opcode::CREATE2),
        0xFA => Some(Opcode::STATICCALL),
        0xFD => Some(Opcode::REVERT),
        0xFE => Some(Opcode::INVALID),
        0xFF => Some(Opcode::SELFDESTRUCT),
        _ => None,
    }
}

pub fn is_push(opcode: &Opcode) -> bool {
    matches!(
        opcode,
        Opcode::PUSH0
            | Opcode::PUSH1
            | Opcode::PUSH2
            | Opcode::PUSH3
            | Opcode::PUSH4
            | Opcode::PUSH5
            | Opcode::PUSH6
            | Opcode::PUSH7
            | Opcode::PUSH8
            | Opcode::PUSH9
            | Opcode::PUSH10
            | Opcode::PUSH11
            | Opcode::PUSH12
            | Opcode::PUSH13
            | Opcode::PUSH14
            | Opcode::PUSH15
            | Opcode::PUSH16
            | Opcode::PUSH17
            | Opcode::PUSH18
            | Opcode::PUSH19
            | Opcode::PUSH20
            | Opcode::PUSH21
            | Opcode::PUSH22
            | Opcode::PUSH23
            | Opcode::PUSH24
            | Opcode::PUSH25
            | Opcode::PUSH26
            | Opcode::PUSH27
            | Opcode::PUSH28
            | Opcode::PUSH29
            | Opcode::PUSH30
            | Opcode::PUSH31
            | Opcode::PUSH32
    )
}

pub fn arity(opcode: &Opcode) -> u8 {
    match opcode {
        Opcode::STOP
        | Opcode::CALLDATACOPY
        | Opcode::CODECOPY
        | Opcode::EXTCODECOPY
        | Opcode::RETURNDATACOPY
        | Opcode::POP
        | Opcode::MSTORE
        | Opcode::MSTORE8
        | Opcode::SSTORE
        | Opcode::JUMP
        | Opcode::JUMPI
        | Opcode::JUMPDEST
        | Opcode::TSTORE
        | Opcode::MCOPY
        | Opcode::SWAP1
        | Opcode::SWAP2
        | Opcode::SWAP3
        | Opcode::SWAP4
        | Opcode::SWAP5
        | Opcode::SWAP6
        | Opcode::SWAP7
        | Opcode::SWAP8
        | Opcode::SWAP9
        | Opcode::SWAP10
        | Opcode::SWAP11
        | Opcode::SWAP12
        | Opcode::SWAP13
        | Opcode::SWAP14
        | Opcode::SWAP15
        | Opcode::SWAP16
        | Opcode::LOG0
        | Opcode::LOG1
        | Opcode::LOG2
        | Opcode::LOG3
        | Opcode::LOG4
        | Opcode::RETURN
        | Opcode::REVERT
        | Opcode::INVALID
        | Opcode::SELFDESTRUCT => 0,
        Opcode::ADD
        | Opcode::MUL
        | Opcode::SUB
        | Opcode::DIV
        | Opcode::SDIV
        | Opcode::MOD
        | Opcode::SMOD
        | Opcode::ADDMOD
        | Opcode::MULMOD
        | Opcode::EXP
        | Opcode::SIGNEXTEND
        | Opcode::LT
        | Opcode::GT
        | Opcode::SLT
        | Opcode::SGT
        | Opcode::EQ
        | Opcode::ISZERO
        | Opcode::AND
        | Opcode::OR
        | Opcode::XOR
        | Opcode::NOT
        | Opcode::BYTE
        | Opcode::SHL
        | Opcode::SHR
        | Opcode::SAR
        | Opcode::KECCAK256
        | Opcode::ADDRESS
        | Opcode::BALANCE
        | Opcode::ORIGIN
        | Opcode::CALLER
        | Opcode::CALLVALUE
        | Opcode::CALLDATALOAD
        | Opcode::CALLDATASIZE
        | Opcode::CODESIZE
        | Opcode::GASPRICE
        | Opcode::EXTCODESIZE
        | Opcode::RETURNDATASIZE
        | Opcode::EXTCODEHASH
        | Opcode::BLOCKHASH
        | Opcode::COINBASE
        | Opcode::TIMESTAMP
        | Opcode::NUMBER
        | Opcode::DIFFICULTY
        | Opcode::GASLIMIT
        | Opcode::CHAINID
        | Opcode::SELFBALANCE
        | Opcode::BASEFEE
        | Opcode::BLOBHASH
        | Opcode::BLOBBASEFEE
        | Opcode::MLOAD
        | Opcode::SLOAD
        | Opcode::PC
        | Opcode::MSIZE
        | Opcode::GAS
        | Opcode::TLOAD
        | Opcode::PUSH0
        | Opcode::PUSH1
        | Opcode::PUSH2
        | Opcode::PUSH3
        | Opcode::PUSH4
        | Opcode::PUSH5
        | Opcode::PUSH6
        | Opcode::PUSH7
        | Opcode::PUSH8
        | Opcode::PUSH9
        | Opcode::PUSH10
        | Opcode::PUSH11
        | Opcode::PUSH12
        | Opcode::PUSH13
        | Opcode::PUSH14
        | Opcode::PUSH15
        | Opcode::PUSH16
        | Opcode::PUSH17
        | Opcode::PUSH18
        | Opcode::PUSH19
        | Opcode::PUSH20
        | Opcode::PUSH21
        | Opcode::PUSH22
        | Opcode::PUSH23
        | Opcode::PUSH24
        | Opcode::PUSH25
        | Opcode::PUSH26
        | Opcode::PUSH27
        | Opcode::PUSH28
        | Opcode::PUSH29
        | Opcode::PUSH30
        | Opcode::PUSH31
        | Opcode::PUSH32
        | Opcode::DUP1
        | Opcode::DUP2
        | Opcode::DUP3
        | Opcode::DUP4
        | Opcode::DUP5
        | Opcode::DUP6
        | Opcode::DUP7
        | Opcode::DUP8
        | Opcode::DUP9
        | Opcode::DUP10
        | Opcode::DUP11
        | Opcode::DUP12
        | Opcode::DUP13
        | Opcode::DUP14
        | Opcode::DUP15
        | Opcode::DUP16
        | Opcode::CREATE
        | Opcode::CALL
        | Opcode::CALLCODE
        | Opcode::DELEGATECALL
        | Opcode::CREATE2
        | Opcode::STATICCALL => 1,
    }
}

pub fn opcode_as_byte(opcode: &Opcode) -> u8 {
    match opcode {
        Opcode::STOP => 0x00,
        Opcode::ADD => 0x01,
        Opcode::MUL => 0x02,
        Opcode::SUB => 0x03,
        Opcode::DIV => 0x04,
        Opcode::SDIV => 0x05,
        Opcode::MOD => 0x06,
        Opcode::SMOD => 0x07,
        Opcode::ADDMOD => 0x08,
        Opcode::MULMOD => 0x09,
        Opcode::EXP => 0x0A,
        Opcode::SIGNEXTEND => 0x0B,
        Opcode::LT => 0x10,
        Opcode::GT => 0x11,
        Opcode::SLT => 0x12,
        Opcode::SGT => 0x13,
        Opcode::EQ => 0x14,
        Opcode::ISZERO => 0x15,
        Opcode::AND => 0x16,
        Opcode::OR => 0x17,
        Opcode::XOR => 0x18,
        Opcode::NOT => 0x19,
        Opcode::BYTE => 0x1A,
        Opcode::SHL => 0x1B,
        Opcode::SHR => 0x1C,
        Opcode::SAR => 0x1D,
        Opcode::KECCAK256 => 0x20,
        Opcode::ADDRESS => 0x30,
        Opcode::BALANCE => 0x31,
        Opcode::ORIGIN => 0x32,
        Opcode::CALLER => 0x33,
        Opcode::CALLVALUE => 0x34,
        Opcode::CALLDATALOAD => 0x35,
        Opcode::CALLDATASIZE => 0x36,
        Opcode::CALLDATACOPY => 0x37,
        Opcode::CODESIZE => 0x38,
        Opcode::CODECOPY => 0x39,
        Opcode::GASPRICE => 0x3A,
        Opcode::EXTCODESIZE => 0x3B,
        Opcode::EXTCODECOPY => 0x3C,
        Opcode::RETURNDATASIZE => 0x3D,
        Opcode::RETURNDATACOPY => 0x3E,
        Opcode::EXTCODEHASH => 0x3F,
        Opcode::BLOCKHASH => 0x40,
        Opcode::COINBASE => 0x41,
        Opcode::TIMESTAMP => 0x42,
        Opcode::NUMBER => 0x43,
        Opcode::DIFFICULTY => 0x44,
        Opcode::GASLIMIT => 0x45,
        Opcode::CHAINID => 0x46,
        Opcode::SELFBALANCE => 0x47,
        Opcode::BASEFEE => 0x48,
        Opcode::BLOBHASH => 0x49,
        Opcode::BLOBBASEFEE => 0x4A,
        Opcode::POP => 0x50,
        Opcode::MLOAD => 0x51,
        Opcode::MSTORE => 0x52,
        Opcode::MSTORE8 => 0x53,
        Opcode::SLOAD => 0x54,
        Opcode::SSTORE => 0x55,
        Opcode::JUMP => 0x56,
        Opcode::JUMPI => 0x57,
        Opcode::PC => 0x58,
        Opcode::MSIZE => 0x59,
        Opcode::GAS => 0x5A,
        Opcode::JUMPDEST => 0x5B,
        Opcode::TLOAD => 0x5C,
        Opcode::TSTORE => 0x5D,
        Opcode::MCOPY => 0x5E,
        Opcode::PUSH0 => 0x5F,
        Opcode::PUSH1 => 0x60,
        Opcode::PUSH2 => 0x61,
        Opcode::PUSH3 => 0x62,
        Opcode::PUSH4 => 0x63,
        Opcode::PUSH5 => 0x64,
        Opcode::PUSH6 => 0x65,
        Opcode::PUSH7 => 0x66,
        Opcode::PUSH8 => 0x67,
        Opcode::PUSH9 => 0x68,
        Opcode::PUSH10 => 0x69,
        Opcode::PUSH11 => 0x6A,
        Opcode::PUSH12 => 0x6B,
        Opcode::PUSH13 => 0x6C,
        Opcode::PUSH14 => 0x6D,
        Opcode::PUSH15 => 0x6E,
        Opcode::PUSH16 => 0x6F,
        Opcode::PUSH17 => 0x70,
        Opcode::PUSH18 => 0x71,
        Opcode::PUSH19 => 0x72,
        Opcode::PUSH20 => 0x73,
        Opcode::PUSH21 => 0x74,
        Opcode::PUSH22 => 0x75,
        Opcode::PUSH23 => 0x76,
        Opcode::PUSH24 => 0x77,
        Opcode::PUSH25 => 0x78,
        Opcode::PUSH26 => 0x79,
        Opcode::PUSH27 => 0x7A,
        Opcode::PUSH28 => 0x7B,
        Opcode::PUSH29 => 0x7C,
        Opcode::PUSH30 => 0x7D,
        Opcode::PUSH31 => 0x7E,
        Opcode::PUSH32 => 0x7F,
        Opcode::DUP1 => 0x80,
        Opcode::DUP2 => 0x81,
        Opcode::DUP3 => 0x82,
        Opcode::DUP4 => 0x83,
        Opcode::DUP5 => 0x84,
        Opcode::DUP6 => 0x85,
        Opcode::DUP7 => 0x86,
        Opcode::DUP8 => 0x87,
        Opcode::DUP9 => 0x88,
        Opcode::DUP10 => 0x89,
        Opcode::DUP11 => 0x8A,
        Opcode::DUP12 => 0x8B,
        Opcode::DUP13 => 0x8C,
        Opcode::DUP14 => 0x8D,
        Opcode::DUP15 => 0x8E,
        Opcode::DUP16 => 0x8F,
        Opcode::SWAP1 => 0x90,
        Opcode::SWAP2 => 0x91,
        Opcode::SWAP3 => 0x92,
        Opcode::SWAP4 => 0x93,
        Opcode::SWAP5 => 0x94,
        Opcode::SWAP6 => 0x95,
        Opcode::SWAP7 => 0x96,
        Opcode::SWAP8 => 0x97,
        Opcode::SWAP9 => 0x98,
        Opcode::SWAP10 => 0x99,
        Opcode::SWAP11 => 0x9A,
        Opcode::SWAP12 => 0x9B,
        Opcode::SWAP13 => 0x9C,
        Opcode::SWAP14 => 0x9D,
        Opcode::SWAP15 => 0x9E,
        Opcode::SWAP16 => 0x9F,
        Opcode::LOG0 => 0xA0,
        Opcode::LOG1 => 0xA1,
        Opcode::LOG2 => 0xA2,
        Opcode::LOG3 => 0xA3,
        Opcode::LOG4 => 0xA4,
        Opcode::CREATE => 0xF0,
        Opcode::CALL => 0xF1,
        Opcode::CALLCODE => 0xF2,
        Opcode::RETURN => 0xF3,
        Opcode::DELEGATECALL => 0xF4,
        Opcode::CREATE2 => 0xF5,
        Opcode::STATICCALL => 0xFA,
        Opcode::REVERT => 0xFD,
        Opcode::INVALID => 0xFE,
        Opcode::SELFDESTRUCT => 0xFF,
    }
}

pub fn push_size(opcode: &Opcode) -> Option<usize> {
    if is_push(opcode) {
        Some(match opcode {
            Opcode::PUSH0 => 0,
            Opcode::PUSH1 => 1,
            Opcode::PUSH2 => 2,
            Opcode::PUSH3 => 3,
            Opcode::PUSH4 => 4,
            Opcode::PUSH5 => 5,
            Opcode::PUSH6 => 6,
            Opcode::PUSH7 => 7,
            Opcode::PUSH8 => 8,
            Opcode::PUSH9 => 9,
            Opcode::PUSH10 => 10,
            Opcode::PUSH11 => 11,
            Opcode::PUSH12 => 12,
            Opcode::PUSH13 => 13,
            Opcode::PUSH14 => 14,
            Opcode::PUSH15 => 15,
            Opcode::PUSH16 => 16,
            Opcode::PUSH17 => 17,
            Opcode::PUSH18 => 18,
            Opcode::PUSH19 => 19,
            Opcode::PUSH20 => 20,
            Opcode::PUSH21 => 21,
            Opcode::PUSH22 => 22,
            Opcode::PUSH23 => 23,
            Opcode::PUSH24 => 24,
            Opcode::PUSH25 => 25,
            Opcode::PUSH26 => 26,
            Opcode::PUSH27 => 27,
            Opcode::PUSH28 => 28,
            Opcode::PUSH29 => 29,
            Opcode::PUSH30 => 30,
            Opcode::PUSH31 => 31,
            Opcode::PUSH32 => 32,
            _ => unreachable!(),
        })
    } else {
        None
    }
}

pub fn is_terminating(opcode: &Opcode) -> bool {
    matches!(
        opcode,
        Opcode::JUMP
            | Opcode::JUMPI
            | Opcode::STOP
            | Opcode::RETURN
            | Opcode::REVERT
            | Opcode::SELFDESTRUCT
            | Opcode::INVALID
    )
}

/// define an opcode which is turned into an operation with pushes if specified
#[macro_export]
macro_rules! op {
    ($code:ident) => {
        Operation {
            opcode: Opcode::$code,
            pushes: Vec::new()
        }
    };
    ($code:ident, [$($pushes:expr),+ $(,)?]) => {
        Operation {
            opcode: Opcode::$code,
            pushes: vec![$($pushes),+]
        }
    };
    ($code:ident, $pushes:expr) => {
        Operation {
            opcode: Opcode::$code,
            pushes: vec![$pushes]
        }
    };
}

mod tests {
    use crate::{opcode::Opcode, Operation};

    #[test]
    fn test_op() {
        assert_eq!(
            op!(DUP1),
            Operation {
                opcode: Opcode::DUP1,
                ..Default::default()
            }
        )
    }

    #[test]
    fn op_push() {
        assert_eq!(
            op!(PUSH0),
            Operation {
                opcode: Opcode::PUSH0,
                ..Default::default()
            }
        );

        assert_eq!(
            op!(PUSH1, 0),
            Operation {
                opcode: Opcode::PUSH1,
                pushes: vec![0]
            }
        );

        assert_eq!(
            op!(PUSH2, [255, 255]),
            Operation {
                opcode: Opcode::PUSH2,
                pushes: vec![255, 255]
            }
        );
    }
}
