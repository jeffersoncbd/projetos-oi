use crate::{cases::history, Configurations};

pub fn run(configurations: Configurations) -> Result<(), &'static str> {
    let executions = history::read(&configurations.history_folder_path)?;
    Ok(())
}
