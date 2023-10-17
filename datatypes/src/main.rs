fn main() {
    //scalar Data Types
    let x=2;
    let y: i32 = 2;
    //interger data type include
    //i8,i16,i32,i64,i128
    let a: u32=9;
    //u unsigned 
    //i signed
    let b: f32 =10.34;
    //two types floating point 
    //f32 single presision
    //f64 double pression
    let c: bool= false;
    // boolean value true,false
    //0,1 can also be used
    let d: char ='s';
    //single charater
  
    //Compond Data Types
    //tuple
    let tup: (i32,bool,char) = (1,true,'s');
    //imutable
    println!("{}",tup.1);
    //cant print tup with diff datatype togther
    //print tuple by tuple with index name
    let mut tup2: (i32,bool,char)= (23,false,'g');
    println!("{}",tup.0);
    tup2.0=10;
    println!("{}",tup.0);
    let arr =[1,2,3,4];
    arr[2]=45;
    println!("{}",arr[2]);
    //type change in array data type with number of elements
    let mut arry2: [i128 : 6]=[1,23,43,65,22,42,34];
    println!("{}",arr2[2]);
    let f: u32 =45;
    let g=f;
    println!=("{},{}",g,f);
    
}
