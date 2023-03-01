use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, stderr, stdout, BufReader, Error, ErrorKind, Result};
use std::path::Path;
use std::process::{Command, ExitStatus};

#[derive(Serialize, Deserialize, Debug)]
struct NxWorkspaceFile {
    version: u8,
    projects: HashMap<String, String>,
}

// We need a empty struct to correctly parse the project.json files in the projects. This is a
// quick win how to make the parse work and have robust typing as we care only about the task name
// and not it's definition.
#[derive(Serialize, Deserialize, Debug)]
struct Target {
    // executor: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NxProjectFile {
    name: String,
    tags: Vec<String>,
    targets: HashMap<String, Target>,
}

/*
 * [x] Show all projects in monorepo
 * [x] Select project and prject's task
 * [x] Run the task from the project
 * [x] improve terminal output
 * [ ] Run the last task with `rx -`
 *     [x] store last run command - fail to write into existing file
 *     [x] parse `-` args - ignoring the -
 *     [x] read last run command and run it
 * [ ] Split code to smaller functions
 *
 * [ ] passing `-` arg when there is not file or argument available shouldn't fail
 * [ ] Support Esc to cancel project and task selection and go back or quit
 * [ ] Highlight the default project in the listing
 * [ ] Show tasks directly for specified project
 * [ ] set default project (for rx)
 *     [ ] use monorepo default project as a default value
 * [ ] Support passing additional arguments to the command
 * [ ] Add tests
 **/
fn main() {
    static LAST_COMMAND_PREFIX: &str = "last_command=";

    // Detection if we are in monorepo
    let file_result = File::open("nx.json");
    let f = match file_result {
        Ok(file) => file,
        Err(error) => panic!("nx.json file not found. Folder not recognized as Nx monorepo."),
    };

    let args: Vec<String> = env::args().collect();
    let repeat = parse_args(&args);
    let mut cmd: String = String::new();

    if repeat {
        let mut rx_file = open_rx_file();
        let mut buffer = String::new();

        rx_file.read_to_string(&mut buffer).unwrap();

        for line in buffer.lines() {
            if line.starts_with(LAST_COMMAND_PREFIX) {
                cmd = line.strip_prefix(LAST_COMMAND_PREFIX).unwrap().to_string();
            }
        }
    } else {
        // TODO: Get default project

        // Read and parse Nx projects in the monorepo
        let ws_file = File::open("workspace.json").unwrap();
        let reader = BufReader::new(ws_file);

        let workspace_json: NxWorkspaceFile = serde_json::from_reader(reader).unwrap();

        // Propare items
        // TODO: mark default project
        let items: Vec<String> = workspace_json.projects.keys().cloned().collect();

        let selected_index: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .with_prompt("Select a project")
            .interact()
            .unwrap();

        let selected_project = &items.get(selected_index).unwrap().to_string();
        let project_path = workspace_json.projects.get(selected_project).unwrap();

        // Select a command
        let path = Path::new(project_path).join("project.json");
        let project_file = File::open(path).unwrap();
        let project_file_reader = BufReader::new(project_file);

        let project_json: NxProjectFile = serde_json::from_reader(project_file_reader).unwrap();

        let job_items: Vec<String> = project_json.targets.keys().cloned().collect();

        let selected_job_index: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&job_items)
            .default(0)
            .with_prompt("Select a task")
            .interact()
            .unwrap();
        let selected_job = &job_items.get(selected_job_index).unwrap().to_string();

        // Run command
        cmd = format!("{}:{}", selected_project, selected_job);
    }

    if cmd.len() == 0 {
        panic!("Some error")
    }

    println!("Running nx command {}", cmd);

    let mut child = Command::new("pnpm")
        .arg("nx")
        .arg("run")
        .arg(&cmd)
        .spawn()
        .unwrap();

    let ecode = child.wait().expect("Rx command not run.");

    // Store the last command in a config file
    let mut rx_file = open_rx_file();
    let mut content_buffer = String::new();
    let mut new_content = String::new();

    new_content.push_str(format!("{}{}\n", LAST_COMMAND_PREFIX, cmd).as_str());

    rx_file.read_to_string(&mut content_buffer).unwrap();
    for line in content_buffer.lines() {
        if !line.starts_with(LAST_COMMAND_PREFIX) {
            new_content.push_str(line);
        }
    }

    rx_file.rewind().unwrap();
    rx_file
        .write(new_content.as_bytes())
        .expect("Write to config file failed");

    std::process::exit(ecode.code().unwrap())
}

fn open_rx_file() -> File {
    static RX_FILE_PATH: &str = ".rx";

    let rx_file_res: Result<File> = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(RX_FILE_PATH);

    match rx_file_res {
        Ok(file) => file,
        Err(_) => panic!("Failed to read or create .rx file"),
    }
}

fn parse_args(args: &[String]) -> bool {
    if args.len() > 1 {
        return args[1] == String::from("-");
    }

    false
}
