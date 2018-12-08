use std::cmp::max;
use step::Step;

/// Solver for part 2 - returns the ammount of time required to build the sleigh,
/// given a bias (bias + letter position in alphabet = time to complete step) and
/// number of workers.
pub fn get_time_to_complete(steps: &Vec<Step>, bias: u32, workers: u32) -> u32 {
    let mut second: u32 = 0;

    // Get a new list for steps left to complete and sort it
    let mut steps_left: Vec<&Step> = steps.iter().collect();
    steps_left.sort_unstable_by(|step1, step2| step1.get_id().cmp(&step2.get_id()));

    // List of completed steps
    let mut completed_steps: Vec<char> = Vec::new();

    // Create a list of tasks each worker is working on
    // (step, second started)
    let mut tasks: Vec<(char, u32)> = Vec::with_capacity(workers as usize);

    // Initialize task list for all workers not doing anything
    for _ in 0..workers {
        // ' ' indicates the worker is not working on anything
        tasks.push((' ', 0));
    }

    // Loop while there are still steps to complete
    while steps_left.len() > 0 {
        // Check if task is done
        for task_index in 0..tasks.len() {
            // Check if task is being worked on
            if tasks[task_index].0 != ' ' {
                // If so, check if it's done yet
                let time_for_task = bias + tasks[task_index].0 as u32 - 'A' as u32 + 1;
                if second - tasks[task_index].1 >= time_for_task {
                    completed_steps.push(tasks[task_index].0);
                    tasks[task_index].0 = ' ';
                }
            }
        }

        // Check if workers need new tasks
        for task_index in 0..tasks.len() {
            // Skip this worker if they have a task
            if tasks[task_index].0 != ' ' {
                continue;
            }

            // Find a step for the worker to work on
            let mut remove_index = steps_left.len();
            for (index, &step) in steps_left.iter().enumerate() {
                if step_available(step, &completed_steps) {
                    // set worker to this step
                    tasks[task_index].0 = step.get_id();
                    tasks[task_index].1 = second;

                    // Remove from list
                    remove_index = index;
                    break;
                }
            }

            // If a step was found (remove_index < steps_left.len()) remove it from the available steps
            if remove_index < steps_left.len() {
                steps_left.remove(remove_index);
            }
        }
        // Increment second counter
        second += 1;
    }

    // Simulate rest of work
    let mut seconds_left = 0;
    for (task, start_time) in tasks.into_iter().filter(|task| task.0 != ' ') {
        let time_for_task = bias + task as u32 - 'A' as u32 + 1;
        seconds_left = max(seconds_left, time_for_task - (second - start_time));
    }

    return second + seconds_left;
}

/// Solver for part 1 - returns a string with each step in order
pub fn get_ordered_steps(steps: &Vec<Step>) -> String {
    // Get a new list of references to each step, sorted in order
    let mut list: Vec<&Step> = steps.iter().collect();
    list.sort_unstable_by(|step1, step2| step1.get_id().cmp(&step2.get_id()));

    // List of steps taken so far
    let mut ordered_steps: Vec<char> = Vec::new();

    // Loop until input list is empty
    while list.len() > 0 {
        // Loop through steps and add the first available one (all prereqs met) to the list
        let mut remove_index = list.len();
        for (index, &step) in list.iter().enumerate() {
            if step_available(step, &ordered_steps) {
                // Add to ordered_steps
                ordered_steps.push(step.get_id());

                // Remove from list
                remove_index = index;
                break;
            }
        }

        if remove_index < list.len() {
            list.remove(remove_index);
        } else {
            println!("No available step found...");
        }
    }

    return ordered_steps.into_iter().collect::<String>();
}

fn step_available(step: &Step, run_steps: &Vec<char>) -> bool {
    let mut prereqs_met: bool = true;

    for prereq in step.get_prerequisites().iter() {
        prereqs_met &= run_steps.contains(prereq);

        if !prereqs_met {
            break;
        }
    }

    return prereqs_met;
}
