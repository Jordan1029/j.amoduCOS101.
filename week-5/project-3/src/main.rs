use std::io;

fn main() {
    let input = io::stdin();

    let mut x:f32 = 0.00;

    println!("Da_Cybercafe");

    println!("P = Poundo Yam/Edikainko Soup - ₦3,200");

    println!("F = Fried RIce and Chicken    - ₦3,000");

    println!("A = Amala and Ewedu soup      - ₦2,500");

    println!("E = Eba & Egusi               - ₦2,000");

    println!("W = White rice and stew       - ₦2,500");


    println!("Type O to place order");
    loop{
        let mut chow = String::new();

        input.read_line(&mut chow).expect("Invalid Input");

        let chow = chow.trim();

        if chow == "P"{
            x += 3200.0;
        }else if chow == "F"{
            x += 3000.0;
        }else if chow == "A"{
            x += 2500.0; 
        }else if chow == "E"{
            x += 2000.0;
        }else if chow == "W"{
            x += 2500.0;
        }else if chow == "O"{
            break;
        }else {
            println!("Type what we have and in CAPS");

            continue;
        }
    }

    println!("theres promo because you spent: {}",x);

    if x > 10_000.0 {
        let discount:f32 = x * 0.95;

        println!("Your bill is now {}",discount);
    } else {
        println!("Your bill is: N{}",x );
    }
}
