# situs-cli

[![CI](https://github.com/toddkim95/situs/actions/workflows/ci.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/ci.yml)
[![Security](https://github.com/toddkim95/situs/actions/workflows/security.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/security.yml)

[English](../../README.md) | [한국어](../ko/README.md) | [简体中文](../zh-Hans/README.md) | [Español](README.md) | [日本語](../ja/README.md)

`situs` es un pequeño **command cwd resolver** para zsh.

Recuerda dónde funcionó un comando anteriormente, y luego te permite ejecutar o preparar ese comando desde el directorio recordado sin tener que moverte manualmente con `cd`.

> [!NOTE]
> Este documento es la traducción al español. El [README.md](../../README.md) en inglés es el source of truth. Los nombres de comandos, flags, variables de entorno, claves de configuración y valores del shell protocol se mantienen intencionalmente en inglés.

```text
~/notes
> cargo build
  presiona Ctrl-G

Situs abre un selector compacto:
  cargo build --release        .../work/app        ok        2h ago
> cargo build
  esc salir  up/down seleccionar  tab cd  enter ejecutar
```

### Por qué existe esto

1. Ejecutas `cargo build --release` con éxito en `/Users/me/work/app`.
2. Más tarde, desde otro directorio, escribes `cargo build --release`.
3. Presionas el atajo de teclado de Situs.
4. Eliges el directorio de trabajo anterior.
5. Situs convierte la línea de la shell en `cd -- /Users/me/work/app && cargo build --release`.

Situs no es un reemplazo completo del historial de la shell. Atuin, McFly, fzf y HSTR son excelentes buscadores de historial; zoxide es un excelente saltador de directorios. Situs se enfoca en una tarea estrecha: resolver "command cwd resolver" (¿dónde funcionó este comando antes?).

## Capturas de pantalla

### Selector inline

| Buscar | Detalle | Ayuda |
| --- | --- | --- |
| ![Selector inline que muestra comandos recientes de cargo sobre la línea de consulta fija](../assets/screenshots/inline-search.svg) | ![Vista de detalle inline que muestra el comando seleccionado, cwd, estado, origen, ejecuciones y acciones](../assets/screenshots/inline-inspect.svg) | ![Vista de ayuda inline que muestra los atajos de teclado de Situs](../assets/screenshots/inline-help.svg) |

### Selector a pantalla completa

| Buscar | Detalle | Ayuda |
| --- | --- | --- |
| ![Selector a pantalla completa que muestra el mismo flujo de resolución de directorios con más espacio vertical](../assets/screenshots/fullscreen-search.svg) | ![Vista de detalle a pantalla completa con metadatos del comando](../assets/screenshots/fullscreen-inspect.svg) | ![Vista de ayuda a pantalla completa con atajos de teclado](../assets/screenshots/fullscreen-help.svg) |

## Características

- Recuerda el comando, cwd, estado de salida, marca de tiempo y origen.
- Prioriza las ejecuciones de comandos exitosas por defecto.
- Abre un selector inline compacto que mantiene visible tu línea de comandos actual.
- Admite un selector TUI a pantalla completa cuando prefieres una superficie más grande.
- Permite que `Tab` prepare el directorio y comando seleccionados sin ejecutarlos.
- Permite que `Enter` haga cd al directorio seleccionado y ejecute el comando del historial seleccionado.
- Amplía la coincidencia de comandos exactos a comandos parciales útiles como `cargo install`, `cargo install --path` y `cargo install --path .`.
- Filtra por historial local, historial de Atuin, directorio actual o espacio de trabajo de git actual.
- Puede importar el historial de SQLite de Atuin en modo de solo lectura.
- Mantiene un selector simple basado en líneas para escenarios sin TTY y de scripting.
- Soporta las shells `zsh`, `bash` y `fish` en entornos macOS y Linux.

## Instalación

### Desde GitHub

Una vez que este repositorio sea público:

```sh
cargo install --git https://github.com/toddkim95/situs
```

Si el repositorio se publica bajo un propietario o nombre diferente, reemplace la URL con la URL final de GitHub.

### Desde un Checkout Local

```sh
git clone https://github.com/toddkim95/situs
cd situs
cargo install --path .
```

### Desde crates.io

Una vez que el crate esté publicado:

```sh
cargo install situs-cli
```

## Inicio rápido

Agrega Situs a zsh:

```sh
eval "$(situs init zsh)"
```

Coloca esa misma línea cerca del final de tu `~/.zshrc`, luego abre una nueva terminal.

El atajo por defecto es `Ctrl-G`. Puedes cambiarlo antes de cargar el script de inicialización:

```sh
export SITUS_BINDKEY='^G'
eval "$(situs init zsh)"
```

Ejecuta diagnósticos:

```sh
situs doctor
```

Muestra los atajos del selector:

```sh
situs keymap
```

Para un flujo de configuración guiado:

```sh
situs setup
```

Más detalles de instalación en [docs/installation.md](../installation.md).

## Uso diario

Ejecuta comandos normalmente. La integración de zsh registra los comandos interactivos después de que terminan:

```sh
cd ~/work/app
cargo test
```

Más tarde, desde cualquier directorio:

```sh
cargo test
# presiona Ctrl-G
```

En el selector:

| Tecla | Acción |
| --- | --- |
| `Up` / `Down` | Seleccionar filas del historial y sincronizar la consulta con el comando seleccionado |
| `Left` / `Right` | Mover el cursor de consulta |
| `Tab` | `cd` al directorio seleccionado y mantener el comando en el buffer de la shell |
| `Enter` | `cd` al directorio seleccionado y ejecutar el comando seleccionado del historial |
| `Alt-Enter` / `Alt-Y` | Pegar el comando seleccionado en el buffer sin cambiar de directorio ni ejecutarlo |
| `Ctrl-F` | Mostrar u ocultar el historial de comandos fallidos |
| `Ctrl-O` | Inspeccionar el elemento del historial seleccionado |
| `F2` | Ciclar el filtro de origen: all, local, Atuin |
| `F3` | Ciclar el filtro de contexto: all, directory, workspace |
| `Ctrl-Y` | Copiar el comando seleccionado |
| `Ctrl-D` | Eliminar la fila de historial de Situs seleccionada |
| `Esc` | Salir y mantener la entrada original de la shell |

Las notas de uso completas están en [docs/usage.md](../usage.md).

## Modos del Selector

Selector inline, por defecto:

```sh
situs choose --picker inline --command "cargo build"
```

Selector a pantalla completa:

```sh
situs choose --picker fullscreen --command "cargo build"
```

Haz que la pantalla completa sea el valor predeterminado con:

```sh
export SITUS_PICKER=fullscreen
```

o ejecuta:

```sh
situs setup
```

Cuando varias filas visibles comparten el mismo prefijo de directorio, Situs oculta ese prefijo común con `*` para que el segmento de ruta significativo sea más fácil de escanear. El directorio seleccionado real sigue siendo la ruta completa.

## Atuin

Situs puede importar el historial de Atuin sin modificar su base de datos:

```sh
situs import atuin
```

Activa la importación automática de solo lectura antes de las búsquedas:

```sh
situs atuin enable
```

Comprueba el estado o desactívalo:

```sh
situs atuin status
situs atuin disable
```

Los detalles de la integración con Atuin están en [docs/configuration.md](../configuration.md).

## Comandos

```sh
situs init zsh
situs setup
situs doctor
situs keymap
situs atuin enable
situs atuin status
situs import atuin
situs record --cwd "$PWD" --status 0 -- "cargo build"
situs choose --picker fullscreen --mode restore --command "cargo build"
situs choose --context workspace --command "cargo test"
situs choose --print-dir --command "cargo build"
situs run -- cargo build
situs stats
```

Ejecuta `situs --help` para ver el resumen completo de comandos.

## Configuración

Variables de entorno comunes:

| Variable | Propósito |
| --- | --- |
| `SITUS_BINDKEY` | atajo de teclado de zsh, por defecto `^G` |
| `SITUS_MODE` | modo de ejecución de zsh: `stay` o `restore` |
| `SITUS_PICKER` | modo de selector: `inline` o `fullscreen` |
| `SITUS_INLINE_ROWS` | número de filas del selector inline |
| `SITUS_HISTORY` | anular la ruta del archivo de historial |
| `SITUS_CONFIG` | anular la ruta del archivo de configuración |
| `SITUS_ATUIN_SYNC` | anulación de sincronización de Atuin: `off`, `auto` o `always` |
| `SITUS_LANG` | idioma de la interfaz: `en`, `ko`, `zh-Hans`, `es` o `ja` |
| `SITUS_PLAIN` | usar el selector simple basado en líneas |

Consulta [docs/configuration.md](../configuration.md) para conocer las rutas de almacenamiento, los valores del archivo de configuración y los detalles del modo de ejecución.

## Comparación con otras herramientas

| Herramienta | Trabajo principal | Relación con Situs |
| --- | --- | --- |
| Atuin | Historial rico de shell, contexto, sincronización | Situs puede importar Atuin y usa un flujo de resolución de cwd más pequeño |
| McFly | Búsqueda inteligente de historial de shell | Situs resuelve el cwd para el comando que ya comenzaste a escribir |
| fzf | Buscador difuso general y atajos de shell | Situs tiene un selector diseñado específicamente y un protocolo de shell |
| zoxide | Saltos de directorio | Situs salta según el historial de comandos, no la frecuencia de directorios |
| HSTR | Cuadro de sugerencias de historial de shell | Situs mantiene unidos el comando, cwd, estado y semántica de acción |

La comparación larga está en [docs/comparison.md](../comparison.md).

## Desarrollo

Ejecute la matriz de verificación completa (formateo, clippy, pruebas unitarias/de aceptación, traducciones de documentos y pruebas de humo PTY) localmente:

```sh
scripts/verify-all.sh
```

También puede ejecutar pasos individuales:

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo package --locked --no-verify
scripts/verify-doc-i18n.sh
scripts/verify-picker-modes.sh
```

Las auditorías de avisos de RustSec se ejecutan en GitHub Actions. Para comprobar localmente:

```sh
cargo install cargo-audit --locked
cargo audit
```

Más orientación para colaboradores en [CONTRIBUTING.md](../../CONTRIBUTING.md) y [docs/development.md](../development.md).

Al agregar o cambiar funciones visibles para el usuario, actualice la cobertura del mensaje i18n en inglés, coreano y chino simplificado, o documente un fallback explícito en el mismo cambio. El mantenimiento de las traducciones del tiempo de ejecución y del README está documentado en [docs/i18n.md](../i18n.md).

Regenere las capturas de pantalla del README con:

```sh
scripts/capture-screenshots.js
```

El script de captura de pantalla utiliza el selector real con historial simulado de `fixtures/screenshot-history.tsv` y reintenta cada captura hasta tres veces.

## Resolución de problemas

Comienza con:

```sh
situs doctor
```

Soluciones comunes:

- Asegúrate de que `eval "$(situs init zsh)"` esté cargado en `~/.zshrc`.
- Abre una nueva shell después de cambiar `SITUS_BINDKEY`, `SITUS_PICKER` o `SITUS_MODE`.
- Después de reinstalar con `cargo install --path . --force`, ejecuta `source ~/.zshrc` o abre una nueva terminal para que se actualice el widget de zsh ya cargado.
- Usa `situs stats` para confirmar que se está registrando el historial.
- Usa `situs atuin status` si los resultados de Atuin no aparecen.
- Establece `SITUS_PLAIN=1` para aislar problemas de renderizado del terminal.

Más casos se cubren en [docs/troubleshooting.md](../troubleshooting.md).

## Contribución

Los informes de errores, las notas de UX y las solicitudes de extracción pequeñas y enfocadas son bienvenidos. Los cambios en el selector necesitan cobertura unitaria y cobertura de humo de zsh/PTY porque los pequeños cambios en el protocolo del terminal pueden romper el flujo de trabajo real de la shell.

Lee [CONTRIBUTING.md](../../CONTRIBUTING.md) antes de abrir una solicitud de extracción.

## Seguridad

No abras issues públicos para informes sensibles a la seguridad. Consulta [SECURITY.md](../../SECURITY.md).

## Licencia

MIT. Ver [LICENSE](../../LICENSE).
