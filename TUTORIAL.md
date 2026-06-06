# Tutorial: Plantilla ESP32-C3 con Rust `no_std` y Simulación en Wokwi

> **Para docentes:** Esta guía documenta **paso a paso** todo el proceso de creación de esta plantilla, incluyendo cada error encontrado y su solución. Está diseñada para ser replicada en clase.

---

## Tabla de Contenidos

1. [Descripción del proyecto](#1-descripción-del-proyecto)
2. [Arquitectura y herramientas](#2-arquitectura-y-herramientas)
3. [Prerrequisitos](#3-prerrequisitos)
4. [Uso de la plantilla (camino rápido)](#4-uso-de-la-plantilla-camino-rápido)
5. [Creación desde cero (camino largo – para clase)](#5-creación-desde-cero-camino-largo--para-clase)
6. [Estructura del proyecto explicada](#6-estructura-del-proyecto-explicada)
7. [Código fuente explicado](#7-código-fuente-explicado)
8. [Errores encontrados y soluciones](#8-errores-encontrados-y-soluciones)
9. [Simulación con Wokwi](#9-simulación-con-wokwi)
10. [Grabar en hardware real](#10-grabar-en-hardware-real)
11. [Ejercicios propuestos](#11-ejercicios-propuestos)

---

## 1. Descripción del proyecto

Este proyecto es una **plantilla de GitHub Codespaces** para desarrollar firmware en Rust bare-metal (`no_std`) para el microcontrolador **ESP32-C3** de Espressif.

### ¿Qué hace el firmware de ejemplo?

El firmware lee tres periféricos y muestra su estado por el monitor serie cada 200 ms:

| Periférico | Pin | Función |
|---|---|---|
| Potenciómetro | GPIO4 (ADC1_CH4) | Lectura analógica 0–4095 |
| Botón pulsador | GPIO9 (pull-up interno) | Entrada digital activa en LOW |
| LED rojo | GPIO2 + resistor 220 Ω | Salida digital |

**Lógica de control del LED:**
```
LED = ON  si  botón_presionado  OR  potenciómetro > 50 % (>2048/4095)
LED = OFF en caso contrario
```

### ¿Por qué Rust `no_std`?

- Sin sistema operativo (bare-metal)
- Sin heap por defecto (sin `alloc`)
- Control total del hardware
- Binarios muy pequeños (~50 KB)
- Ideal para introducir programación de sistemas embebidos

---

## 2. Arquitectura y herramientas

```
┌─────────────────────────────────────────────────────────┐
│                  GitHub Codespaces                       │
│                                                          │
│  ┌─────────────┐   cargo build   ┌──────────────────┐   │
│  │  src/main.rs│ ──────────────► │  ELF firmware     │   │
│  │  (no_std)   │                 │  target/riscv*/   │   │
│  └─────────────┘                 └────────┬─────────┘   │
│                                           │              │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Extensión Wokwi (VS Code)                        │   │
│  │  ┌──────────────┐   ┌───────────────────────┐    │   │
│  │  │ diagram.json │   │ Simulación ESP32-C3   │    │   │
│  │  │ wokwi.toml   │──►│ + ADC + GPIO + Serie  │    │   │
│  │  └──────────────┘   └───────────────────────┘    │   │
│  └──────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Crates (bibliotecas) utilizadas

| Crate | Versión | Rol |
|---|---|---|
| `esp-hal` | 1.1 | HAL (Hardware Abstraction Layer) para ESP32-C3 |
| `esp-backtrace` | 0.19 | Manejador de pánico y excepciones |
| `esp-println` | 0.17 | `println!` por puerto serie (USB/UART) |
| `nb` | 1.1 | Primitivas non-blocking para ADC |

---

## 3. Prerrequisitos

### Para usar el Codespace (recomendado para clase)

- Cuenta de GitHub
- Navegador web (Chrome, Firefox, Edge)
- Licencia de la extensión Wokwi para VS Code (gratuita para educación: https://wokwi.com/pricing)

### Para desarrollo local

- Rust: https://rustup.rs
- Target RISC-V: `rustup target add riscv32imc-unknown-none-elf`
- `espflash` (para hardware real): `cargo install espflash`
- Extensión Wokwi para VS Code

---

## 4. Uso de la plantilla (camino rápido)

### Paso 1: Crear repositorio desde esta plantilla

1. Ir a https://github.com/jecarrique/ESP32-C3-rust
2. Clic en **"Use this template"** → **"Create a new repository"**
3. Nombrar el repositorio y crearlo

### Paso 2: Abrir en GitHub Codespaces

1. En el nuevo repositorio, clic en **Code** → **Codespaces** → **Create codespace on main**
2. Esperar ~2 minutos mientras se construye el contenedor
3. El script `.devcontainer/setup.sh` se ejecuta automáticamente:
   - Actualiza el toolchain de Rust
   - Instala el target `riscv32imc-unknown-none-elf`
   - Instala `espflash` (opcional, para hardware real)

### Paso 3: Compilar el proyecto

Abrir la terminal integrada y ejecutar:

```bash
cargo build
```

✅ Esperado: `Finished 'dev' profile [optimized + debuginfo] target(s)`

### Paso 4: Simular en Wokwi

1. Abrir el archivo `diagram.json` en VS Code
2. La extensión Wokwi detecta el archivo y ofrece **"Start Simulation"**
3. Clic en ▶ para iniciar
4. El monitor serie muestra los valores en tiempo real
5. Interactuar con el potenciómetro y el botón en la simulación

---

## 5. Creación desde cero (camino largo – para clase)

Esta sección documenta **cada comando ejecutado** para crear el proyecto, incluyendo los errores encontrados.

### 5.1 Instalar Rust

```bash
# Instalar rustup (gestor de versiones de Rust)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Recargar el entorno
source ~/.cargo/env

# Verificar
rustc --version   # rustc 1.96.0 (o similar)
cargo --version   # cargo 1.96.0
```

### 5.2 Instalar el target para ESP32-C3

El ESP32-C3 usa arquitectura **RISC-V 32-bit**, por lo que necesitamos el target `riscv32imc-unknown-none-elf`:

```bash
rustup target add riscv32imc-unknown-none-elf
```

> **¿Por qué este target?**
> - `riscv32` = arquitectura RISC-V de 32 bits
> - `imc` = extensiones Integer + Multiplication + Compressed instructions
> - `unknown` = sin OS conocido
> - `none` = sin sistema operativo
> - `elf` = formato de binario ELF

### 5.3 Crear el proyecto Cargo

```bash
cargo new esp32c3-demo
cd esp32c3-demo
```

### 5.4 Configurar el toolchain (rust-toolchain.toml)

Crear el archivo `rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
targets = ["riscv32imc-unknown-none-elf"]
components = ["rust-src", "rustfmt", "clippy"]
```

### 5.5 Configurar Cargo para cross-compilación (.cargo/config.toml)

Crear `.cargo/config.toml`:

```toml
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor"
rustflags = [
  "-C", "link-arg=-Tlinkall.x",
  "-C", "force-frame-pointers",
]

[build]
target = "riscv32imc-unknown-none-elf"

[env]
ESP_LOG = "info"
```

> **Explicación de los flags:**
> - `-Tlinkall.x`: Script de enlazado de `esp-hal` que configura el mapa de memoria del ESP32-C3
> - `force-frame-pointers`: Necesario para que `esp-backtrace` pueda mostrar el stack trace

### 5.6 Configurar las dependencias (Cargo.toml)

```toml
[package]
name = "esp32c3-demo"
version = "0.1.0"
edition = "2021"
resolver = "2"

[dependencies]
esp-hal       = { version = "1.1", features = ["esp32c3", "unstable"] }
esp-backtrace = { version = "0.19", features = ["esp32c3", "panic-handler", "println"] }
esp-println   = { version = "0.17", features = ["esp32c3", "log-04"] }
nb            = "1.1"

[profile.dev]
opt-level = "s"
debug = true

[profile.release]
opt-level = "s"
debug = true
debug-assertions = true
lto = "fat"
codegen-units = 1
```

> **⚠️ Nota sobre `features = ["unstable"]` en `esp-hal`:**
> El módulo ADC está marcado como `unstable` en `esp-hal 1.x` porque su API aún puede cambiar en futuras versiones. Es necesario activar este feature para usar el ADC.

### 5.7 Escribir el código fuente

Ver sección [Código fuente explicado](#7-código-fuente-explicado).

### 5.8 Primer intento de compilación – Errores encontrados

Ver sección [Errores encontrados y soluciones](#8-errores-encontrados-y-soluciones).

---

## 6. Estructura del proyecto explicada

```
ESP32-C3-rust/
│
├── .cargo/
│   └── config.toml          # Target RISC-V + flags del enlazador
│
├── .devcontainer/
│   ├── devcontainer.json     # Configuración del Codespace (imagen, extensiones)
│   └── setup.sh             # Script de instalación automática
│
├── src/
│   └── main.rs              # Código fuente principal (no_std)
│
├── .gitignore               # Excluir /target/ del repositorio
├── Cargo.toml               # Dependencias y metadatos del proyecto
├── Cargo.lock               # Versiones exactas (reproducibilidad)
├── diagram.json             # Circuito Wokwi
├── rust-toolchain.toml      # Versión y target de Rust
├── TUTORIAL.md              # Este archivo
└── wokwi.toml               # Configuración de la simulación Wokwi
```

---

## 7. Código fuente explicado

### Cabecera `no_std` / `no_main`

```rust
#![no_std]   // No usar la biblioteca estándar de Rust
#![no_main]  // No usar el punto de entrada estándar de Rust
```

En un sistema embebido no hay sistema operativo que llame a `main()`. Estas directivas le indican al compilador que nosotros proporcionamos el punto de entrada y el manejador de errores.

### Importaciones clave

```rust
use esp_backtrace as _;  // Manejador de pánico (no se usa directamente, solo se registra)
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},  // ADC (Convertidor Analógico-Digital)
    delay::Delay,                                  // Retardos bloqueantes
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},  // GPIO
    main,                                          // Macro de punto de entrada
    peripherals::ADC1,                             // Tipo del periférico ADC1
};
use esp_println::println;  // println! que usa el puerto serie USB/UART
```

### Punto de entrada

```rust
#[main]
fn main() -> ! {
    // `-> !` significa que la función nunca retorna (loop infinito)
```

### Inicialización del sistema

```rust
let peripherals = esp_hal::init(esp_hal::Config::default());
```

Esta llamada inicializa el reloj del sistema, deshabilita los watchdog timers y devuelve un struct con todos los periféricos disponibles.

### Configuración del LED (GPIO salida)

```rust
let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
```

- `peripherals.GPIO2`: toma posesión del pin GPIO2
- `Level::Low`: nivel inicial (LED apagado)
- `OutputConfig::default()`: configuración por defecto (push-pull, sin pull resistors)

### Configuración del botón (GPIO entrada con pull-up)

```rust
let button_config = InputConfig::default().with_pull(Pull::Up);
let button = Input::new(peripherals.GPIO9, button_config);
```

El **pull-up interno** mantiene el pin en HIGH cuando el botón no está presionado. Al presionar el botón (que conecta el pin a GND), el pin lee LOW. Esto se llama **lógica activa en bajo**.

### Configuración del ADC

```rust
let mut adc1_config: AdcConfig<ADC1> = AdcConfig::new();
let mut adc_pin = adc1_config.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
```

- `AdcConfig<ADC1>`: configuración para ADC1 (ESP32-C3 solo tiene ADC1)
- `Attenuation::_11dB`: atenuación de ~11 dB, permite medir voltajes hasta ~3.1 V
- `enable_pin(GPIO4, ...)`: configura GPIO4 como entrada analógica del canal 4

**Tabla de atenuaciones disponibles:**

| Atenuación | Rango de voltaje |
|---|---|
| `_0dB` | 0 – ~750 mV |
| `_2p5dB` | 0 – ~1.05 V |
| `_6dB` | 0 – ~1.3 V |
| `_11dB` | 0 – ~3.1 V (recomendado para 3.3V) |

### Lectura del ADC (bloqueante)

```rust
let adc_value: u16 = nb::block!(adc1.read_oneshot(&mut adc_pin)).unwrap_or(0);
```

- `read_oneshot` retorna `nb::Result`, que puede ser `WouldBlock` (conversión en curso) o `Ok(valor)`
- `nb::block!` espera bloqueando hasta que la conversión termina
- `unwrap_or(0)` retorna 0 en caso de error (en lugar de provocar un pánico)
- El resultado es un valor de 12 bits: **0 a 4095**

### Lectura del botón

```rust
let button_pressed = button.is_low();
```

`is_low()` retorna `true` cuando el pin está en LOW, es decir, cuando el botón está **presionado**.

---

## 8. Errores encontrados y soluciones

Durante el desarrollo de esta plantilla se encontraron los siguientes errores. Se documentan aquí para que los estudiantes puedan reconocerlos y resolverlos.

---

### ❌ Error 1: Feature `exception-handler` inexistente

**Comando ejecutado:**
```bash
cargo build
```

**Error mostrado:**
```
error: failed to select a version for `esp-backtrace`.
    ... required by package `esp32c3-demo v0.1.0`
versions that meet the requirements `^0.19` are: 0.19.0

package `esp32c3-demo` depends on `esp-backtrace` with feature `exception-handler`
but `esp-backtrace` does not have that feature.
help: available features: colors, custom-halt, custom-pre-backtrace, default,
defmt, esp32, esp32c2, esp32c3, esp32c5, esp32c6, esp32c61, esp32h2, esp32s2,
esp32s3, halt-cores, panic-handler, print-float-registers, println, semihosting
```

**Causa:**
En `esp-backtrace 0.19`, el feature `exception-handler` fue eliminado o renombrado. La documentación de versiones anteriores todavía lo menciona.

**Solución:**
En `Cargo.toml`, eliminar `"exception-handler"` de las features de `esp-backtrace`:

```toml
# ❌ Incorrecto:
esp-backtrace = { version = "0.19", features = ["esp32c3", "exception-handler", "panic-handler", "println"] }

# ✅ Correcto:
esp-backtrace = { version = "0.19", features = ["esp32c3", "panic-handler", "println"] }
```

**Lección aprendida:**
Cuando un feature no existe, Cargo indica exactamente cuáles features **sí están disponibles**. Leer el mensaje de ayuda (`help: available features`) es suficiente para resolver este tipo de error.

---

### ✅ Compilación exitosa

Después de corregir el error anterior:

```
Compiling esp32c3-demo v0.1.0 (/workspace/ESP32-C3-rust)
Finished `dev` profile [optimized + debuginfo] target(s) in 25.03s
```

El firmware se genera en:
```
target/riscv32imc-unknown-none-elf/debug/esp32c3-demo
```

---

### ⚠️ Advertencia sobre `Cargo.lock`

Por defecto, los proyectos de bibliotecas no incluyen `Cargo.lock` en el repositorio. Para proyectos **binarios** (como firmware embebido), es **recomendable** incluirlo para garantizar que todos reproduzcan exactamente las mismas versiones de dependencias.

Por eso en el `.gitignore` **no** excluimos `Cargo.lock`.

---

## 9. Simulación con Wokwi

### ¿Qué es Wokwi?

Wokwi (https://wokwi.com) es un simulador de hardware electrónico que soporta ESP32, Arduino y muchos otros microcontroladores. Tiene una extensión para VS Code que permite:

1. Compilar el firmware automáticamente con `cargo build`
2. Cargar el binario ELF compilado en el simulador
3. Simular el circuito descrito en `diagram.json`

### Archivos de configuración de Wokwi

#### `wokwi.toml`
```toml
[wokwi]
version = 1
firmware = "target/riscv32imc-unknown-none-elf/debug/esp32c3-demo"
chip = "esp32c3"
```

Este archivo le dice a Wokwi:
- Qué chip simular (`esp32c3`)
- Dónde encontrar el firmware compilado

#### `diagram.json`
Describe el circuito: qué componentes hay y cómo están conectados.

### Circuito implementado

```
                         220Ω
ESP32-C3 GPIO2 ────────[R]──── [LED: Ánodo]
                                [LED: Cátodo] ─── GND

ESP32-C3 GPIO9 (pull-up interno)
      │
   [Botón]
      │
     GND

ESP32-C3 GPIO4 ─────────────── [Potenciómetro: SIG/Wiper]
ESP32-C3 3V3  ──────────────── [Potenciómetro: VCC]
ESP32-C3 GND  ──────────────── [Potenciómetro: GND]
```

### Cómo ejecutar la simulación

1. Abrir el proyecto en VS Code (o Codespace)
2. Instalar la extensión **Wokwi Simulator** (ID: `wokwi.wokwi-vscode`)
3. Abrir `diagram.json`
4. Hacer clic en **▶ Start Simulation** (o `F1` → `Wokwi: Start Simulator`)
5. La extensión compilará automáticamente y abrirá el simulador

### Monitor serie en Wokwi

La salida serie aparece en el panel de Wokwi y muestra mensajes como:

```
=== ESP32-C3 Demo: Pot + Botón + LED ===
GPIO4 -> Potenciómetro (ADC1)
GPIO9 -> Botón (pull-up, activo en LOW)
GPIO2 -> LED (con resistor 220 ohm)
=========================================
Pot:    0/4095  Botón: libre       LED: OFF
Pot: 1024/4095  Botón: libre       LED: OFF
Pot: 2100/4095  Botón: libre       LED: ON
Pot: 2100/4095  Botón: PRESIONADO  LED: ON
Pot:  500/4095  Botón: PRESIONADO  LED: ON
Pot:  500/4095  Botón: libre       LED: OFF
```

---

## 10. Grabar en hardware real

Si tienes una placa ESP32-C3 física (como la ESP32-C3-DevKitM-1):

### Instalar espflash

```bash
cargo install espflash
```

### Compilar en modo release

```bash
cargo build --release
```

### Grabar el firmware

```bash
cargo run --release
```

O directamente:

```bash
espflash flash target/riscv32imc-unknown-none-elf/release/esp32c3-demo --monitor
```

> **Nota:** Puede ser necesario mantener presionado el botón BOOT de la placa al conectarla para entrar en modo de programación.

### Conexiones físicas

| ESP32-C3 Pin | Componente |
|---|---|
| GPIO2 | Ánodo LED (a través de resistor 220 Ω) |
| GND | Cátodo LED |
| GPIO9 | Terminal 1 del botón |
| GND | Terminal 2 del botón |
| GPIO4 | Wiper (pin central) del potenciómetro |
| 3V3 | Extremo VCC del potenciómetro |
| GND | Extremo GND del potenciómetro |

---

## 11. Ejercicios propuestos

### Nivel básico

1. **Cambiar el umbral del LED**: Modificar el valor `2048` para que el LED se encienda al 25% (1024) o al 75% (3072) del recorrido del potenciómetro.

2. **Invertir la lógica del botón**: Hacer que el botón **apague** el LED en lugar de encenderlo.

3. **Cambiar el intervalo de muestreo**: Modificar `200` en `delay.delay_millis(200)` para muestrear cada 50 ms o cada 1000 ms. ¿Qué diferencia se observa?

### Nivel intermedio

4. **Añadir segundo LED**: Conectar un segundo LED a GPIO8 que se encienda solo cuando el potenciómetro esté entre 25% y 75%.

5. **Mapa de voltaje**: Convertir el valor ADC (0–4095) a voltaje en mV:
   ```rust
   let voltage_mv = (adc_value as u32 * 3300) / 4095;
   ```

6. **Contador de pulsaciones**: Usar una variable `u32` para contar cuántas veces se presiona el botón y mostrarla en el monitor serie.

### Nivel avanzado

7. **Histéresis**: Implementar histéresis para evitar que el LED parpadee cuando el potenciómetro está cerca del umbral. El LED se enciende si ADC > 2200 y se apaga si ADC < 1800.

8. **Promedio móvil**: Leer el ADC 8 veces y promediar los resultados para reducir el ruido.

9. **PWM con LED**: Usar el periférico LEDC del ESP32-C3 para controlar el brillo del LED proporcionalmente al valor del potenciómetro.

---

## Referencias

- [Libro oficial: Rust on ESP](https://docs.esp-rs.org/book/)
- [Entrenamiento no_std esp-rs](https://docs.esp-rs.org/no_std-training/)
- [Documentación esp-hal](https://docs.rs/esp-hal/latest/esp_hal/)
- [Ejemplos oficiales esp-hal](https://github.com/esp-rs/esp-hal/tree/main/examples)
- [Wokwi Simulator](https://wokwi.com)
- [Wokwi ESP32-C3 DevKit M1](https://docs.wokwi.com/parts/board-esp32c3-devkitm-1)
- [ESP32-C3 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf)
