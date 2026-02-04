import { tauriInvoke, tauriListen } from '~/utils/tauri'

export function useDictation() {
  const isRecording = ref(false)
  const isProcessing = ref(false)
  const lastResult = ref('')
  const error = ref(null)

  async function startDictation() {
    try {
      error.value = null
      await tauriInvoke('start_dictation')
      isRecording.value = true
    } catch (e) {
      error.value = e
    }
  }

  async function stopDictation() {
    try {
      error.value = null
      const text = await tauriInvoke('stop_dictation')
      lastResult.value = text
      isRecording.value = false
      isProcessing.value = false
      return text
    } catch (e) {
      error.value = e
      isRecording.value = false
      isProcessing.value = false
    }
  }

  async function toggleDictation() {
    if (isRecording.value) {
      return await stopDictation()
    } else {
      return await startDictation()
    }
  }

  async function getStatus() {
    try {
      const status = await tauriInvoke('get_dictation_status')
      if (status) {
        isRecording.value = status.is_recording
        isProcessing.value = status.is_processing
      }
      return status
    } catch (e) {
      error.value = e
    }
  }

  let unlistenStatus = null
  let unlistenResult = null
  let unlistenToggle = null

  onMounted(async () => {
    unlistenStatus = await tauriListen('dictation-status', (event) => {
      isRecording.value = event.payload.is_recording
      isProcessing.value = event.payload.is_processing
    })

    unlistenResult = await tauriListen('dictation-result', (event) => {
      lastResult.value = event.payload.text
    })

    unlistenToggle = await tauriListen('toggle-dictation', () => {
      if (!isProcessing.value) {
        toggleDictation()
      }
    })
  })

  onUnmounted(() => {
    if (unlistenStatus) unlistenStatus()
    if (unlistenResult) unlistenResult()
    if (unlistenToggle) unlistenToggle()
  })

  return {
    isRecording,
    isProcessing,
    lastResult,
    error,
    startDictation,
    stopDictation,
    toggleDictation,
    getStatus
  }
}
