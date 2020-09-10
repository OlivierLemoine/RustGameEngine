use clap::Clap;
use inflector::Inflector;

pub enum CLIRes {
    Run(std::path::PathBuf),
    Stop,
}

#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}
#[derive(Clap, Debug)]
enum SubCommand {
    NewComponent(NewComponent),
    Build(Build),
    Run(Run),
}
#[derive(Clap, Debug)]
struct NewComponent {
    #[clap(short)]
    path: String,
}
#[derive(Clap, Debug)]
struct Build {
    #[clap(short)]
    path: Option<String>,
}
#[derive(Clap, Debug)]
struct Run {
    #[clap(short)]
    path: Option<String>,
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

fn build(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut component_path = path.clone();
    component_path.push("components.list");
    let components_list_raw = std::fs::read_to_string(&component_path)?;
    let components_list = components_list_raw
        .split('\n')
        .map(|v| v.trim())
        .collect::<Vec<_>>();

    for p in components_list {
        let mut path = path.clone();
        path.push(p);
        path.push("src/prelude.rs");

        std::fs::copy("./resources/entity/src/prelude.rs", &path)?;

        path.pop();
        path.pop();

        let mut proc = std::process::Command::new("cargo");
        proc.current_dir(path).arg("build");
        if !cfg!(debug_assertions) {
            proc.arg("--release");
        }
        proc.spawn()?.wait()?;
    }

    Ok(())
}

pub fn run() -> Result<CLIRes, Box<dyn std::error::Error>> {
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
        SubCommand::Build(Build { path }) => {
            let path = std::path::PathBuf::from(path.unwrap_or(".".into()));
            build(path)?;
        }
        SubCommand::Run(Run { path }) => {
            let path = std::path::PathBuf::from(path.unwrap_or(".".into()));
            build(path.clone())?;

            return Ok(CLIRes::Run(path));
        }
    }
    Ok(CLIRes::Stop)
}
