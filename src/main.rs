//! # ESP32-C3 Demo: Potenciómetro, Botón y LED
//!
//! Ejemplo en Rust `no_std` para ESP32-C3 que demuestra:
//! - **ADC**: Lectura analógica de un potenciómetro en GPIO4
//! - **GPIO entrada**: Lectura de un botón pulsador en GPIO9 (pull-up interno)
//! - **GPIO salida**: Control de un LED en GPIO2
//!
//! ## Lógica de control
//! El LED se enciende cuando:
//! - El botón está presionado (GPIO9 LOW), O
//! - El potenciómetro supera el 50% del recorrido (valor ADC > 2048)
//!
//! ## Conexiones del circuito
//! ```
//! ESP32-C3      Componente
//! ─────────     ──────────
//! GPIO4    ──── Potenciómetro (pin SIG/wiper)
//! 3V3      ──── Potenciómetro (extremo VCC)
//! GND      ──── Potenciómetro (extremo GND)
//!
//! GPIO9    ──── Botón pulsador (un terminal)
//! GND      ──── Botón pulsador (otro terminal)
//! (pull-up interno activado en el código)
//!
//! GPIO2    ──── Resistor 220Ω ──── Ánodo LED
//!               GND          ──── Cátodo LED
//! ```

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    peripherals::ADC1,
};
use esp_println::println;

#[main]
fn main() -> ! {
    // Inicializar el sistema (reloj, watchdogs, etc.)
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // ── LED ─────────────────────────────────────────────────────────────────
    // GPIO2 como salida digital, nivel inicial LOW (apagado)
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    // ── BOTÓN ────────────────────────────────────────────────────────────────
    // GPIO9 como entrada con pull-up interno
    // Cuando se presiona el botón (conectado a GND) el pin lee LOW
    let button_config = InputConfig::default().with_pull(Pull::Up);
    let button = Input::new(peripherals.GPIO9, button_config);

    // ── POTENCIÓMETRO (ADC) ──────────────────────────────────────────────────
    // GPIO4 = ADC1_CH4, atenuación 11dB para rango 0..~3.1V
    let mut adc1_config: AdcConfig<ADC1> = AdcConfig::new();
    let mut adc_pin = adc1_config.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    // ── DELAY ────────────────────────────────────────────────────────────────
    let delay = Delay::new();

    // Mensajes de inicio por puerto serie (USB/UART)
    println!("=== ESP32-C3 Demo: Pot + Botón + LED ===");
    println!("GPIO4 -> Potenciómetro (ADC1)");
    println!("GPIO9 -> Botón (pull-up, activo en LOW)");
    println!("GPIO2 -> LED (con resistor 220 ohm)");
    println!("=========================================");

    loop {
        // Leer valor analógico del potenciómetro (resolución 12 bits: 0–4095)
        let adc_value: u16 = nb::block!(adc1.read_oneshot(&mut adc_pin)).unwrap_or(0);

        // Leer estado del botón: LOW = presionado (pull-up activo)
        let button_pressed = button.is_low();

        // Decidir estado del LED
        let led_on = button_pressed || adc_value > 2048;

        // Aplicar estado al LED
        if led_on {
            led.set_high();
        } else {
            led.set_low();
        }

        // Imprimir valores en el monitor serie cada 200 ms
        println!(
            "Pot: {:4}/4095  Botón: {:10}  LED: {}",
            adc_value,
            if button_pressed { "PRESIONADO" } else { "libre" },
            if led_on { "ON " } else { "OFF" },
        );

        delay.delay_millis(200);
    }
}
