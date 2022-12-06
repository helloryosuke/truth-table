use std::io::stdin;
use itertools::Itertools;

fn print_truth_table(n: usize) {

    let true_string = "T \t";
    let false_string = "F \t";

    // initialize table headers
    for i in 0..n {
        print!("p_{} \t", i+1);
    }
    println!("biconditional \t m\n");

    // evaluate the combinations of truth values
    for i in 0..n+1 {

        // get all possible combinations i-number of true values
        let patterns = (0..n).combinations(i);

        for pattern in patterns {

            // if the number of true values is 0
            // then the entire row is false
            if pattern.is_empty() {
                let row: String = false_string.repeat(n);
                print!("{}F\t\t", row);
            }
            // otherwise, evaluate the truth value for each cell
            else {

                // store the final truth value of p1 <-> p2 <-> p3... <-> p_n
                let mut truth_value: bool = true;

                // search each proposition value to determine whether true or false
                for p in 0..n {
                    
                    // if the pattern contains sub-p
                    // then print true
                    if pattern.contains(&p) {
                        print!("{}", true_string);
                        // evalute the truth value of p_i <-> p_(i-1)
                        truth_value = truth_value == true;
                    }
                    else {
                        print!("{}", false_string);
                        // evalute the truth value of p_i <-> p_(i-1)
                        truth_value = truth_value == false;
                    }
                }
                // append the truth value to row display
                print!("{}\t\t", if truth_value { "T" } else { "F" });
            }
            // display m (count of true values)
            println!("{}\n", pattern.len());
        }
    }

}

fn main() {
    
    loop {

        // handle input
        let mut input = String::new();
        
        println!("Enter number of propositions (enter non-integer value to quit)");

        stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let size = input.trim();
    
        // if expected input is detected then print truth table
        match size.parse::<usize>() {
            Ok(n) => print_truth_table(n),
            Err(..) => {
                println!("quiting program...");
                break;
            },
        };

    }
    
    

}
