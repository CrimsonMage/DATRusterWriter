#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct MotionCommand(pub u32);

impl From<u32> for MotionCommand {
    fn from(value: u32) -> Self { Self(value) }
}

impl From<MotionCommand> for u32 {
    fn from(value: MotionCommand) -> Self { value.0 }
}
