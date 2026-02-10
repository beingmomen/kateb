import { tauriInvoke } from '~/utils/tauri'

export default defineNuxtPlugin(async (nuxtApp) => {
  const i18n = nuxtApp.$i18n

  try {
    const settings = await tauriInvoke('get_all_settings') || []
    const langSetting = settings.find(s => s.key === 'language')
    if (langSetting) {
      let lang = langSetting.value
      try {
        lang = JSON.parse(lang)
      } catch { /* use raw value */ }
      if (lang === 'en' || lang === 'ar') {
        await i18n.setLocale(lang)
      }
    }
  } catch { /* Tauri not available or settings error */ }
})
