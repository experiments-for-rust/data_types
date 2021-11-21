use std::io;
use std::io::Write; //flush()

fn main() {
    //scalar
    const INT1:i8=-1_28; //-128~127
    const INT2:u8=255; //0~255
    const INT3:isize=0x00; //based on system architecture
    const FLOAT1:f64=2.0;
    const FLOAT2:f32=3.0;
    let x=2.0; //f64 as default 
    let t=true; //printed as char "true"
    let f:bool = false;
    let a="a";
    let b:char = 65 as char; //cast between types
    println!(
        "{},{},{},{},{},{},{},{},{},{}", 
        INT1,INT2,INT3,FLOAT1,FLOAT2,x,t,f,a,b
    );

    //compound data types
    let touple:(i32,i64,i128)=(114514,19198110,893931);
    let (c,d,e)=touple; //assigned in group
    let g=touple.0; //the value of the first element in touple is assigned to variable f
    let array1:[i32;5]=[1,2,3,4,5];
    let array2=[3;5]; //[3,3,3,3,3]

    println!(
        "{},{},{},{}", 
        c,d,e,g
    );
    for i in 0..5{
        println!("array1[{}]={}",i,array1[i]);
    }
    for i in array2.iter(){
        println!("directly print the value of array array2[]: {}",i);
    }

    //invalid visit of array
    print!("enter an array index for array1[]: ");
    io::stdout().flush().unwrap(); //flush stdout when using print!()
    let mut index = String::new();
    io::stdin().read_line(&mut index)
        .expect("read line failed"); //may cause runtime error when index > 4

    let index:usize=index.trim() //remove leading and trailing whitespaces
        .parse().expect("index entered was not a number");
    let element = array1[index]; //CANNOT use isize for index
    println!(
        "value of the element at index {}: {}",
        index,element
    );
}