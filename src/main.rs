use rand::Rng;
use ordered_float::OrderedFloat;
use rand_distr::{Normal, Distribution};

const CASES: usize = 6;
const DIM1: usize = 8;
const DIM2: usize = 8;
const DEBUG: bool = true;

fn main() {
    //Define matrix of this stupid thing because Rust hates structs
    let mut matrix : [ [ [f64; CASES]; DIM1]; DIM2] = [[[0.0; CASES]; DIM1]; DIM2]; 
    
    data_gen(&mut matrix);

    proto_collapse(&mut matrix);
    
    matrix_print(matrix);

}

//Function to generate probablities for each outcome
fn data_gen(matrix : &mut [ [ [ f64; CASES]; DIM1]; DIM2]) {
    //Make a normal distribution
    let normal = Normal::new(1.0/CASES as f64, 1.0/(CASES as f64).sqrt()).unwrap();
    let mut rng = rand::thread_rng();

    for x in matrix {
        for y in x {
            let mut sum = 0.0;
            for z in y.iter_mut() {
                let value = normal.sample(&mut rng);
                sum += value;
                *z = value;
                if DEBUG {println!("Random value assigned - {}\nSum - {}", z, sum);}
            }
            for z in y.iter_mut() {
                *z = *z/sum;
                if DEBUG {println!("Random value after div - {}", z);}
            }
        }
    }
}


//Function to collapse each individual cell without any influence from neighboring cells
fn proto_collapse(matrix : &mut [ [ [ f64; CASES]; DIM1]; DIM2]) {
    let mut rng = rand::thread_rng();
    //Observe the values and collapse them assuming independence of all cells
    for x in matrix {
        for y in x {
            //Apparently floats can't possibly be compared to one another, thanks Rust
            let max = (0..y.len()).max_by_key(|&i| OrderedFloat(y[i]));
            if DEBUG {println!("Highest element probability is {}",  y[max.unwrap()]);}
            
            //Set every entry to 0.0 before 1.0 assignment to prevent branching
            for elem in y.iter_mut() { *elem = 0.0; }
            if DEBUG {println!("Accessing random y element- {}",  y[rng.gen_range(0..CASES)]);}
            
            //Set max prob to 1.0
            y[max.unwrap()] = 1.0;
            if DEBUG {println!("Highest element probability should now be 1.0 - {}",  y[max.unwrap()]);}

        }
    }
}

//Function to print the matrix
fn matrix_print(matrix : [ [ [ f64; CASES]; DIM1]; DIM2]) {
    //Outer loop to print newlines
    for x in matrix {
        //Inner loop to print the pretty colors
        for y in x {
            match y.iter().position(|z| f64::eq(z, &1.0)) {
                Some(0) => print!("🟥"),
                Some(1) => print!("🟧"),
                Some(2) => print!("🟨"),
                Some(3) => print!("🟩"),
                Some(4) => print!("🟦"),
                Some(5) => print!("🟪"),
                Some(_) => print!("⬜"),
                None => print!("⬛")
            }
        }
        println!();
    }
}