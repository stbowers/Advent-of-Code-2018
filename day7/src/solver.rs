use step::Step;

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
