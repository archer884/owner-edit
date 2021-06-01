use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io,
};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opts {
    path: String,
    #[structopt(subcommand)]
    command: Command,
    #[structopt(short, long)]
    force: bool,
}

#[derive(Clone, Debug, StructOpt)]
enum Command {
    /// Add an owner
    Add(Change),
    /// List all owners
    List,
    /// Pretty-print the owners file
    Print,
    /// Remove an owner
    #[structopt(name = "rm")]
    Remove(Change),
}

#[derive(Clone, Debug, StructOpt)]
struct Change {
    users: Vec<String>,
    #[structopt(short = "e", long = "environment")]
    environments: Vec<String>,
    #[structopt(short = "g", long = "group")]
    groups: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Permissions {
    #[serde(flatten)]
    groups: BTreeMap<String, DevGroup>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct DevGroup {
    #[serde(flatten)]
    environments: BTreeMap<String, BTreeSet<String>>,
}

struct Filter {
    entries: Vec<String>,
}

impl Filter {
    fn from_entries(entries: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self {
            entries: entries
                .into_iter()
                .map(|entry| entry.as_ref().to_lowercase())
                .collect(),
        }
    }

    fn filter(&self, candidate: &str) -> bool {
        self.entries.is_empty() || self.entries.contains(&candidate.to_lowercase())
    }
}

fn main() {
    let opts = Opts::from_args();
    if let Err(e) = run(&opts) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(opts: &Opts) -> io::Result<()> {
    let content = fs::read_to_string(&opts.path)?;
    let mut permissions: Permissions = serde_json::from_str(&content)?;

    match &opts.command {
        Command::Add(add) => add_user(&mut permissions, add),
        Command::List => {
            list_users(&permissions);
            return Ok(());
        }
        // To pretty print the file is literally a no-op
        Command::Print => (),
        Command::Remove(remove) => remove_user(&mut permissions, remove),
    }

    if opts.force {
        serde_json::to_writer_pretty(&mut File::create(&opts.path)?, &permissions)?;
    } else {
        serde_json::to_writer_pretty(&mut io::stdout(), &permissions)?;
    }

    Ok(())
}

fn add_user(permissions: &mut Permissions, add: &Change) {
    filter_user_listings(add, permissions).for_each(|listing| {
        listing.extend(add.users.iter().cloned());
    });
}

fn remove_user(permissions: &mut Permissions, change: &Change) {
    filter_user_listings(change, permissions).for_each(|listing| {
        change.users.iter().for_each(|user| {
            listing.remove(user);
        });
    });
}

fn list_users(permissions: &Permissions) {
    let users: BTreeSet<_> = permissions
        .groups
        .iter()
        .flat_map(|(_, group)| group.environments.iter())
        .flat_map(|(_, environment)| environment)
        .collect();

    for user in users {
        println!("{}", user);
    }
}

fn filter_user_listings<'a>(
    change: &Change,
    permissions: &'a mut Permissions,
) -> impl Iterator<Item = &'a mut BTreeSet<String>> {
    let group_filter = Filter::from_entries(&change.groups);
    let environment_filter = Filter::from_entries(&change.environments);

    permissions
        .groups
        .iter_mut()
        .filter(move |group| group_filter.filter(&group.0))
        .flat_map(|group| &mut group.1.environments)
        .filter(move |environment| environment_filter.filter(&environment.0))
        .map(|environment| environment.1)
}
