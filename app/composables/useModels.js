import { tauriInvoke, tauriListen } from '~/utils/tauri'

export function useModels() {
  const availableModels = ref([])
  const activeModel = ref(null)
  const isDownloading = ref(false)
  const downloadingModelId = ref(null)
  const downloadProgress = ref(0)
  const downloadedBytes = ref(0)
  const totalBytes = ref(0)
  const error = ref(null)
  const isLoading = ref(false)

  async function getAvailableModels() {
    isLoading.value = true
    try {
      const models = await tauriInvoke('get_available_models')
      availableModels.value = models || []
      return availableModels.value
    } catch (e) {
      error.value = e
      return []
    } finally {
      isLoading.value = false
    }
  }

  async function getActiveModel() {
    try {
      activeModel.value = await tauriInvoke('get_active_model')
      return activeModel.value
    } catch (e) {
      error.value = e
      return null
    }
  }

  async function hasActiveModel() {
    try {
      return await tauriInvoke('has_active_model')
    } catch {
      return false
    }
  }

  async function downloadModel(modelId) {
    isDownloading.value = true
    downloadingModelId.value = modelId
    downloadProgress.value = 0
    downloadedBytes.value = 0
    error.value = null
    try {
      const path = await tauriInvoke('download_specific_model', { modelId })
      await getAvailableModels()
      return path
    } catch (e) {
      error.value = e
      throw e
    } finally {
      isDownloading.value = false
      downloadingModelId.value = null
    }
  }

  async function setActiveModel(modelId) {
    try {
      await tauriInvoke('set_active_model', { modelId })
      await getActiveModel()
    } catch (e) {
      error.value = e
      throw e
    }
  }

  async function deleteModel(modelId) {
    try {
      await tauriInvoke('delete_model', { modelId })
      await getAvailableModels()
      await getActiveModel()
    } catch (e) {
      error.value = e
      throw e
    }
  }

  async function reloadModel() {
    try {
      await tauriInvoke('reload_model')
    } catch (e) {
      error.value = e
      throw e
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
  }

  let unlistenProgress = null
  let unlistenStatus = null

  async function setupListeners() {
    unlistenProgress = await tauriListen('model-download-progress', (event) => {
      downloadProgress.value = event.payload.progress
      downloadedBytes.value = event.payload.downloaded
      totalBytes.value = event.payload.total
    })

    unlistenStatus = await tauriListen('model-download-status', (event) => {
      if (event.payload.status === 'started') {
        isDownloading.value = true
        downloadProgress.value = 0
      } else if (event.payload.status === 'completed') {
        downloadProgress.value = 100
      } else if (event.payload.status === 'error') {
        error.value = event.payload.message
      }
    })
  }

  function cleanupListeners() {
    if (unlistenProgress) unlistenProgress()
    if (unlistenStatus) unlistenStatus()
  }

  onMounted(setupListeners)
  onUnmounted(cleanupListeners)

  return {
    availableModels,
    activeModel,
    isDownloading,
    downloadingModelId,
    downloadProgress,
    downloadedBytes,
    totalBytes,
    error,
    isLoading,
    getAvailableModels,
    getActiveModel,
    hasActiveModel,
    downloadModel,
    setActiveModel,
    deleteModel,
    reloadModel,
    formatBytes
  }
}
