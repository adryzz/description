use description::{Description, OptionalDescription};

#[derive(Description)]
enum ChargerStatus {
    #[description("Charger connected!")]
    Connected,

    #[description("Charger disconnected!")]
    Disconnected,
}


#[derive(OptionalDescription)]
enum BatteryStatus {
    Okay,

    #[description("Low battery!")]
    LowBattery,

    #[description("Fully charged! Please remove the charger.")]
    FullyCharged,

    #[description("Battery overheating!")]
    Overheating,
}

fn main() {
    let charger = ChargerStatus::Connected;

    let battery1 = BatteryStatus::LowBattery;

    let battery2 = BatteryStatus::Okay;

    println!("Charger notification: {}", charger.description());

    if let Some(description) = battery1.description() {
        println!("Battery notification: {}", description);
    }

    if let Some(description) = battery2.description() {
        println!("Battery notification: {}", description);
    }
}