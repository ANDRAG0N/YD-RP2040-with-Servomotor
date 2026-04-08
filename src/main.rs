#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
// use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;

use vcc_gnd_yd_rp2040::entry;
use vcc_gnd_yd_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac, 
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry] // Начало работы прошивки.
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap(); // Видимо тут мы подключаемся к переферии то есть к vcc_gnd_yd_rp2040 пинам чтобы не подключатся вручную
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC, // Модуль кварца	Включает внешний генератор и усиливает герцовку подключённых модулей чтобы они работали быстрее
        pac.CLOCKS,
        pac.PLL_SYS, // Умножает частоту для системной шины (обычно до 125 MHz)
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )

    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut in1 = pins.gpio0.into_push_pull_output();
    let mut in2 = pins.gpio1.into_push_pull_output();
    let mut in3 = pins.gpio2.into_push_pull_output();
    let mut in4 = pins.gpio3.into_push_pull_output();

    let sequence = [
        [true, false, false, false],
        [true, true, false, false],
        [false, true, false, false],
        [false, true, true, false],
        [false, false, true, false],
        [false, false, true, true],
        [false, false, false, true],
        [true, false, false, true],
    ];

    loop {
        for step in &sequence {
            let _ = in1.set_state(step[0].into());
            let _ = in2.set_state(step[1].into());
            let _ = in3.set_state(step[2].into());
            let _ = in4.set_state(step[3].into());

            delay.delay_ms(1);
        }
    }
}