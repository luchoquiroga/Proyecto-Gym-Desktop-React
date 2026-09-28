# AGENTS.md

Este repo es el **shell de escritorio del gimnasio**: una ventana de Tauri v2
que carga la web (`../Gym-Project-Front-End`). No tiene UI propia.

Las decisiones que lo definen viven en el repo de la web, no acá:
`specs/STACK.md` §6 y `specs/ARQUITECTURA-APPS.md`. La bitácora del trabajo es
`../Gym-Project-Front-End/specs/BITACORA.md`.

## Reglas que no se negocian

- **El shell carga la URL de la web. No empaqueta el build.** Si la página
  corriera desde `tauri://localhost`, la cookie `refreshToken` sería de tercero
  y se perdería el silent refresh (la sesión moriría a los 30 minutos).
- **Cero lógica de negocio, cero llamadas propias al API.** Si el shell parece
  necesitar llamar al API, se reabre la decisión en `ARQUITECTURA-APPS.md`, no
  se resuelve en el código.
- **La página no recibe permisos de Tauri**: `app.security.capabilities` está
  vacío y no hay comandos ni plugins. Agregar uno amplía lo que la web (y
  cualquier script que se cuele en ella) puede hacerle a la PC.
- Lo que sí es del shell: ventana, ícono, título, instalador y auto-update.

## Configuración

`src-tauri/tauri.conf.json`:

- `build.devUrl`: `http://localhost:5173` (el `pnpm dev` de la web).
- `build.frontendDist`: la URL de producción,
  `https://gimnasioathletics.vercel.app` (paso 8.3). Si la web cambia de
  dominio, hay que cambiarla acá y volver a generar el instalador.
- `productName` y título de la ventana: `Athletics`, el nombre de la web
  (`lib/marca.ts` del front). El `identifier` (`ar.gimnasio.escritorio`) **no
  se cambia**: es lo que Windows usa para reconocer la app entre versiones.
- Instalador NSIS en español, `installMode: currentUser` (se instala sin
  permisos de administrador, en `%LOCALAPPDATA%`).
- El ícono sale de `icon-fuente/athletics.svg`, copia del `favicon.svg` de la
  web. Si cambia el símbolo: `pnpm tauri icon icon-fuente/athletics.svg` y
  borrar las carpetas `android/` e `ios/` que genera.

## Pendiente

- **Firma de código**: el instalador no está firmado, así que Windows
  SmartScreen avisa "editor desconocido" (se instala igual con "Más
  información → Ejecutar de todas formas"). Sacarlo requiere un certificado.
- **Auto-update**: no está armado. Mientras tanto, como la app carga la URL,
  los cambios de la web llegan solos; solo hace falta reinstalar si cambia el
  shell (nombre, ícono, URL).

## Uso

```bash
pnpm install
pnpm dev     # necesita la web en :5173 y el backend en :8080
pnpm build   # instalador NSIS en src-tauri/target/release/bundle/nsis/
```

Requiere Rust (toolchain `stable-msvc`) y las Build Tools de C++ de Visual
Studio. La primera compilación tarda unos minutos.

## Git

No commitear ni pushear sin que lo pida el dueño.
