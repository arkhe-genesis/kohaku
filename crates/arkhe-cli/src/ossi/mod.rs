use clap::Subcommand;

#[derive(Subcommand)]
pub enum OssiCommand {
    /// Run constitutional verifiers
    Verify,
    /// Launch holographic renderer
    Render { format: Option<String> },
    /// Execute autonomous taskset
    Task { skill: String },
    /// Run stress-test harness
    Stress { substrate: String, scale: Option<u64> },
    /// Clone codebase
    Code { substrate: String },
    /// Start world simulation
    World { name: String },
}

pub async fn handler(args: OssiCommand) {
    match args {
        OssiCommand::Verify => { println!("Running OSSI Verify..."); }
        OssiCommand::Render { format } => { println!("Running OSSI Render with format: {:?}", format); }
        OssiCommand::Task { skill } => { println!("Running OSSI Task with skill: {:?}", skill); }
        OssiCommand::Stress { substrate, scale } => { println!("Running OSSI Stress on {} scale {:?}", substrate, scale); }
        OssiCommand::Code { substrate } => { println!("Running OSSI Code for {:?}", substrate); }
        OssiCommand::World { name } => { println!("Running OSSI World {:?}", name); }
    }
}
