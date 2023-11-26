use std::io;

fn main() {
    let mut input1 = String::new();
    println!("How many siblings do you have");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let sib_num:u16 = input1.trim().parse().expect("Invalid input");

    for x in 1..=sib_num{

        let mut input2 =String::new();
        let mut input3 =String::new();
        let mut input4 =String::new();
        let mut input5 =String::new();
        let mut input6 =String::new();
        let mut input7 =String::new();
        let mut input8 =String::new();
        let mut input9 =String::new();
        let mut input10 =String::new();
        let mut input11 =String::new();
        let mut input12 =String::new();

            println!("\nSibling no.{}",x);

            println!("\nEnter sibling name");
            io::stdin().read_line(&mut input2).expect("Failed to read input");
            let sib_name = input2.trim();

            println!("How old is {}",sib_name);
            io::stdin().read_line(&mut input3).expect("Failed to read input");
            let sib_age:u16 = input3.trim().parse().expect("Invalid input");

            if sib_age > 18{
                println!("What is {}'s marital status(single/married)",sib_name);
                io::stdin().read_line(&mut input4).expect("Failed to read input");
                let marr_sta = input4.trim().to_uppercase();

                if marr_sta =="SINGLE"{
                println!(" Is {}'s a worker or student(worker/student)",sib_name);
                io::stdin().read_line(&mut input5).expect("Failed to read input");
                let sib_sta = input5.trim().to_uppercase();

                if sib_sta=="STUDENT" {
                println!("What University does {}'s attend",sib_name);
                io::stdin().read_line(&mut input6).expect("Failed to read input");
                let sib_sch1 = input6.trim().to_uppercase();

                 println!("What course is {}'s studying",sib_name);
                io::stdin().read_line(&mut input7).expect("Failed to read input");
                let sib_course = input7.trim().to_uppercase();

                
            }

    }

    else if marr_sta == "MARRIED"{
        println!("How many children does {} have",sib_name);
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let sib_children:u16 = input8.trim().parse().expect("Invalid input");

        println!("What city does {}'s family live in", sib_name);
        io::stdin().read_line(&mut input9).expect("Failed to read input");
        let sib_reside = input9.trim().to_uppercase();
    }
}
    else if sib_age <= 18{
        println!("Has {} written waec examination(yes/no)",sib_name);
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let sib_waec = input10.trim().to_lowercase();
        
        if sib_waec == "yes"{
        println!("What Secondary school did {}'s attend", sib_name);
        io::stdin().read_line(&mut input11).expect("Failed to read input");
        let sib_reside = input11.trim().to_uppercase();
    }
else if sib_waec =="no"{
    println!("What is {}'s current level in school",sib_name);
        io::stdin().read_line(&mut input12).expect("Failed to read input");
        let sib_children:u16 = input12.trim().parse().expect("Invalid input");
}
}
}
}