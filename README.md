# 🌌 Sistema Solar — Shader Dinámico en Rust

![Demo](docs/demo.gif)  
*[demo.mp4](docs/demo.mp4)*

Simulación 3D interactiva de un sistema solar implementada desde cero en Rust, con:
- 🌞 Sol procedural con textura dinámica (plasma, células, turbulencia)
- 🪐 Planetas realistas: Marte, Saturno (con anillos), Urano, Neptuno, y más
- 🚀 Nave espacial orbitando con rotación sincronizada
- 🌠 Fondo estelar generado proceduralmente (`draw_stars`)
- 🎮 Cámara libre (movimiento)
- 🖥️ Renderizado por software (sin GPU, usando framebuffer personalizado)

Todos los shaders están escritos manualmente: iluminación, mapeo UV esférico, ruido fractal, y más.

---

## 🛠️ Cómo ejecutar

Primero, asegúrate de tener [Rust](https://www.rust-lang.org/tools/install) instalado.

Luego, clona y ejecuta en modo **release** (recomendado para mejor rendimiento):

```bash
git clone https://github.com/tu-usuario/sistema-solar-rust.git
cd sistema-solar-rust
cargo run --release
```
---

## 🎮 Controles

| Tecla          | Acción                                 |
|----------------|----------------------------------------|
| **← (Izquierda)** | Rotar la cámara **hacia la izquierda** |
| **→ (Derecha)**   | Rotar la cámara **hacia la derecha**   |
| **↑ (Arriba)**    | Inclinar la cámara **hacia arriba**    |
| **↓ (Abajo)**     | Inclinar la cámara **hacia abajo**     |