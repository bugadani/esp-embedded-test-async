#![no_std]
#![no_main]

#[embedded_test::tests(executor = esp_hal::embassy::executor::thread::Executor::new())]
#[cfg(test)]
mod tests {
    use defmt_rtt as _;
    use esp_hal as _;

    #[init]
    fn init() -> () {
        defmt::info!("Init testing environment");
    }

    #[test]
    fn example() {
        assert!(true)
    }
}