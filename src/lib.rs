#![no_std]

pub use description_macro::*;
/// Like [`Display`], but [`'static`], no_std and no_alloc.
/// 
/// See also [`OptionalDescription`]
/// 
/// # Example
///
/// ```rust
/// # use description::Description;
///
/// #[derive(Description)]
/// enum ChargerStatus {
///    #[description("Charger connected!")]
///    Connected,
///
///    #[description("Charger disconnected!")]
///    Disconnected,
/// }
/// 
/// fn main() {
///     let charger = ChargerStatus::Connected;
/// 
///     println!("Charger notification: {}", charger.description());
///     // Charger notification: Charger connected!
/// }
/// ```
pub trait Description {
    fn description(&self) -> &'static str;
}

/// Like [`Display`], but [`'static`], no_std and no_alloc.
/// 
/// See also [`Description`]
/// 
/// # Example
///
/// ```rust
/// #[derive(OptionalDescription)]
/// enum BatteryStatus {
///     Okay,
/// 
///     #[description("Low battery!")]
///     LowBattery,
/// 
///     #[description("Fully charged! Please remove the charger.")]
///     FullyCharged,
/// }
/// 
/// fn main() {
///     let battery1 = BatteryStatus::LowBattery;
/// 
///     let battery2 = BatteryStatus::Okay;
/// 
///     if let Some(description) = battery1.description() {
///         println!("Battery notification: {}", description);
///     }
/// 
///     if let Some(description) = battery2.description() {
///         println!("Battery notification: {}", description);
///     }
/// 
///     // Battery notification: Low battery!
///     //
/// }
///```
pub trait OptionalDescription {
    fn description(&self) -> Option<&'static str>;
}