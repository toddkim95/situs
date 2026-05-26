use super::MessageKey;

pub(super) fn es(key: MessageKey) -> &'static str {
    match key {
        MessageKey::KeymapTitle => "Keymap de Situs",
        MessageKey::KeymapPicker => "Selector",
        MessageKey::KeymapViews => "Vistas",
        MessageKey::KeymapHistory => "Historial",
        MessageKey::KeymapUpDown => "seleccionar filas del historial",
        MessageKey::KeymapPage => "desplazarse por las filas",
        MessageKey::KeymapLeftRight => "mover el cursor de consulta inferior",
        MessageKey::KeymapHomeEnd => "ir al inicio o al final de la consulta",
        MessageKey::KeymapTab => "cd al directorio seleccionado y mantener la consulta en zsh",
        MessageKey::KeymapEnter => {
            "cd al directorio seleccionado y ejecutar el comando seleccionado"
        }
        MessageKey::KeymapPut => "pegar el comando seleccionado sin cd ni ejecutar",
        MessageKey::KeymapEsc => "salir y mantener la entrada original de la shell",
        MessageKey::KeymapHelp => "mostrar/ocultar ayuda",
        MessageKey::KeymapFailed => "mostrar u ocultar historial fallido",
        MessageKey::KeymapInspect => "inspeccionar historial seleccionado",
        MessageKey::KeymapSource => "ciclar filtro de origen: all, local, atuin",
        MessageKey::KeymapContext => "ciclar filtro de contexto: all, directory, workspace",
        MessageKey::KeymapCopy => "copiar comando seleccionado",
        MessageKey::KeymapDelete => "eliminar fila seleccionada del historial de situs",
        MessageKey::SetupTitle => "Configuración de Situs",
        MessageKey::SetupPickerUi => "UI del Selector:",
        MessageKey::SetupInline => "  1) inline      selector compacto debajo de tu prompt",
        MessageKey::SetupFullscreen => "  2) fullscreen  TUI en pantalla completa",
        MessageKey::SetupChoose => "Elegir [1]: ",
        MessageKey::SetupPickerModeSetPrefix => "Modo de selector establecido en",
        MessageKey::SetupAtuinFound => {
            "Se encontró el historial de Atuin. ¿Activar la sincronización automática de Atuin?"
        }
        MessageKey::SetupAtuinImportFound => "¿Importar ahora el historial de Atuin en Situs?",
        MessageKey::SetupAtuinSetPrefix => "Sincronización automática de Atuin establecida en",
        MessageKey::SetupZshrcHint => "Agrega esto a tu ~/.zshrc si aún no está allí:",
        MessageKey::DoctorTitle => "Diagnóstico de Situs",
        MessageKey::DoctorHistoryPath => "ruta del historial",
        MessageKey::DoctorHistoryRecords => "registros del historial",
        MessageKey::DoctorKeyBinding => "atajo de teclado",
        MessageKey::DoctorMode => "modo",
        MessageKey::DoctorPickerMode => "modo de selector",
        MessageKey::DoctorAtuinSync => "sincronización de atuin",
        MessageKey::DoctorZshIntegration => "integración con zsh",
        MessageKey::DoctorAtuinDb => "bd de atuin",
        MessageKey::DoctorConfigured => "configurado",
        MessageKey::DoctorNotFound => "no encontrado",
        MessageKey::PickerSearch => "Buscar",
        MessageKey::PickerInspect => "Detalle",
        MessageKey::PickerKeyboard => "Teclado",
        MessageKey::PickerLoadingHistory => "Cargando historial de comandos",
        MessageKey::PickerNoHistory => "No se encontró historial de directorios",
        MessageKey::PickerNoHistoryHint => {
            "Ejecuta el comando una vez en el directorio correcto y vuelve a intentarlo."
        }
        MessageKey::PickerNoMatches => "Ningún directorio coincide con la consulta actual.",
        MessageKey::PickerNoSelected => {
            "No hay ningún elemento de historial seleccionado para inspeccionar."
        }
        MessageKey::PickerCandidateCommand => "Comando",
        MessageKey::PickerCandidateDirectory => "Directorio",
        MessageKey::PickerCandidateStatus => "Estado",
        MessageKey::PickerCandidateWhen => "Cuándo",
        MessageKey::PickerCandidateCompactHeader => "comando / directorio",
        MessageKey::PickerResultSingular => "resultado",
        MessageKey::PickerResultPlural => "resultados",
        MessageKey::PickerSuccessfulHistory => "solo exitoso",
        MessageKey::PickerAllHistory => "todo el historial",
        MessageKey::PickerHelpQuit => "salir",
        MessageKey::PickerHelpSelect => "seleccionar",
        MessageKey::PickerHelpEdit => "editar",
        MessageKey::PickerHelpCd => "cd",
        MessageKey::PickerHelpRun => "ejecutar",
        MessageKey::PickerHelpCopy => "copiar",
        MessageKey::PickerHelpDelete => "eliminar",
        MessageKey::PickerHelpSource => "origen",
        MessageKey::PickerHelpContext => "contexto",
        MessageKey::PickerHelpHelp => "ayuda",
        MessageKey::PickerHelpSelectPrevious => "seleccionar comando anterior",
        MessageKey::PickerHelpEditQuery => "editar la consulta inferior fija",
        MessageKey::PickerHelpCdKeepQuery => "cd al directorio seleccionado y mantener la consulta",
        MessageKey::PickerHelpRunSelected => "ejecutar el comando seleccionado en ese directorio",
        MessageKey::PickerHelpPasteCommand => "pegar el comando seleccionado sin cd ni ejecutar",
        MessageKey::PickerHelpCopyCommand => "copiar comando seleccionado",
        MessageKey::PickerHelpDeleteRow => "eliminar fila de historial local seleccionada",
        MessageKey::PickerHelpCycleSource => "ciclar filtro de origen",
        MessageKey::PickerHelpCycleContext => "ciclar filtro de contexto",
        MessageKey::PickerHelpShowHideFailed => "mostrar u ocultar comandos fallidos",
        MessageKey::PickerInspectCommand => "comando",
        MessageKey::PickerInspectCwd => "cwd",
        MessageKey::PickerInspectStatus => "estado",
        MessageKey::PickerInspectSource => "origen",
        MessageKey::PickerInspectRuns => "ejecuciones",
        MessageKey::PickerInspectWhen => "cuándo",
        MessageKey::PickerInspectEnter => "ejecutar comando en este directorio",
        MessageKey::PickerInspectTab => "cd aquí y mantener la consulta",
        MessageKey::PickerMessageCopied => "comando copiado",
        MessageKey::PickerMessageCopyFailed => "error al copiar",
        MessageKey::PickerMessageNothingSelected => "nada seleccionado",
        MessageKey::PickerMessageDeletedRows => "filas del historial",
        MessageKey::PickerMessageNothingDeleted => "nada eliminado",
        MessageKey::PickerMessageSource => "origen",
        MessageKey::PickerMessageContext => "contexto",
        MessageKey::PickerMessageShowingFailed => "mostrando historial fallido",
        MessageKey::PickerMessageHidingFailed => "ocultando historial fallido",
        MessageKey::StatsTitle => "Estadísticas de Situs",
        MessageKey::StatsRecords => "registros",
        MessageKey::StatsSuccessful => "exitosos",
        MessageKey::StatsFailed => "fallidos",
        MessageKey::StatsLocal => "local",
        MessageKey::StatsAtuin => "atuin",
        MessageKey::StatsTopCommands => "Comandos principales",
        MessageKey::StatsTopDirectories => "Directorios principales",
        MessageKey::StatsNone => "ninguno",
        MessageKey::SetupTuiTitle => "Configuración de Situs CLI (TUI)",
        MessageKey::SetupTuiHelp => {
            "Arr/Aba: Navegar | Izq/Der/Espacio/Enter: Alternar | S: Guardar | Esc/Q: Cancelar"
        }
        MessageKey::SetupTuiPickerMode => "Modo de Interfaz del Selector",
        MessageKey::SetupTuiAtuinSync => "Sincronización Automática de Atuin",
        MessageKey::SetupTuiLanguage => "Idioma de Visualización",
        MessageKey::SetupTuiSaveBtn => "[ Guardar Configuración ]",
        MessageKey::SetupTuiCancelBtn => "[ Cancelar ]",
        MessageKey::SetupTuiSavedMessage => "¡Configuración guardada correctamente!",
        MessageKey::SetupTuiWidgetKey => "Tecla de atajo del widget",
        MessageKey::SetupTuiShellInit => "Agregar automáticamente al perfil",
        MessageKey::SetupTuiAtuinImport => "Importación única de Atuin",
    }
}

pub(super) const ES_HELP_TEXT: &str = "\
situs - recuerda dónde funcionaron antes los comandos de shell

Usage:
  situs setup
  situs init zsh
  situs doctor
  situs keymap
  situs atuin enable|disable|status
  situs import atuin [--db <path>]
  situs record --cwd <dir> --status <code> -- <command>
  situs choose [--mode stay|restore] [--picker inline|fullscreen] [--context all|directory|workspace] --command <command>
  situs choose --print-dir --command <command>
  situs choose --print-selection --command <command>
  situs choose --print-widget-selection --command <command>
  situs run -- <command>
  situs stats

Notes:
  choose abre el selector de directorio, luego ejecuta el comando en el directorio seleccionado.
  --mode restore regresa al directorio original de la shell cuando lo usa la integración de zsh.
  --include-failed muestra ejecuciones de comandos fallidas además de las exitosas.
  --context directory limita las coincidencias al directorio actual; workspace las limita al repositorio git actual.
  --print-dir imprime el directorio seleccionado para integraciones de shell.
  --print-selection imprime el directorio seleccionado y el comando en líneas separadas.
  --print-widget-selection imprime la acción, el directorio, el comando y la consulta para integraciones de shell.
  --print-widget-selection requiere un selector TUI y nunca recurre al selector simple.
  doctor imprime diagnósticos de instalación e historial.
  keymap imprime los atajos de teclado del selector.
  stats resume los comandos recordados, los directorios, la mezcla de fuentes y los fallos.
  setup configura el modo de selector y la sincronización automática opcional de Atuin.
  atuin enable almacena la sincronización automática de Atuin en el archivo de configuración de situs.
  import atuin lee el historial de SQLite de Atuin en el historial de situs.
  Establezca SITUS_PICKER=fullscreen para anular el modo de selector configurado.
  Establezca SITUS_ATUIN_SYNC=auto para anular el modo de sincronización de Atuin configurado.

Try:
  eval \"$(situs init zsh)\"
";
