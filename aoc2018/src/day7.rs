use utils;
use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 7).unwrap();

    match part {
        Part::One => part_one(input),
        Part::Two => part_two(input).to_string(),
    }

}

fn part_two(input:String) -> i32 {

    let mut steps_map = HashMap::new();

    for line in input.lines() {
        let step = Step::from(line);

        let mut entry = steps_map.entry(step.label).or_insert(step);

        entry.prereqs.insert(Step::from(line).prereqs.iter().cloned().next().unwrap());
    }

    let step_labels:HashSet<char> = steps_map.iter()
        .flat_map(|(_, step)| step.prereqs.iter().cloned())
        .collect();

    for label in step_labels {
        steps_map.entry(label).or_insert(Step{label, prereqs:HashSet::new()});
    }


    let mut workers = [Worker::new(); 5];
    let mut timer = 0;

    loop {
        print!("{:3}: ", timer);
        for worker in workers.iter_mut() {
            if !worker.is_free {
                worker.time_step();
            }

            if worker.is_free {
                for (_, reqs) in steps_map.iter_mut() {
                    reqs.prereqs.remove(&worker.task_label);
                }
            }
            //print!("| {} ", worker);
        }
        //println!();

        let mut available_steps:Vec<char> = steps_map
            .iter()
            .filter_map(|(label, step)| if step.prereqs.len() == 0 {Some(*label)} else {None})
            .collect();

        available_steps.sort();

        for (step, worker) in available_steps
            .iter()
            .zip(workers.iter_mut().filter(|w| w.is_free)) {
            worker.assign_task(step);

            steps_map.remove(step);
            //for (_, reqs) in steps_map.iter_mut() {
                //reqs.prereqs.remove(&step);
            //}
        }

        println!("Available Steps: {:?}", available_steps);

        for worker in workers.iter() {
            print!("| {} ", worker);
        }
        println!();

        if workers.iter().all(|worker| worker.is_free) {
            break;
        }


        timer += 1;
    }



    timer
}


#[derive(Debug, Copy, Clone)]
struct Worker {
    task_label:char,
    time_left:usize,
    is_free:bool,
}

impl std::fmt::Display for Worker {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " {}:{:2} ", self.task_label, self.time_left)
    }
}

impl Worker {
    fn new() -> Worker {
        Worker {time_left:0, is_free:true, task_label:'.'}
    }

    fn assign_task(&mut self, task:&char) {
        if !self.is_free {panic!();}

        static CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
            'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
            'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

        self.time_left = CHARS.iter().position(|c| c == task).unwrap() + 61;
        self.is_free = false;
        self.task_label = task.clone();
    }

    fn time_step(&mut self) {
        self.time_left -= 1;
        if self.time_left == 0 {
            self.is_free = true;
        }
    }
}


fn part_one(input:String) -> String {

    let mut steps_map = HashMap::new();

    for line in input.lines() {
        let step = Step::from(line);

        let mut entry = steps_map.entry(step.label).or_insert(step);

        entry.prereqs.insert(Step::from(line).prereqs.iter().cloned().next().unwrap());
    }

    let step_labels:HashSet<char> = steps_map.iter()
        .flat_map(|(_, step)| step.prereqs.iter().cloned())
        .collect();

    for label in step_labels {
        steps_map.entry(label).or_insert(Step{label, prereqs:HashSet::new()});
    }

    let mut result = String::new();

    loop {
        let mut available_steps:Vec<char> = steps_map
            .iter()
            .filter_map(|(label, step)| if step.prereqs.len() == 0 {Some(*label)} else {None})
            .collect();

        if available_steps.len() == 0 {
            break;
        }

        available_steps.sort();

        result += &available_steps[0].to_string();

        steps_map.remove(&available_steps[0]);
        for (_, reqs) in steps_map.iter_mut() {
            reqs.prereqs.remove(&available_steps[0]);
        }
    }

    result
}

#[derive(Debug )]
struct Step {
    label:char,
    prereqs:HashSet<char>,
}

impl Step {
    fn from(input:&str) -> Step {
        let tokens:Vec<&str> = input.split_whitespace().collect();

        Step {
            label:tokens[7].parse().unwrap(),
            prereqs:{
                let mut set = HashSet::new();
                set.insert(tokens[1].parse::<char>().unwrap());
                set
            }
        }
    }


}
