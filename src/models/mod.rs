pub mod client {
    mod bot;
    mod instruction;
    pub use bot::{Bot, BotFunx};
    pub use instruction::Instruction;
}

mod world;
pub use world::World;

mod ejection;
pub use ejection::Ejection;
