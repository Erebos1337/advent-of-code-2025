use std::collections::VecDeque;

use regex::Regex;
use utils::inputs::read_lines;

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
    variable,
};

pub fn main() {
    let lines = read_lines("./day10/input.txt");

    let mut part1: u64 = 0;
    let mut part2: f64 = 0.0;

    let re = Regex::new(r"^\[(.+)\] \((.+)\)+ \{(.+)\}$").unwrap();

    // parse tile coordinates
    let machines: Box<[(Box<[bool]>, Box<[Box<[usize]>]>, Box<[usize]>)]> = lines
        .map(|line| {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            (
                caps.get(1).unwrap().as_str().to_string(),
                caps.get(2).unwrap().as_str().to_string(),
                caps.get(3).unwrap().as_str().to_string(),
            )
        })
        .map(|(lights, buttons, joltage)| {
            let lights: Box<[bool]> = lights
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid light character"),
                })
                .collect();

            let buttons: Box<[Box<[usize]>]> = buttons
                .split(") (")
                .map(|group| {
                    group
                        .split(',')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Box<[usize]>>()
                })
                .collect();

            let joltage: Box<[usize]> = joltage
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            (lights, buttons, joltage)
        })
        .collect();

    // solve part 1 with breadth-first search
    'startup_loop: for machine in machines.iter() {
        let (lights, buttons, _) = machine;
        let mut queue: VecDeque<(u64, Box<[bool]>)> = VecDeque::new();
        queue.push_back((0, lights.clone()));
        while let Some((steps, state)) = queue.pop_front() {
            for button in buttons {
                let mut new_state = state.clone();
                for &light_idx in button {
                    new_state[light_idx] = !new_state[light_idx];
                }
                if new_state.iter().all(|b| *b == false) {
                    part1 += steps + 1;
                    continue 'startup_loop;
                }
                queue.push_back((steps + 1, new_state));
            }
        }
    }

    // solve part 2 with linear programming
    for (_, buttons, joltage) in machines.iter() {
        // setup linear programming problem
        let mut problem = ProblemVariables::new();

        // create an integer non-negative variable for each button
        let x = problem.add_vector(variable().integer().min(0), buttons.len());

        // create one equation for each joltage requirement
        let mut equations: Vec<Vec<Variable>> = vec![Vec::new(); joltage.len()];

        // assign the buttons' variables to the equations for the targeted joltages
        for (i, targets) in buttons.iter().enumerate() {
            for &light_idx in targets.iter() {
                equations[light_idx].push(x[i]);
            }
        }

        // define objective as the sum of all button presses
        let objective: Expression = x.iter().sum();

        // define each equation to match the required joltage
        let constraints = equations.iter().enumerate().map(|(i, vars)| {
            constraint!(Expression::from(joltage[i] as f64) == vars.iter().sum::<Expression>())
        });

        // solve the linear programming problem
        let solution = problem
            // minimise the sum of button presses
            .minimise(objective)
            .using(default_solver)
            // all contraints must be satisfied
            .with_all(constraints)
            .solve()
            .unwrap();

        // add sum of button presses to part 2 solution
        part2 += x.iter().map(|var| solution.value(*var)).sum::<f64>();
    }

    println!("day  10");
    println!("  - part 1: {}", part1); // 466
    println!("  - part 2: {}", part2); // 17214
}
