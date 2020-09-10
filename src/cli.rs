use clap::Clap;
use inflector::Inflector;

#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}
#[derive(Clap, Debug)]
enum SubCommand {
    NewComponent(NewComponent),
    Build,
}
#[derive(Clap, Debug)]
struct NewComponent {
    #[clap(short)]
    path: String,
}

fn replace_in_string(s: String, entity_name: &str) -> String {
    s.replace("__CHANGE_ME_ENTITY_NAME__", &entity_name)
        .replace(
            "__CHANGE_ME_ENTITY_NAME_CLASS_CASE__",
            &entity_name.to_class_case(),
        )
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
            let name = replace_in_string(
                entry
                    .file_name()
                    .to_str()
                    .map(|v| String::from(v))
                    .ok_or("")?,
                entity_name,
            );

            to.push(name);

            std::fs::write(
                &to,
                replace_in_string(std::fs::read_to_string(entry.path())?, entity_name),
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
        SubCommand::Build => {
            let components_list_raw = std::fs::read_to_string("components.list")?;
            let components_list = components_list_raw
                .split('\n')
                .map(|v| v.trim())
                .collect::<Vec<_>>();

            for path in components_list {
                let mut proc = std::process::Command::new("cargo");
                proc.current_dir(path).arg("build");
                if !cfg!(debug_assertions) {
                    proc.arg("--release");
                }
                proc.spawn()?;
            }
        }
    }
    Ok(())
}
