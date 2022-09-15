/********************
Name: Abi Kunkle
Date: 9/14/2022
Assignment: Program 0
Compute the volume of a sphere.
The mathematical formula, where R represents the radius of the
sphere, is V = 4/3piR^3. The volume is to be computed in both cubic-cm and cubic-inches.  The user will provide the length of the
radius in centimeters. This will require you to perform a conversion from cm to inches.  There are 2.54
centimeters per inch.
 *********************/

use std::io;
use std::f64::consts::PI; // pi

const CM_PER_INCH: f64 = 2.54; // constant for centimeters per inch.

/*
    main()
    Simply calls prints statement and then calls the calculation loop.
 */
fn main() {
    println!("Welcome to the volume calculator. You may enter quit at the prompt at any time to exit the program. ");
    calculation_loop();     //perform calculation on a loop. breaks when user enters "quit"
}

/*
    calculation_loop()
    Takes in user input and loops until user enters "quit". On each loop, it will take the user
    input (radius_cm) and feed it to the function compute_vol(). This returns a tuple with
    the volume in inches and centimeters.
 */
fn calculation_loop() {
    loop {          // Rust has a loop statement that executes until there is a break statement.

        // Read  in the user input.
        println!("Enter the radius of your sphere (in cm): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Valid input is integer or 'quit'");

        // Check input and exit program if user entered 'quit
        if input.trim() == "quit" {
            break;
        }

        // variable radius(cm) is the input as a float f64
        let radius_cm: f64 = input.trim().parse().unwrap();
        // compute volume. takes in radius and returns a tuple with vol inches and cm
        let (volume_inches, volume_cm) = compute_vol(radius_cm);

        // print output statement
        print!("For a sphere with a radius of {radius_cm}\nThe volume is: {volume_cm} cm-cubed,\n or {volume_inches} inches-cubed\n");

    }
}


/*
    compute(vol) is called in the calculation_loop. It takes in the radius in centimeters,
    converts it to inches, and performs the calculations on both. Returns a tuple with the
    volume in inches and centimeters. I know I probably could cut down on some repeated code here.
 */
fn compute_vol(radius_cm: f64) -> (String, String){
    // get radius as inch
    let radius_in: f64 = radius_cm / CM_PER_INCH;

    // perform R^3 for both inches and cm
    let radius_cubed_in: f64 = f64::powf(radius_in, 3.0);
    let radius_cubed_cm: f64 = f64::powf(radius_cm, 3.0);

   // calculate volume in inches and cm. Format to .3 so it matches the format of the example output
    let volume_in: String = format!("{:.3}", (4.0 / 3.0) * PI * radius_cubed_in as f64);
    let volume_cm: String = format!("{:.3}", (4.0 / 3.0) * PI * radius_cubed_cm as f64);

    return (volume_in, volume_cm);      // return as tuple
}
