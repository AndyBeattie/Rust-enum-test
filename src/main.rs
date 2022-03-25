fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let work = IpAddrKind::V6(String::from("::1"));

    let mut day = DayOfWeek::Wednesday(String::from("The 3rd Day Of The Week"));
    let mut day_1 = DayOfWeek::Sunday(String::from("The 7th day Of The Week"), Weekend::Yes);
    let mut day_3 = DayOfWeek::Monday(String::from("The First  Day Of the week"));

    println!("{:?}", day);

    day.print_message();
    day_1.print_message();
    day_3.print_message();
}

enum IpAddrKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum DayOfWeek {
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday(String, Weekend),
    Sunday(String, Weekend),
}
#[derive(Debug)]
enum Weekend {
    Yes,
    No,
}

impl DayOfWeek {
    fn print_message(&mut self) {
        let day_number = DayOfWeek::number_of_day_of_week(self);

        if day_number == 1 {
            println!("Oh POO! It's Monday,");
        }
        if day_number == 2 {
            println!("Oh POO! It's Tuesday")
        }
        if day_number == 3 {
            println!("Oh POO! It's Wednesday")
        }
        if day_number == 4 {
            println!("Oh POO! It's Thursday")
        }
        if day_number == 5 {
            println!("Oh POO! It's Friday")
        }
        if day_number == 6 {
            println!(
                "Oh POO! It's Saturday, {:?}, But At Least It's The Weekend!",
                Weekend::Yes
            )
        }
        if day_number == 7 {
            println!(
                "Oh POO! It's Sunday, {:?}, But At Least It's The Weekend!",
                Weekend::Yes
            )
        }
    }

    fn number_of_day_of_week(day: &mut DayOfWeek) -> u32 {
        match day {
            DayOfWeek::Monday(_) => 1,
            DayOfWeek::Tuesday(_) => 2,
            DayOfWeek::Wednesday(_) => 3,
            DayOfWeek::Thursday(_) => 4,
            DayOfWeek::Friday(_) => 5,
            DayOfWeek::Saturday(_, Weekend::Yes) => 6,
            DayOfWeek::Sunday(_, Weekend::Yes) => 7,
            _ => 0, //this is a special match test, to prevent having to
                    //check for the NO parts of weekend enum, we  could error check against this value
        }
    }
}
