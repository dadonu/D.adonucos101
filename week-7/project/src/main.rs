use std::io;

fn read_input(prompt: &str) -> f64 {

    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(num) = input.trim().parse::<f64>() {

            if num > 0.0 {
                return num; 
            } 

else {
                println!("Invalid input! Measurement must be greater than zero.");
            }
        } 
else {
            println!("Invalid input! Please type a valid number.");
        }
        
        println!("Please try again.\n");
    }
}

fn calculate_trapezium_area() -> f64 {
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base 1:");
    let base2 = read_input("Enter base 2:");
    
    (height / 2.0) * (base1 + base2)
}

fn calculate_rhombus_area() -> f64 {
    let diagonal1 = read_input("Enter diagonal 1:");
    let diagonal2 = read_input("Enter diagonal 2:");
    
    0.5 * diagonal1 * diagonal2
}

fn calculate_parallelogram_area() -> f64 {
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    
    base * altitude
}

fn calculate_cube_surface_area() -> f64 {
    let side = read_input("Enter side length:");
    
    6.0 * side * side
}

fn calculate_cylinder_volume() -> f64 {
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");
    let pi = 3.14159265359;
    
    pi * radius * radius * height
}
fn main() {
    println!("\n>>>>The Shape Calculator<<<<");
    println!("\nChoose a shape to calculate:");
    println!("\n1 =>Trapezium (Area)");
    println!("2 =>Rhombus (Area)");
    println!("3 =>Parallelogram (Area)");
    println!("4 =>Cube (Surface Area)");
    println!("5 =>Cylinder (Volume)");
    println!("\nEnter choice (1-5):");


    let mut select = String::new();
    io::stdin().read_line(&mut select).expect("Failed to read input");
    
        let select: u32 = select.trim().parse().expect("Please input a valid integer");

  
    if select == 1 {
        let result = calculate_trapezium_area();
        println!("The area of the Trapezium is: {}", result);
    } 
else if select == 2 {
        let result = calculate_rhombus_area();
        println!("The area of the Rhombus is: {}", result);
    } 

else if select == 3 {
        let result = calculate_parallelogram_area();
        println!("The area of the Parallelogram is: {}", result);
    } 

else if select == 4 {
        let result = calculate_cube_surface_area();
        println!("The surface area of the Cube is: {}", result);
    } 
else if select == 5 {
        let result = calculate_cylinder_volume();
        println!("The volume of the Cylinder is: {}", result);
    } 

else {
        println!("Invalid choice! Please restart the program and enter a number between 1 and 5.");
    }

}