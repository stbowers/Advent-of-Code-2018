pub struct Step {
    id: char,
    prereqs: Vec<char>,
}

impl Step {
    pub fn new(id: char) -> Step {
        return Step {
            id: id,
            prereqs: Vec::new(),
        };
    }

    pub fn add_prerequisite(&mut self, prereq: char) {
        self.prereqs.push(prereq);
    }

    pub fn get_id(&self) -> char {
        return self.id;
    }

    pub fn get_prerequisites(&self) -> &Vec<char> {
        return &self.prereqs;
    }
}

pub fn parse_steps(lines: Vec<&str>) -> Vec<Step> {
    let mut steps: Vec<(char, Step)> = Vec::new();

    for &line in lines.iter() {
        let (prereq, step): (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", prereq, step);

        // Add prereq as step
        let prereq_search: Result<usize, usize> =
            steps.binary_search_by(|item| item.0.cmp(&prereq));
        if prereq_search.is_ok() {
            // already exists
        } else {
            let mut new_step: Step = Step::new(prereq);
            steps.insert(prereq_search.unwrap_err(), (prereq, new_step));
        }

        // Add step
        let search: Result<usize, usize> = steps.binary_search_by(|item| item.0.cmp(&step));
        if search.is_ok() {
            steps[search.unwrap()].1.add_prerequisite(prereq);
        } else {
            let mut new_step: Step = Step::new(step);
            new_step.add_prerequisite(prereq);
            steps.insert(search.unwrap_err(), (step, new_step));
        }
    }

    return steps.into_iter().map(|step| step.1).collect();
}
