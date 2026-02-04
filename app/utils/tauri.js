export function isTauri() {
  return typeof window !== 'undefined' && window.__TAURI_INTERNALS__
}

export async function tauriInvoke(command, args) {
  if (!isTauri()) {
    console.warn(`[Tauri Mock] invoke('${command}') — not in Tauri environment`)
    return null
  }
  const { invoke } = await import('@tauri-apps/api/core')
  return invoke(command, args)
}

export async function tauriListen(event, callback) {
  if (!isTauri()) {
    console.warn(`[Tauri Mock] listen('${event}') — not in Tauri environment`)
    return () => {}
  }
  const { listen } = await import('@tauri-apps/api/event')
  return listen(event, callback)
}
