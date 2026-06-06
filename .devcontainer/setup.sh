#!/usr/bin/env bash
# ============================================================
# setup.sh  –  Configura el entorno de desarrollo ESP32-C3 Rust
# Se ejecuta automáticamente al crear el Codespace
# ============================================================
set -euo pipefail

echo "========================================="
echo " Configurando entorno ESP32-C3 Rust"
echo "========================================="

# 1. Actualizar rustup y Rust
echo "[1/4] Actualizando Rust toolchain..."
rustup update stable
rustup target add riscv32imc-unknown-none-elf
rustup component add clippy rustfmt rust-src

# 2. Instalar espflash (para grabar en hardware real)
echo "[2/4] Instalando espflash..."
cargo install espflash --locked 2>/dev/null || echo "espflash ya instalado o fallo no crítico"

# 3. Instalar cargo-espflash (herramienta de Espressif)
echo "[3/4] Instalando probe-rs (depurador)..."
cargo install probe-rs-tools --locked 2>/dev/null || echo "probe-rs ya instalado o fallo no crítico"

# 4. Verificar instalación
echo "[4/4] Verificando instalación..."
echo "  Rust: $(rustc --version)"
echo "  Cargo: $(cargo --version)"
echo "  Target riscv32imc: $(rustup target list --installed | grep riscv32imc || echo 'NO INSTALADO')"

echo ""
echo "==========================================="
echo " ✅ Entorno configurado correctamente"
echo " 📦 Para compilar:   cargo build"
echo " 🔌 Para grabar:     cargo run"
echo " 🧪 Para simular:    abre diagram.json en VS Code con la extensión Wokwi"
echo "==========================================="
