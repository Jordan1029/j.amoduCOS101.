    use std::io;
    fn main(){
        let mut count = 0;
        while count <=150{
            let mut input1 = String::new();
            let mut input2 = String::new();
            let mut input3 = String::new();
            let mut input4 = String::new();

            println!("\n Enter your name");
            io::stdin().read_line(&mut input1).expect("Not a valid string");

            println!("\n Enter your email address");
            io::stdin().read_line(&mut input2).expect("Not a valid string");

            println!("\n Enter your State of Origin");
            io::stdin().read_line(&mut input3).expect("Not a valid string");

            println!("\n Enter your Department");
            io::stdin().read_line(&mut input4).expect("Not a valid string");

            let mut class_rep = String::new();
            println!("Are you a current Class Rep?\nEnter 1 for yes and 2 for no");
            io::stdin().read_line(&mut class_rep).expect("Failed to read input");
            let class_rep:i32 = class_rep.trim().parse().expect("Not a valid integer");

            let mut level = String::new();
            println!("Are you in 100 level?\nEnter 1 for yes and 2 for no");
            io::stdin().read_line(&mut level).expect("Failed to read input");
            let level:i32 = level.trim().parse().expect("Not a valid integer");

            let mut cgpa = String::new();
            println!("Is your CGPA above 4.0?\nEnter 1 for yes and 2 for no");
            io::stdin().read_line(&mut cgpa).expect("Failed to read input");
            let cgpa:i32 = cgpa.trim().parse().expect("Not a valid integer");

            if (class_rep== 1) && (level == 2) && (cgpa==1){
                println!("You can vote!");
                }
                
                else{
                    println!("Sorry, you are not eligible to vote");
                }
                count+=1
            }
    }





  
