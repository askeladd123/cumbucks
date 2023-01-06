use std::array::IntoIter;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    WorkHard,
    WorkEasy,
    TaskHard,
    TaskEasy,
    RewardSmall,
    RewardBig,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Instruction::*;
        f.write_str(match self {
            WorkHard => "work hard",
            WorkEasy => "work easy",
            TaskHard => "task hard",
            TaskEasy => "task easy",
            RewardSmall => "reward small",
            RewardBig => "reward big",
        })
    }
}

impl Instruction {
    pub fn iter() -> IntoIter<Instruction, 6> {
        use Instruction::*;
        [
            WorkHard,
            WorkEasy,
            TaskHard,
            TaskEasy,
            RewardSmall,
            RewardBig,
        ]
        .into_iter()
    }
}
