use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, stderr, stdout, BufReader};
use std::path::Path;
use std::process::Command;

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
 * [ ] Highlight the default project in the listing
 * [ ] Run the last task with `rx -`
 * [ ] set default project (for rx)
 *     [ ] use monorepo default project as a default value
 * [ ] Show tasks directly for specified project
 * [ ] Support Esc to cancel project and task selection and go back or quit
 **/
fn main() {
    // Detection if we are in monorepo
    let file_result = File::open("nx.json");
    let f = match file_result {
        Ok(file) => file,
        Err(error) => panic!("nx.json file not found. Folder not recognized as Nx monorepo."),
    };

    // Get default project
    // TODO:

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
    let cmd = format!("{}:{}", selected_project, selected_job);

    println!("Running nx command {}", cmd);

    let _output = Command::new("pnpm")
        .arg("nx")
        .arg("run")
        .arg(cmd)
        .spawn()
        .unwrap();

    // TODO: better output handling in the console
}
