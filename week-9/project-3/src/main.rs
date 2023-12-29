use std::io::Write;

fn main() {
    let name_of_commisioner = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministry = [
        "        |Internal Affairs",
        "        | Justice",
        "|Defense",
        "        |Power and Steel",
        "        |Petroleum",
    ];

    let Geopolitical_zone = [
        "|South west",
        "|North Eat",
        "|South South",
        "|South West",
        "|South East",
    ];

    let mut file = std::fs::File::create("EFCC.txt").expect("Failed to create");

    file.write_all("\t\tINFORMATION SERVICE DEPARTMENT\n".as_bytes())
        .expect("Failed to write");
    file.write_all("\t\t------------------------\n\n".as_bytes())
        .expect("Failed to write");

    for x in 0..name_of_commisioner.len() {
        file.write_all(name_of_commisioner[x].as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to write");

        file.write_all(ministry[x].as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to write");

        file.write_all(Geopolitical_zone[x].as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to write");
    }

    println!("\nDone");