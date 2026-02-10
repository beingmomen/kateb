import { tauriInvoke } from '~/utils/tauri'

export default defineNuxtPlugin(async () => {
  const { setLocale } = useI18n()

  try {
    const settings = await tauriInvoke('get_all_settings') || []
    const langSetting = settings.find(s => s.key === 'language')
    if (langSetting) {
      let lang = langSetting.value
      try {
        lang = JSON.parse(lang)
      } catch { /* use raw value */ }
      if (lang === 'en' || lang === 'ar') {
        await setLocale(lang)
      }
    }
  } catch { /* Tauri not available or settings error */ }
})
