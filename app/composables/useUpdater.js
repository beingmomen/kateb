import { isTauri } from '~/utils/tauri'

export function useUpdater() {
  const updateAvailable = ref(false)
  const updateVersion = ref('')
  const updateBody = ref('')
  const isDownloading = ref(false)
  const downloadProgress = ref(0)
  const updateInstance = ref(null)

  async function checkForUpdates() {
    if (!isTauri()) return

    try {
      const { check } = await import('@tauri-apps/plugin-updater')
      const update = await check()

      if (update) {
        updateAvailable.value = true
        updateVersion.value = update.version
        updateBody.value = update.body || ''
        updateInstance.value = update
      }
    } catch (e) {
      console.warn('[updater] Check failed:', e)
    }
  }

  async function downloadAndInstall() {
    if (!updateInstance.value) return

    isDownloading.value = true
    downloadProgress.value = 0

    try {
      let downloaded = 0
      let contentLength = 0

      await updateInstance.value.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength || 0
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            if (contentLength > 0) {
              downloadProgress.value = Math.round((downloaded / contentLength) * 100)
            }
            break
          case 'Finished':
            downloadProgress.value = 100
            break
        }
      })

      const { relaunch } = await import('@tauri-apps/plugin-process')
      await relaunch()
    } catch (e) {
      console.error('[updater] Download failed:', e)
      isDownloading.value = false
    }
  }

  function dismiss() {
    updateAvailable.value = false
    updateInstance.value = null
  }

  return {
    updateAvailable,
    updateVersion,
    updateBody,
    isDownloading,
    downloadProgress,
    checkForUpdates,
    downloadAndInstall,
    dismiss
  }
}
