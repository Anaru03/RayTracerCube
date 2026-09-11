# RayTracerCube

Implementación de un cubo 3D utilizando Ray Tracing en Rust, con iluminación difusa y una cámara orbital interactiva.

Este ejercicio forma parte del curso de Gráficas por Computadora y tiene como objetivo aplicar los fundamentos de ray tracing para generar geometría tridimensional mediante rayos primarios, intersecciones, normales e iluminación.

---

## Características

- Ray tracing con un rayo primario por píxel
- Intersección entre rayos y un cubo 3D
- Cálculo del punto de impacto
- Normales independientes para cada cara del cubo
- Iluminación difusa
- Fuente de luz fija en el mundo
- Cámara orbital interactiva
- Rotación horizontal y vertical
- Framebuffer propio
- Cubo con color base rojo
- Base para el escenario

---

## Cámara orbital

La cámara permanece apuntando hacia el centro de la escena mientras se desplaza alrededor del cubo.

Los movimientos horizontales corresponden al **yaw** y los movimientos verticales al **pitch**.

| Tecla | Acción |
|:---:|---|
| `←` | Orbitar hacia la izquierda |
| `→` | Orbitar hacia la derecha |
| `↑` | Orbitar hacia arriba |
| `↓` | Orbitar hacia abajo |
| `Esc` | Cerrar |

El cubo y la fuente de luz permanecen fijos. Únicamente cambia la posición de la cámara.

---

## Iluminación difusa

La escena utiliza únicamente iluminación difusa.

Para cada punto de impacto se calcula la dirección hacia la fuente de luz y se compara con la normal de la superficie mediante el producto punto.

```text
Píxel
  ↓
Rayo primario
  ↓
Intersección con el cubo
  ↓
Punto de impacto
  ↓
Normal de la cara
  ↓
Dirección hacia la luz
  ↓
Producto punto
  ↓
Intensidad difusa
  ↓
Color del píxel
```

Todas las caras del cubo utilizan el mismo color base. Las diferencias de intensidad visibles entre ellas son resultado de su orientación respecto a la fuente de luz.

---

## Ejecución

Requiere Rust y Cargo.

```bash
cargo run
```

---

## Tecnologías

- Rust
- Ray Tracing
- minifb

---

## Branch

El código correspondiente a este ejercicio se encuentra en:

```text
ejercicio-cube
```