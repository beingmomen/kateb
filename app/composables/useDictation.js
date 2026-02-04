import { tauriInvoke, tauriListen } from '~/utils/tauri'

export function useDictation() {
  const isRecording = ref(false)
  const isProcessing = ref(false)
  const lastResult = ref('')
  const streamingText = ref('')
  const partialChunks = ref([])
  const error = ref(null)

  async function startDictation() {
    try {
      error.value = null
      streamingText.value = ''
      partialChunks.value = []
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
  let unlistenPartial = null

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

    unlistenPartial = await tauriListen('dictation-partial', (event) => {
      const { text, is_final } = event.payload
      if (is_final) {
        lastResult.value = text
        streamingText.value = ''
        partialChunks.value = []
      } else {
        partialChunks.value.push(text)
        streamingText.value = partialChunks.value.join(' ')
      }
    })
  })

  onUnmounted(() => {
    if (unlistenStatus) unlistenStatus()
    if (unlistenResult) unlistenResult()
    if (unlistenToggle) unlistenToggle()
    if (unlistenPartial) unlistenPartial()
  })

  return {
    isRecording,
    isProcessing,
    lastResult,
    streamingText,
    error,
    startDictation,
    stopDictation,
    toggleDictation,
    getStatus
  }
}
