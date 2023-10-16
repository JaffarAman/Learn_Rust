fn main() {
   //     let mut x = 10;
//     let y = 6;
//     println!("x = {} and y = {} are two numbers" , x , y);
   
//   x = x + y;
//   println!("the result value is {}" , x);
 
//     let mul =  run(2 , 5);
//     println!("mul value: {}" , mul );
// }

// fn run(x:i32 , y:i32)->i32{
//         println!("x={} , y={}" , x , y);
//         // return x * y;
//         x * y
// }


fn main(){
    
    //integer
    // let num:u8 = 260;  error
    // let num :u8 = -100 error
    let num:u8 = 200;
    println!("num {}" , num);
    
    //float
    let floatnum:f32 = 250.11;
    println!("floatNum {}" , floatnum);
    
    //char
    // let name = "jaffar";
    // let name = 'h'
    // println!("name {}"  , name)
    
    //Boolean
    let a:bool = true;
    println!("a {}"  ,a);
    
    //compound types
    // Array
    let mut arr = [1,2,3,4,5];
    println!("arr {}" , arr[1]);
    arr[4] = 100;
    println!("{}",arr[4]);
    
    // Tupple's
    let tup = (5 , "jaffar" , true);
    println!("tupple {} " , tup.1);
       
       
    //  IF else  condition 
    let x:u8 = 10;
    
    if x==10{
        println!("x is 10")
    } else{
        println!("x is not equal 10")
    }
}
}
