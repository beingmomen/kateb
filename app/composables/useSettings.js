import { tauriInvoke } from '~/utils/tauri'

export function useSettings() {
  const settings = ref([])
  const loading = ref(false)
  const error = ref(null)

  async function fetchSettings() {
    loading.value = true
    try {
      settings.value = await tauriInvoke('get_all_settings') || []
    } catch (e) {
      error.value = e
    } finally {
      loading.value = false
    }
  }

  async function getSetting(key) {
    try {
      return await tauriInvoke('get_setting', { key })
    } catch (e) {
      error.value = e
      return null
    }
  }

  async function updateSetting(key, value) {
    await tauriInvoke('update_setting', { key, value: String(value) })
    await fetchSettings()
  }

  function getSettingValue(key, defaultValue = null) {
    const setting = settings.value.find(s => s.key === key)
    if (!setting) return defaultValue
    try {
      return JSON.parse(setting.value)
    } catch {
      return setting.value
    }
  }

  return {
    settings,
    loading,
    error,
    fetchSettings,
    getSetting,
    updateSetting,
    getSettingValue
  }
}
