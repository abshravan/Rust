fn main() {
    let x =4; //declare x as 4 implsic type(can't cange value trougout program)
    let y: u32 =5; //declare y as 5 unsigned 32 bit integer
    println!("x is: {}",x); //print x inside the curly bracket
    println!("y is: {}",y);
    let mut z =10; //declare z as 10 and the variable is mutable 
    println!("z is: {} ",z);
    z=100;
    //variable in a diffrent scope (name shadowing)
    {
        let z = 35;
        println!("z is {}",z) 
    }
    println!("z is {}",z);
    //name shadowing from inner scope and using it in outer scope
    {
        z =z-4;
        println!("z is {}",z);
            }
    z=z+1;
    println!("z is {} ",z);
    //changing data types of a variable
    let  p=53;
    println!("p is {}",p);
    let  p="print";
    println!("p is {}",p);
    //constats(value and type can't be changed throughout the program)
    const HOUR: u32 =60;
    //constant name must be full capital letter and must contain a datatype while declare  
    println!("constant is {}",HOUR);
     
}