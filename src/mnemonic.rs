
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mnemonic {
	Invalid, Unsupported,
	ADD, ADC, SUB, SBB, CMP, INC, DEC, NEG, IMUL, IDIV, MUL, DIV,
	OR, AND, XOR, NOT, TEST,
	ROL, ROR, RCL, RCR, SHL, SHR, SAL, SAR,
	CBW, CWDE, CDQE, CWD, CDQ, CQO,
	MOV, LEA, MOVSXD, XCHG,
	NOP, PAUSE, HLT,
	IN, OUT,
	INS, OUTS, MOVS, CMPS, STOS, LODS, SCAS,
	CALLF, RETF, JMPF, CALL, RET, JMP, JCC,
	INT, INTO, IRET,
	BOUND, XLAT, ARPL, LFP,
	PUSH, POP, PUSHA, POPA, PUSHF, POPF, ENTER, LEAVE,
	SAHF, LAHF, SALC, CMC, CLC, STC, CLI, STI, CLD, STD,
	DAA, DAS, AAA, AAS, AAM, AMX, AAD, ADX,
}
