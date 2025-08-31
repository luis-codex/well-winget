# well-winget — Un widget Wayland con WebKit

Un microcontenedor en Rust para interfaces web en Wayland. Echo con GTK4 + WebKitGTK 6, para que puedas construir tu propia barra, lanzador o panel en la web y mostrarlo como overlay nativo en tu compositor.

Esta pensado para que sea ultra-flexible con el poder de la web para crear interfaces rápidas, animadas y totalmente personalizadas.

## Características
- Hecho en Rust (edición 2024): rendimiento nativo y seguridad de memoria.
- Multi-monitor: crea una ventana por monitor automáticamente.
- Overlay nativo Wayland: usa `gtk4-layer-shell` en capa `Overlay` y zona exclusiva 0 (flotante).
- Fondo transparente real: el `WebView` utiliza RGBA con alpha.
- Animación hover/slide: aparece al pasar el ratón y se oculta dejando un “peek”.
- Carga desde `http://localhost:<APP_PORT>/`: integrable con cualquier servidor local (Svelte/React/Vue, estático, etc.).
- Dev extras activados: consola y herramientas de dev de WebKit para depurar tu UI.

## Requisitos
- Wayland (variable `GDK_BACKEND=wayland` se establece automáticamente).
- WebKitGTK 6.0.
- GTK4.
- Rust y Cargo.

Ejemplo en (Arch):
```
sudo pacman -S webkitgtk-6.0 gtk4
```
En (Debian/Ubuntu):
```
sudo apt install libwebkit2gtk-6.0-dev libgtk-4-dev
```


## Instalación y compilación
Clona y compila en modo release para generar el binario:
```
git clone https://github.com/luis-codex/well-winget
cd well-winget
cargo build --release
```
El binario queda en `target/release/well-winget`.

## Inicio rápido
1) Arranca tu UI local (ejemplo estático con Python):
```
mkdir -p ui && cat > ui/index.html <<'HTML'
<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <style>
      html,body { margin:0; height:100%; }
      body { display:flex; align-items:center; justify-content:center; color:#e5e7eb; background:transparent; font:16px/1.4 system-ui; }
      .chip { padding:8px 12px; border-radius:12px; background:rgba(30,30,35,.65); backdrop-filter: blur(6px); }
    </style>
  </head>
  <body>
    <div class="chip">Hello from WebKit overlay ✨</div>
  </body>
  </html>
HTML

pushd ui >/dev/null
python -m http.server 2002
```

2) En otra terminal, usa el binario compilado (recomendado) o `cargo run` (desarrollo):
```
# Binario compilado
APP_PORT=2002 ./target/release/well-winget

# (Opcional) Desarrollo
APP_PORT=2002 cargo run --release
```
Al pasar el ratón por el borde asignado, la ventana se desliza y muestra tu UI.

## Uso del binario
- Ejecuta el binario y apunta `APP_PORT` al puerto de tu UI.
- Ejemplo:
```
APP_PORT=2002 ./target/release/well-winget
```
- Instalación local opcional para tenerlo en PATH:
```
install -Dm755 target/release/well-winget ~/.local/bin/well-winget
```
Luego ejecútalo simplemente con:
```
APP_PORT=2002 well-winget
```

## Cómo funciona
- El binario crea una ventana por monitor y la ancla a un borde (por defecto, inferior) mediante layer-shell.
- Embebe un `WebView` de WebKitGTK con fondo transparente y carga `http://localhost:<APP_PORT>/`.
- Implementa una animación de deslizamiento controlada por hover para mostrar/ocultar el panel.


## Configuración
Actualmente, la configuración es mínima y se controla por:
- `APP_PORT`: puerto del servidor UI (por defecto `2002`).
- Constantes en `src/main.rs`:
  - `BAR_HEIGHT` y `BAR_WIDTH`: tamaño del panel.
  - `BAR_POS`: `Top` o `Bottom`.
  - `PEEK`: píxeles visibles cuando está oculto.
  - `ANIM_MS`: duración aproximada de la animación.

## Rendimiento y buenas prácticas
- Mantén la UI ligera (evita bundles enormes, usa SSR/streaming si aplica).
- Usa `--release` y activa compresión/caché en tu servidor UI.
- Evita repintados costosos; usa `transform/opacity` para animaciones suaves.


## Roadmap
- Parámetros por CLI/env para tamaño, posición, y borde.
- Soporte para `Top/Bottom/Left/Right` y auto-size dinámico.
- Modo barra “persistente” con `exclusive_zone` opcional.
- IPC y APIs (ej. DBus/Unix socket) para controlar la UI.
- Plantillas de ejemplo para barra y launcher.
- Tests de integración básicos y CI.

## Licencia
MIT. Consulta `LICENSE` para el texto completo.
