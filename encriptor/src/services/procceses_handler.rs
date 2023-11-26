use std::{thread, time};
use std::io::Read;
use std::process::{Child, ChildStdout, Command, Stdio};

pub fn run_processes(process_path: &str, processes_count: u8, args: Vec<Vec<&str>>) {
    if args.len() != processes_count as usize {
        panic!("Invalid number of arguments for the given number of processes!");
    }

    let mut commands = Vec::new();
    for arg in args {
        commands.push((process_path, arg));
    }

    let mut children_processes = start_processes(commands);

    let outputs = process_children(&mut children_processes);

    for (_, output) in outputs {
        println!("Output: {}", output);
    }
}

fn process_children(
    children_processes: &mut Vec<(usize, Child, ChildStdout)>,
) -> Vec<(usize, String)> {
    let mut outputs: Vec<(usize, String)> = Vec::new();
    let mut finished_indices = Vec::new();

    while !children_processes.is_empty() {
        for (index, child, child_stdout) in children_processes.iter_mut() {
            if let Ok(Some(_status)) = child.try_wait() {
                let mut output = String::new();
                child_stdout
                    .read_to_string(&mut output)
                    .expect("Failed to read stdout");
                outputs.push((*index, output));
                finished_indices.push(*index);
            }
        }

        children_processes.retain(|(index, _, _)| !finished_indices.contains(index));
        finished_indices.clear();

        thread::sleep(time::Duration::from_millis(100));
    }

    outputs.sort_by_key(|(index, _)| *index);
    outputs
}

fn start_processes(commands: Vec<(&str, Vec<&str>)>) -> Vec<(usize, Child, ChildStdout)> {
    let mut children = Vec::new();

    for (i, (program, args)) in commands.into_iter().enumerate() {
        let args_joined = args.join(" ");

        let mut child = Command::new(program)
            .arg(args_joined)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let stdout = child.stdout.take().expect("Failed to take stdout of child");
        children.push((i, child, stdout));
    }

    return children;
}
