use crate::asm::ast;

mod macros;

macros::isa! {
	pub instructions {
		NOP               = 0x0,
		HLT               = 0x1,
		ADD(a, b, c)      = 0x2,
		SUB(a, b, c)      = 0x3,
		NOR(a, b, c)      = 0x4,
		AND(a, b, c)      = 0x5,
		XOR(a, b, c)      = 0x6,
		RSH(a,    c)      = 0x7,
		LDI(a,  imm)      = 0x8,
		ADI(a,  imm)      = 0x9,
		JMP(      addr)   = 0xA,
		BRH(cond, addr)   = 0xB,
		CAL(      addr)   = 0xC,
		RET               = 0xD,
		LOD(a, b, offset) = 0xE,
		STR(a, b, offset) = 0xF,
	}
	
	pub aliases {
		CMP(a, b) => SUB(a, b, 0),
		MOV(a, c) => ADD(a, 0, c),
		LSH(a, c) => ADD(a, a, c),
		INC(a)    => ADI(a, 1),
		DEC(a)    => ADI(a, 0xFF),
		NOT(a, c) => NOR(a, 0, c),
		NEG(a, c) => SUB(0, a, c),
	}
}

impl TryFrom<&ast::ResolvedInstruction> for Instruction {
	type Error = String;
	
	fn try_from(resolved: &ast::ResolvedInstruction) -> Result<Self, String> {
		Instruction::new(Mnemonic::try_from(resolved.mnemonic.as_str())?, resolved.operands.iter().copied())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn to_machine_code() {
		assert_eq!(Instruction::NOP.as_u16(),                                0x0000);
		assert_eq!(Instruction::HLT.as_u16(),                                0x1000);
		assert_eq!(Instruction::ADD{ a: 0x3, b: 0x4, c: 0x5 }.as_u16(),      0x2345);
		assert_eq!(Instruction::SUB{ a: 0x9, b: 0x3, c: 0x9 }.as_u16(),      0x3939);
		assert_eq!(Instruction::NOR{ a: 0xE, b: 0xE, c: 0xF }.as_u16(),      0x4EEF);
		assert_eq!(Instruction::AND{ a: 0x4, b: 0x0, c: 0x5 }.as_u16(),      0x5405);
		assert_eq!(Instruction::XOR{ a: 0x1, b: 0x2, c: 0x3 }.as_u16(),      0x6123);
		assert_eq!(Instruction::RSH{ a: 0xA,         c: 0xB }.as_u16(),      0x7A0B);
		assert_eq!(Instruction::LDI{ a: 0x9,       imm: 0xFF }.as_u16(),     0x89FF);
		assert_eq!(Instruction::ADI{ a: 0x8,       imm: 0x42 }.as_u16(),     0x9842);
		assert_eq!(Instruction::JMP{              addr: 0x3FF }.as_u16(),    0xA3FF);
		assert_eq!(Instruction::BRH{ cond: 0x0,   addr: 0x222 }.as_u16(),    0xB222);
		assert_eq!(Instruction::BRH{ cond: 0x1,   addr: 0x123 }.as_u16(),    0xB523);
		assert_eq!(Instruction::BRH{ cond: 0x2,   addr: 0x009 }.as_u16(),    0xB809);
		assert_eq!(Instruction::BRH{ cond: 0x3,   addr: 0x3FF }.as_u16(),    0xBFFF);
		assert_eq!(Instruction::CAL{              addr: 0x137 }.as_u16(),    0xC137);
		assert_eq!(Instruction::RET.as_u16(),                                0xD000);
		assert_eq!(Instruction::LOD{ a: 0x1, b: 0x2, offset: 0x3 }.as_u16(), 0xE123);
		assert_eq!(Instruction::STR{ a: 0xF, b: 0xF, offset: 0xF }.as_u16(), 0xFFFF);
	}
	
	#[test]
	fn from_machine_code() {
		assert_eq!(Instruction::NOP,                                Instruction::from(0x0000));
		assert_eq!(Instruction::HLT,                                Instruction::from(0x1000));
		assert_eq!(Instruction::ADD{ a: 0x3, b: 0x4, c: 0x5 },      Instruction::from(0x2345));
		assert_eq!(Instruction::SUB{ a: 0x9, b: 0x3, c: 0x9 },      Instruction::from(0x3939));
		assert_eq!(Instruction::NOR{ a: 0xE, b: 0xE, c: 0xF },      Instruction::from(0x4EEF));
		assert_eq!(Instruction::AND{ a: 0x4, b: 0x0, c: 0x5 },      Instruction::from(0x5405));
		assert_eq!(Instruction::XOR{ a: 0x1, b: 0x2, c: 0x3 },      Instruction::from(0x6123));
		assert_eq!(Instruction::RSH{ a: 0xA,         c: 0xB },      Instruction::from(0x7A0B));
		assert_eq!(Instruction::LDI{ a: 0x9,       imm: 0xFF },     Instruction::from(0x89FF));
		assert_eq!(Instruction::ADI{ a: 0x8,       imm: 0x42 },     Instruction::from(0x9842));
		assert_eq!(Instruction::JMP{              addr: 0x3FF },    Instruction::from(0xA3FF));
		assert_eq!(Instruction::BRH{ cond: 0x0,   addr: 0x222 },    Instruction::from(0xB222));
		assert_eq!(Instruction::BRH{ cond: 0x1,   addr: 0x123 },    Instruction::from(0xB523));
		assert_eq!(Instruction::BRH{ cond: 0x2,   addr: 0x009 },    Instruction::from(0xB809));
		assert_eq!(Instruction::BRH{ cond: 0x3,   addr: 0x3FF },    Instruction::from(0xBFFF));
		assert_eq!(Instruction::CAL{              addr: 0x137 },    Instruction::from(0xC137));
		assert_eq!(Instruction::RET,                                Instruction::from(0xD000));
		assert_eq!(Instruction::LOD{ a: 0x1, b: 0x2, offset: 0x3 }, Instruction::from(0xE123));
		assert_eq!(Instruction::STR{ a: 0xF, b: 0xF, offset: 0xF }, Instruction::from(0xFFFF));
	}
	
	#[test]
	fn from_ast() {
		assert_eq!(Instruction::NOP,                                Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "NOP".into(), operands: vec![] }).unwrap());
		assert_eq!(Instruction::HLT,                                Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "HLT".into(), operands: vec![] }).unwrap());
		assert_eq!(Instruction::ADD{ a: 0x3, b: 0x4, c: 0x5 },      Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "ADD".into(), operands: vec![0x3, 0x4, 0x5] }).unwrap());
		assert_eq!(Instruction::SUB{ a: 0x9, b: 0x3, c: 0x9 },      Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "SUB".into(), operands: vec![0x9, 0x3, 0x9] }).unwrap());
		assert_eq!(Instruction::NOR{ a: 0xE, b: 0xE, c: 0xF },      Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "NOR".into(), operands: vec![0xE, 0xE, 0xF] }).unwrap());
		assert_eq!(Instruction::AND{ a: 0x4, b: 0x0, c: 0x5 },      Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "AND".into(), operands: vec![0x4, 0x0, 0x5] }).unwrap());
		assert_eq!(Instruction::XOR{ a: 0x1, b: 0x2, c: 0x3 },      Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "XOR".into(), operands: vec![0x1, 0x2, 0x3] }).unwrap());
		assert_eq!(Instruction::RSH{ a: 0xA,         c: 0xB },      Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "RSH".into(), operands: vec![0xA, 0xB] }).unwrap());
		assert_eq!(Instruction::LDI{ a: 0x9,       imm: 0xFF },     Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "LDI".into(), operands: vec![0x9, 0xFF] }).unwrap());
		assert_eq!(Instruction::ADI{ a: 0x8,       imm: 0x42 },     Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "ADI".into(), operands: vec![0x8, 0x42] }).unwrap());
		assert_eq!(Instruction::JMP{              addr: 0x3FF },    Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "JMP".into(), operands: vec![0x3FF] }).unwrap());
		assert_eq!(Instruction::BRH{ cond: 0x0,   addr: 0x222 },    Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "BRH".into(), operands: vec![0x0, 0x222] }).unwrap());
		assert_eq!(Instruction::BRH{ cond: 0x1,   addr: 0x123 },    Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "BRH".into(), operands: vec![0x1, 0x123] }).unwrap());
		assert_eq!(Instruction::BRH{ cond: 0x2,   addr: 0x009 },    Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "BRH".into(), operands: vec![0x2, 0x009] }).unwrap());
		assert_eq!(Instruction::BRH{ cond: 0x3,   addr: 0x3FF },    Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "BRH".into(), operands: vec![0x3, 0x3FF] }).unwrap());
		assert_eq!(Instruction::CAL{              addr: 0x137 },    Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "CAL".into(), operands: vec![0x137] }).unwrap());
		assert_eq!(Instruction::RET,                                Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "RET".into(), operands: vec![] }).unwrap());
		assert_eq!(Instruction::LOD{ a: 0x1, b: 0x2, offset: 0x3 }, Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "LOD".into(), operands: vec![0x1, 0x2, 0x3] }).unwrap());
		assert_eq!(Instruction::STR{ a: 0xF, b: 0xF, offset: 0xF }, Instruction::try_from(&ast::ResolvedInstruction { mnemonic: "STR".into(), operands: vec![0xF, 0xF, 0xF] }).unwrap());
	}
}
