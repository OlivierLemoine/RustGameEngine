use clap::Clap;

#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    NewComponent(NewComponent),
}

#[derive(Clap, Debug)]
struct NewComponent {
    #[clap(short)]
    path: String,
}

fn copy_dir_recurs(
    from: std::path::PathBuf,
    mut to: std::path::PathBuf,
    entity_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(to.clone())?;
    for entry in std::fs::read_dir(from.clone())? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            let mut new_from = from.clone();
            let mut new_to = to.clone();
            new_from.push(entry.file_name());
            new_to.push(entry.file_name());
            copy_dir_recurs(new_from, new_to, entity_name)?;
        } else {
            let name = entry.file_name();
            to.push(name);

            std::fs::write(
                &to,
                std::fs::read_to_string(entry.path())?
                    .replace("__CHANGE_ME_ENTITY_NAME__", entity_name),
            )?;
            to.pop();
        }
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    match opts.subcommand {
        SubCommand::NewComponent(NewComponent { path }) => {
            let to = std::path::PathBuf::from(path);
            let name = to
                .clone()
                .file_name()
                .ok_or("No name")?
                .to_str()
                .ok_or("No name")?
                .to_string();

            copy_dir_recurs(std::path::PathBuf::from("./resources/entity"), to, &name)?;
        }
    }
    Ok(())
}
