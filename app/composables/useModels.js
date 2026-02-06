import { tauriInvoke, tauriListen } from '~/utils/tauri'

export function useModels() {
  const modelExists = ref(false)
  const modelInfo = ref(null)
  const isDownloading = ref(false)
  const downloadProgress = ref(0)
  const downloadedBytes = ref(0)
  const totalBytes = ref(0)
  const error = ref(null)
  const isLoading = ref(false)

  async function checkModel() {
    isLoading.value = true
    try {
      modelExists.value = await tauriInvoke('check_model_exists')
      return modelExists.value
    } catch (e) {
      error.value = e
      return false
    } finally {
      isLoading.value = false
    }
  }

  async function getModelInfo() {
    isLoading.value = true
    try {
      modelInfo.value = await tauriInvoke('get_model_info')
      modelExists.value = modelInfo.value?.exists ?? false
      return modelInfo.value
    } catch (e) {
      error.value = e
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function downloadModel() {
    isDownloading.value = true
    downloadProgress.value = 0
    error.value = null
    try {
      const path = await tauriInvoke('download_model')
      modelExists.value = true
      return path
    } catch (e) {
      error.value = e
      throw e
    } finally {
      isDownloading.value = false
    }
  }

  async function deleteModel() {
    try {
      await tauriInvoke('delete_model')
      modelExists.value = false
      modelInfo.value = null
    } catch (e) {
      error.value = e
      throw e
    }
  }

  async function loadModel() {
    try {
      await tauriInvoke('load_model')
    } catch (e) {
      error.value = e
      throw e
    }
  }

  function formatBytes(bytes) {
    if (bytes === 0) return '0 Bytes'
    const k = 1024
    const sizes = ['Bytes', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  let unlistenProgress = null
  let unlistenStatus = null

  onMounted(async () => {
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
        isDownloading.value = false
        downloadProgress.value = 100
        modelExists.value = true
      } else if (event.payload.status === 'error') {
        isDownloading.value = false
        error.value = event.payload.message
      }
    })
  })

  onUnmounted(() => {
    if (unlistenProgress) unlistenProgress()
    if (unlistenStatus) unlistenStatus()
  })

  return {
    modelExists,
    modelInfo,
    isDownloading,
    downloadProgress,
    downloadedBytes,
    totalBytes,
    error,
    isLoading,
    checkModel,
    getModelInfo,
    downloadModel,
    deleteModel,
    loadModel,
    formatBytes
  }
}
