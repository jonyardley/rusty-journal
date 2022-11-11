use anyhow::anyhow;
mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to load Journal file."))?;

    let fact = get_features().await;
    println!("features = {:#?}", fact);

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Feature {
    splitName: String,
    treatment: String,
}

const FEATURE_URL: &str =
    "http://localhost:7548/client/get-treatment?key=my-customer-key&split-name=new_feature";

async fn get_features() -> Result<Feature, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get(FEATURE_URL)
        .send()
        .await?
        .json::<Feature>()
        .await?;

    Ok(body)
}
