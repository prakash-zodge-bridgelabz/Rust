pub fn useLoop(){
    let mut count = 1;
    loop{
        if(count>10){
            break;
        }else{
            println!("count = {}",count);
        }
        count+=1;
    }
}
pub fn useNestedLoops(){
    'outer: loop{
        println!("Entered outer loop");
        'inner: loop{
            println!("Entered inner loop");
            //break 'inner;     --> will break from inner loop only
            break 'outer;   //--> this will break from outer loop
        }
        println!("This point will never be reached");       // in visual studio it shows unreachable code
    }
    println!("Exited outer loop")
}