fn main() {
    ///////////////////////////////////////////////////
    //Variables////////////////////////////////////////
    ///////////////////////////////////////////////////
    
    //If we don't use mut keyword, the value of the variable cannot be reassigned
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants. They are valid throughout the entire program and the intended scope only
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing. A variable can be shadowed. It will not happen if we do not use the let keyword. 
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");

    let spaces = "    "; // Shadowing can even change the data type
    let spaces = spaces.len();
    println!("Spaces: {spaces}"); 

    ///////////////////////////////////////////////////
    //Data types///////////////////////////////////////
    ///////////////////////////////////////////////////
    
    

}
