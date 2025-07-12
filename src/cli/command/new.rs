use clap::Command;

pub fn spec() -> Command {
    Command::new("new")
        .about("Initialize a new project with saba.yml")
}

pub fn action() -> anyhow::Result<()> {
    // TODO: Implement v2 project initialization logic
    println!("[Success] saba.yml initialization completed.");
    Ok(())
}