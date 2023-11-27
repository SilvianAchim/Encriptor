use std::{thread, time};
use std::io::Read;
use std::process::{Child, ChildStdout, Command, Stdio};

use crate::constants::flags::ENCODE_FLAG;
use crate::services::file_actions::write_to_file;

pub fn run_processes(
    process_path: &str,
    processes_count: u8,
    args: Vec<Vec<&str>>,
    flag: &str,
    seed: &str,
) {
    if args.len() != processes_count as usize {
        panic!("Invalid number of arguments for the given number of processes!");
    }

    let start_args = build_start_args(process_path, args, flag, seed);
    let mut children_processes = start_processes(start_args);

    let children_outputs = collect_processes_outputs(&mut children_processes);

    write_output(flag, seed, children_outputs);
}

fn write_output(flag: &str, seed: &str, children_outputs: Vec<(usize, String)>) {
    let mut output = String::new();
    if flag == ENCODE_FLAG {
        output.push_str(format!("{}\n", seed).as_str());
    }
    for (_, children_output) in children_outputs {
        output.push_str(&children_output);
    }
    write_to_file("src/output.txt", &output);
}

fn build_start_args<'lifetime>(
    process_path: &'lifetime str,
    args: Vec<Vec<&'lifetime str>>,
    flag: &'lifetime str,
    seed: &'lifetime str,
) -> Vec<(&'lifetime str, String)> {
    let mut commands = Vec::new();
    for arg in args {
        let args_joined = arg.join(" ");
        let formatted_arg = format!("{} {} {}", flag, seed, args_joined);
        commands.push((process_path, formatted_arg));
    }
    return commands;
}

fn collect_processes_outputs(
    children_processes: &mut Vec<(usize, Child, ChildStdout)>,
) -> Vec<(usize, String)> {
    let mut outputs = Vec::new();

    while !children_processes.is_empty() {
        if let Some((index, output)) = read_output_from_next_finished_child(children_processes) {
            outputs.push((index, output));
        }
        thread::sleep(time::Duration::from_millis(100));
    }

    outputs.sort_by_key(|(index, _)| *index);
    return outputs;
}

fn read_output_from_next_finished_child(
    children_processes: &mut Vec<(usize, Child, ChildStdout)>,
) -> Option<(usize, String)> {
    let finished_index = get_index_of_first_finished_process(children_processes);

    if let Some(index) = finished_index {
        let (child_index, _, mut child_stdout) = children_processes.remove(index);
        let mut output = String::new();
        child_stdout
            .read_to_string(&mut output)
            .expect("Failed to read stdout");
        return Some((child_index, output));
    }

    return None;
}

fn get_index_of_first_finished_process(
    children_processes: &mut Vec<(usize, Child, ChildStdout)>,
) -> Option<usize> {
    let mut finished_index = None;

    for (i, (_, child, _)) in children_processes.iter_mut().enumerate() {
        if let Ok(Some(_)) = child.try_wait() {
            finished_index = Some(i);
            break;
        }
    }

    return finished_index;
}

fn start_processes(commands: Vec<(&str, String)>) -> Vec<(usize, Child, ChildStdout)> {
    let mut children = Vec::new();

    for (i, (program, args)) in commands.into_iter().enumerate() {
        let mut child = Command::new(program)
            .args(args.split_whitespace())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let stdout = child.stdout.take().expect("Failed to take stdout of child");
        children.push((i, child, stdout));
    }

    return children;
}
