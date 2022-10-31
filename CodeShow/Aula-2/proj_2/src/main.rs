const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOURS: u32 = 60;
const SECONDS_IN_HOURS: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOURS;

fn main() {
    let total: u32 = 30;
    let total_in_seconds: u32 = total * SECONDS_IN_HOURS;
    //mudança de escopo
    {
        let total: u32 = total + 20;
        let total_in_seconds: u32 = total * SECONDS_IN_HOURS;

        println!("Trabalhou {} horas",total ); // 50
        println!("isso resulta em {} segundos",total_in_seconds);
    }
    println!("Trabalhou {} horas",total ); // 30
    println!("isso resulta em {} segundos",total_in_seconds);
}