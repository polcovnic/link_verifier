// commands/command_interface.rs

pub trait Command {
    type Output;
    fn execute(&self) -> Self::Output;
}
