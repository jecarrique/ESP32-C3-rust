# ESP32-C3 Rust `no_std` — Plantilla Codespaces con Wokwi

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/jecarrique/ESP32-C3-rust)

Plantilla lista para usar en **GitHub Codespaces** que implementa un ejemplo completo de firmware Rust bare-metal (`no_std`) para el **ESP32-C3**, con simulación integrada en **Wokwi**.

## Ejemplo: Potenciómetro + Botón + LED

| Componente | Pin ESP32-C3 | Descripción |
|---|---|---|
| Potenciómetro | GPIO4 (ADC1_CH4) | Lectura analógica 0–4095 |
| Botón pulsador | GPIO9 | Entrada digital (pull-up interno) |
| LED rojo | GPIO2 | Salida digital (con resistor 220 Ω) |

**Lógica:** el LED se enciende cuando el botón está presionado o el potenciómetro supera el 50%.

## Inicio rápido

1. Clic en **"Use this template"** → **"Create a new repository"**
2. Clic en **Code** → **Codespaces** → **Create codespace on main**
3. Esperar ~2 min a que el entorno se configure
4. Compilar: `cargo build`
5. Simular: abrir `diagram.json` en VS Code y hacer clic en ▶

## Stack tecnológico

- **Rust stable** + target `riscv32imc-unknown-none-elf`
- **esp-hal 1.1** — HAL no_std para ESP32
- **Wokwi** — simulación en el navegador

## Tutorial paso a paso

Ver [`TUTORIAL.md`](TUTORIAL.md) para la guía completa, incluyendo todos los errores encontrados durante el desarrollo y sus soluciones.
