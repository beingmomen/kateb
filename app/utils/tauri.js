export function isTauri() {
  return typeof window !== 'undefined' && window.__TAURI_INTERNALS__
}

export async function tauriInvoke(command, args) {
  if (!isTauri()) {
    console.warn(`[Tauri Mock] invoke('${command}') — not in Tauri environment`)
    if (command === 'has_active_model') return true
    if (command === 'get_active_model') return { id: 'large-v3-turbo', name: 'Large V3 Turbo' }
    if (command === 'get_available_models') return [
      { id: 'tiny', name: 'Tiny', filename: 'tiny.bin', size_bytes: 77704715, size_display: '74 MB', accuracy: 1, speed: 5, ram_mb: 390, description_ar: 'أصغر وأسرع نموذج', pros_ar: ['سريع جداً', 'حجم صغير'], cons_ar: ['دقة منخفضة'], recommended: false, installed: false },
      { id: 'base', name: 'Base', filename: 'base.bin', size_bytes: 147951465, size_display: '141 MB', accuracy: 2, speed: 4, ram_mb: 500, description_ar: 'نموذج أساسي', pros_ar: ['سريع', 'حجم معقول'], cons_ar: ['دقة متوسطة'], recommended: false, installed: true },
      { id: 'small', name: 'Small', filename: 'small.bin', size_bytes: 487601065, size_display: '465 MB', accuracy: 3, speed: 3, ram_mb: 1000, description_ar: 'توازن بين السرعة والدقة', pros_ar: ['دقة جيدة'], cons_ar: ['أبطأ قليلاً'], recommended: false, installed: false },
      { id: 'medium', name: 'Medium', filename: 'medium.bin', size_bytes: 1533774781, size_display: '1.4 GB', accuracy: 4, speed: 2, ram_mb: 2600, description_ar: 'دقة عالية', pros_ar: ['دقة عالية'], cons_ar: ['بطيء', 'حجم كبير'], recommended: false, installed: false },
      { id: 'large-v3-turbo', name: 'Large V3 Turbo', filename: 'large-v3-turbo.bin', size_bytes: 1623507861, size_display: '1.5 GB', accuracy: 5, speed: 3, ram_mb: 3800, description_ar: 'أعلى دقة مع سرعة محسنة', pros_ar: ['أعلى دقة', 'سرعة محسنة'], cons_ar: ['حجم كبير جداً'], recommended: true, installed: true }
    ]
    if (command === 'get_all_settings') return [
      { key: 'language', value: 'ar' },
      { key: 'ai_refinement', value: 'false' },
      { key: 'ai_provider', value: 'local' },
      { key: 'auto_punctuation', value: 'true' },
      { key: 'sound_notifications', value: 'true' },
      { key: 'auto_type', value: 'true' },
      { key: 'active_model', value: 'large-v3-turbo' }
    ]
    if (command === 'get_ai_providers') return [
      { id: 'claude', name: 'Claude' },
      { id: 'openai', name: 'OpenAI' },
      { id: 'gemini', name: 'Gemini' },
      { id: 'grok', name: 'Grok' },
      { id: 'local', name: 'Local' }
    ]
    if (command === 'detect_gpu') return { cuda_available: false, recommended: 'cpu' }
    if (command === 'get_history') return []
    if (command === 'get_usage_stats') return []
    if (command === 'get_summary_stats') return { total_dictations: 0, total_words: 0, total_duration: 0, days_active: 0 }
    if (command === 'get_audio_devices') return []
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
