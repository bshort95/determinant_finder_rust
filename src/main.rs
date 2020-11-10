use std::io;

fn main() {

    let mut fs = true;    
    let mut input = String::new();

    while fs
    {
        println!("-----------------------------");
        println!("welcome to the derivative finder");
        println!("enter 1 to solve a 2x2 matrix");
        println!("enter 2 to solve a 3x3 matrix");
        println!("enter 3 to quit");
        println!("-----------------------------");

        input.clear();
        io::stdin().read_line(&mut input).expect("wrong format");


        let trimput = input.trim();

        if trimput == "1"
        {

            let mut number1 = String::new();
            let mut arr1:[i32;4] = [0,0,0,0];

            for i in 0..4
            {
                println!("please notice the order of the numbers");
                println!("| 1 2 |");
                println!("| 3 4 |");

                println!("{}:",i+1);
        
                number1.clear();
                io::stdin().read_line(&mut number1).expect("wrong format");
    
                let  tnumber1 = number1.trim();
                let tnumarray: i32 = tnumber1.parse().unwrap_or(0);    
    
                arr1[i] = tnumarray;

            }
        
            let answer = two_by_two(arr1[0], arr1[1], arr1[2], arr1[3]);
   
            println!("the determinant of that matrix is {}",answer)

        }
        else if trimput == "2"
        {

            let mut number2 = String::new();
            let mut arr2:[i32;9] = [0,0,0,0,0,0,0,0,0];
    


            for i in 0..9
            {
    
                println!("please notice the order of the numbers");
                println!("| 1 2 3 |");
                println!("| 4 5 6 |");
                println!("| 7 8 9 |");
    
                println!("{}:",i+1);
                number2.clear();
                io::stdin().read_line(&mut number2).expect("wrong format");
    
                let  tnumber2 = number2.trim();
    
                let tnumarray2: i32 = tnumber2.parse().unwrap_or(0);    
    
                arr2[i] = tnumarray2;

            }


            let answer = three_by_three(arr2[0],arr2[1],arr2[2],arr2[3],arr2[4],arr2[5],arr2[6], arr2[7], arr2[8]);
   
            println!("the determinant of that matrix is {}",answer)

        }
        else if trimput == "3"
        {
            println!("have a nice day");
            fs = false;
        }
        else
        {
            println!("please enter an appropriate answer");
        }

    }


}

// where the mathic happens, this calculates a simple 2x2 determinant 
pub fn two_by_two(a: i32 , b: i32, c:i32, d:i32) -> i32
{
    return(a*d)-(b*c);
}

// a little more complicated but this calculates a 3x3 determinant
pub fn three_by_three(a: i32 , b: i32, c:i32, d:i32, e: i32 , f: i32, g:i32, h:i32, i:i32) ->i32
{
    let part1 = a * two_by_two(e,f,h,i);
    let part2 = (b * -1)* two_by_two(d, f, g, i);
    let part3 = c * two_by_two(d, e, g, h);
    return part1 + part2 + part3;
}

