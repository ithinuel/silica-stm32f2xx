use AdvancedPeripheralBus;

pub enum TimerType {
    Basic,
    GeneralPurpose,
    Advanced
}

pub struct TimerPeripheral<'a> {
    pub timer_type: TimerType,
    pub apb: &'a AdvancedPeripheralBus
}
