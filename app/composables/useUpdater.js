import { isTauri } from '~/utils/tauri'
import { check } from '@tauri-apps/plugin-updater'
import { getVersion } from '@tauri-apps/api/app'
import { relaunch } from '@tauri-apps/plugin-process'

export function useUpdater() {
  const updateAvailable = ref(false)
  const updateVersion = ref('')
  const updateBody = ref('')
  const isChecking = ref(false)
  const isDownloading = ref(false)
  const downloadProgress = ref(0)
  const downloadedBytes = ref(0)
  const totalBytes = ref(0)
  const error = ref('')
  const updateInstance = ref(null)
  const appVersion = ref('')

  async function loadAppVersion() {
    if (!isTauri()) {
      appVersion.value = ''
      return
    }
    try {
      appVersion.value = await getVersion()
    } catch {
      appVersion.value = ''
    }
  }

  async function checkForUpdates() {
    if (!isTauri()) return

    isChecking.value = true
    error.value = ''

    try {
      const update = await check()

      if (update) {
        updateAvailable.value = true
        updateVersion.value = update.version
        updateBody.value = update.body || ''
        updateInstance.value = update
      } else {
        updateAvailable.value = false
        updateInstance.value = null
      }
    } catch (e) {
      console.warn('[updater] Check failed:', e)
      error.value = e.toString()
    } finally {
      isChecking.value = false
    }
  }

  async function checkForUpdatesManual() {
    updateAvailable.value = false
    updateVersion.value = ''
    updateBody.value = ''
    error.value = ''
    updateInstance.value = null
    await checkForUpdates()
  }

  function formatBytes(bytes) {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`
  }

  async function downloadAndInstall() {
    if (!updateInstance.value) return

    isDownloading.value = true
    downloadProgress.value = 0
    downloadedBytes.value = 0
    totalBytes.value = 0
    error.value = ''

    try {
      let downloaded = 0

      await updateInstance.value.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            totalBytes.value = event.data.contentLength || 0
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            downloadedBytes.value = downloaded
            if (totalBytes.value > 0) {
              downloadProgress.value = Math.round((downloaded / totalBytes.value) * 100)
            }
            break
          case 'Finished':
            downloadProgress.value = 100
            downloadedBytes.value = totalBytes.value
            break
        }
      })

      await relaunch()
    } catch (e) {
      console.error('[updater] Download failed:', e)
      error.value = e.toString()
      isDownloading.value = false
    }
  }

  function dismiss() {
    updateAvailable.value = false
    updateInstance.value = null
    error.value = ''
  }

  return {
    updateAvailable,
    updateVersion,
    updateBody,
    isChecking,
    isDownloading,
    downloadProgress,
    downloadedBytes,
    totalBytes,
    error,
    appVersion,
    loadAppVersion,
    checkForUpdates,
    checkForUpdatesManual,
    downloadAndInstall,
    formatBytes,
    dismiss
  }
}
