// ---- Register (settable) enum ---------------------------------------------

use crate::recorder::protocol::payload_helpers::is_not_cr;
use std::{fmt, str::FromStr};

use winnow::{
    Parser,
    token::{literal, take_while},
};

use crate::recorder::protocol::{
    In, ParseFn, Value,
    control_chars::{CR, ESC, RCDR, RCDR_LOWER},
    instructions::{Instruction, UnknownInstruction},
    parser_of,
    payload_helpers::{normalize, shorten},
};

//
// ---- Register / token constants from the SIS protocol ---------------------

/// A built-in metadata register that can be written to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Coverage,
    Presenter,
    Relation,
    Source,
    Subject,
    Title,
}

impl Register {
    pub const ALL: &'static [Register] = &[
        Register::Coverage,
        Register::Presenter,
        Register::Relation,
        Register::Source,
        Register::Subject,
        Register::Title,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Register::Coverage => "COVERAGE",
            Register::Presenter => "PRESENTER",
            Register::Relation => "RELATION",
            Register::Source => "SOURCE",
            Register::Subject => "SUBJECT",
            Register::Title => "TITLE",
        }
    }

    fn reg(self) -> &'static str {
        match self {
            Register::Coverage => "M1",
            Register::Presenter => "M2",
            Register::Relation => "M9",
            Register::Source => "M11",
            Register::Subject => "M12",
            Register::Title => "M13",
        }
    }

    /// Build the wire instruction that writes `value` into this register. The
    /// value is truncated to [`MAX_VALUE_LEN`] characters, matching the device.
    pub fn instruction(self, value: &str) -> Instruction {
        let reg = self.reg();
        let value = shorten(value, MAX_VALUE_LEN);
        let payload = format!("{ESC}{reg}*{value}{RCDR}{CR}");
        Instruction {
            name: self.name().to_string(),
            payload,
            parser: settable_echo(reg),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Register {
    type Err = UnknownInstruction;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match normalize(s).as_str() {
            "COVERAGE" => Ok(Register::Coverage),
            "PRESENTER" => Ok(Register::Presenter),
            "RELATION" => Ok(Register::Relation),
            "SOURCE" => Ok(Register::Source),
            "SUBJECT" => Ok(Register::Subject),
            "TITLE" => Ok(Register::Title),
            _ => Err(UnknownInstruction(s.to_string())),
        }
    }
}

/// Maximum value length the SMP 351 accepts for a settable register.
pub const MAX_VALUE_LEN: usize = 127;

/// Echo after writing a register: `Rcdr<reg>*<value> CR`.
fn settable_echo(reg: &str) -> ParseFn {
    let head = format!("{RCDR_LOWER}{reg}*");
    parser_of(
        move |i: &mut In| {
            literal(head.as_str()).parse_next(i)?;
            let v: &str = take_while(0.., is_not_cr).parse_next(i)?;
            literal("\r").parse_next(i)?;
            Ok(v.to_string())
        },
        Value::Text,
    )
}
