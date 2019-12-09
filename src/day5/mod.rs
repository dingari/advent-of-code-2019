use super::intcode::*;

// TODO: Lifetimes of In/Out...
fn run_program<In, Out>(input: &Program, input_fn: &In, output_fn: &Out) -> Program
    where In: Fn() -> i32 + 'static,
          Out: Fn(i32) -> () + 'static
{
    let mut computer = Intcode::new(input.clone(), Some(input_fn), Some(output_fn));

    while !computer.is_halted() {
        computer.cycle();
    }

    computer.program()
}

pub fn run(input_str: &String) {
    println!("\n-- Day 5 --");

    let input: Program = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let print_output = |x: i32| { println!("{}", x); };

    // Part 1
    println!("Part 1");
    let get_input_1 = || -> i32 { 1 };
    assert_eq!(run_program(&vec![1002, 4, 3, 4, 33], &get_input_1, &print_output), vec![1002, 4, 3, 4, 99]);
    assert_eq!(run_program(&vec![1101, 100, -1, 4, 0], &get_input_1, &print_output), vec![1101, 100, -1, 4, 99]);

    run_program(&input, &get_input_1, &print_output);

    // Part 2
    let get_test_input = || -> i32 { 8 };
    let assert_output = |desired_out: i32| { move |out| assert_eq!(out, desired_out) };

    println!("\nPart 2");
    run_program(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &get_test_input, &assert_output(1));
    run_program(&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &get_test_input, &assert_output(0));
    run_program(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &get_test_input, &assert_output(1));
    run_program(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &get_test_input, &assert_output(0));
    run_program(&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], &get_test_input, &assert_output(1));
    run_program(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &get_test_input, &assert_output(1));

    run_program(&input, &|| 5, &print_output);
}
