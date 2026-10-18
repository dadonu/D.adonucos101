use std::io;

fn main() {
    println!("\nTHE EY INTERVIEW");

    let mut total_candidates_input = String::new();

    println!("\nEnter the number of candidates:");
    io::stdin()
        .read_line(&mut total_candidates_input)
        .expect("Failed to read input");

    let total_candidates: usize = total_candidates_input
        .trim()
        .parse()
        .expect("Please enter a valid whole number");

    let mut candidates: Vec<(String, u32)> = Vec::new();


    for i in 0..total_candidates {
        println!("\nEnter Candidate-{} name:", i + 1);

        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read name");

        let name = name_input.trim();

        println!("Enter years of programming experience:");

        let mut experience_input = String::new();
        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read experience");

        let experience: u32 = experience_input
            .trim()
            .parse()
            .expect("Please enter a valid whole number");

        candidates.push((name.to_string(), experience));
    }

  
    if candidates.len() == 0 {
        println!("No candidates were entered.");
    } else {
        
        let mut highest = candidates[0].1;

     
        for i in 1..candidates.len() {
            if candidates[i].1 > highest {
                highest = candidates[i].1;
            }
        }


        println!("\nCandidate(s) with the highest programming experience:");

        for i in 0..candidates.len() {
            if candidates[i].1 == highest {
                println!("Name: {}", candidates[i].0);
                println!("Years of experience: {}", candidates[i].1);
            }
        }
        println!("Total candidates interviewed: {}", candidates.len());
    }
}

