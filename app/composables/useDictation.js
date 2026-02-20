import { tauriInvoke, tauriListen } from '~/utils/tauri'

const MAX_AUDIO_LEVELS = 50

export function useDictation() {
  const isRecording = ref(false)
  const isProcessing = ref(false)
  const isRefining = ref(false)
  const lastResult = ref('')
  const streamingText = ref('')
  const refiningText = ref('')
  const partialChunks = ref([])
  const error = ref(null)
  const isStopping = ref(false)
  const audioLevel = ref(0)
  const audioLevels = ref([])
  const recordingDuration = ref(0)
  const processingDuration = ref(0)
  const refiningDuration = ref(0)
  const silenceCountdown = ref(null)

  let durationInterval = null
  let processingInterval = null
  let refiningInterval = null

  function startDurationTimer() {
    recordingDuration.value = 0
    durationInterval = setInterval(() => {
      recordingDuration.value++
    }, 1000)
  }

  function stopDurationTimer() {
    if (durationInterval) {
      clearInterval(durationInterval)
      durationInterval = null
    }
  }

  function startProcessingTimer() {
    processingDuration.value = 0
    processingInterval = setInterval(() => {
      processingDuration.value += 100
    }, 100)
  }

  function stopProcessingTimer() {
    if (processingInterval) {
      clearInterval(processingInterval)
      processingInterval = null
    }
  }

  function startRefiningTimer() {
    refiningDuration.value = 0
    refiningInterval = setInterval(() => {
      refiningDuration.value += 100
    }, 100)
  }

  function stopRefiningTimer() {
    if (refiningInterval) {
      clearInterval(refiningInterval)
      refiningInterval = null
    }
  }

  async function startDictation() {
    try {
      error.value = null
      streamingText.value = ''
      partialChunks.value = []
      audioLevels.value = []
      audioLevel.value = 0
      refiningDuration.value = 0
      processingDuration.value = 0
      silenceCountdown.value = null
      await tauriInvoke('start_dictation')
      isRecording.value = true
      startDurationTimer()
    } catch (e) {
      error.value = e
    }
  }

  async function stopDictation() {
    if (isStopping.value) return
    isStopping.value = true
    stopDurationTimer()
    isRecording.value = false
    isProcessing.value = true
    silenceCountdown.value = null
    startProcessingTimer()
    try {
      error.value = null
      const text = await Promise.race([
        tauriInvoke('stop_dictation'),
        new Promise((_, reject) =>
          setTimeout(() => reject(new Error('Processing timed out')), 45000)
        )
      ])
      lastResult.value = text
      return text
    } catch (e) {
      error.value = e
    } finally {
      isProcessing.value = false
      isRefining.value = false
      stopProcessingTimer()
      stopRefiningTimer()
      isStopping.value = false
    }
  }

  async function toggleDictation() {
    if (isProcessing.value || isStopping.value) return
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
  let unlistenAudioLevel = null
  let unlistenRefineChunk = null
  let unlistenRefineStatus = null
  let unlistenSilenceCountdown = null
  let unlistenAutoStop = null

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

    unlistenAudioLevel = await tauriListen('audio-level', (event) => {
      audioLevel.value = event.payload.level
      audioLevels.value.push(event.payload.level)
      if (audioLevels.value.length > MAX_AUDIO_LEVELS) {
        audioLevels.value.shift()
      }
    })

    unlistenRefineChunk = await tauriListen('ai-refine-chunk', (event) => {
      refiningText.value = event.payload.accumulated
    })

    unlistenRefineStatus = await tauriListen('ai-refine-status', (event) => {
      if (event.payload.status === 'started') {
        isRefining.value = true
        refiningText.value = ''
        stopProcessingTimer()
        startRefiningTimer()
      } else if (event.payload.status === 'done' || event.payload.status === 'error') {
        isRefining.value = false
        stopRefiningTimer()
      }
    })

    unlistenSilenceCountdown = await tauriListen('silence-countdown', (event) => {
      const { remaining, total } = event.payload
      if (remaining < total - 1.0 && remaining >= 0) {
        silenceCountdown.value = { remaining: Math.ceil(remaining), total }
      } else {
        silenceCountdown.value = null
      }
    })

    unlistenAutoStop = await tauriListen('dictation-auto-stop', () => {
      if (isRecording.value && !isStopping.value) {
        stopDictation()
      }
    })
  })

  let safetyPollTimer = null

  watch(isProcessing, (val) => {
    if (val) {
      const startTime = Date.now()
      safetyPollTimer = setInterval(async () => {
        if (!isProcessing.value) {
          clearInterval(safetyPollTimer)
          safetyPollTimer = null
          return
        }
        if (Date.now() - startTime > 60000) {
          isProcessing.value = false
          isRefining.value = false
          stopProcessingTimer()
          stopRefiningTimer()
          clearInterval(safetyPollTimer)
          safetyPollTimer = null
          return
        }
        try {
          const status = await tauriInvoke('get_dictation_status')
          if (status && !status.is_recording && !status.is_processing) {
            isProcessing.value = false
            isRefining.value = false
            stopProcessingTimer()
            stopRefiningTimer()
            clearInterval(safetyPollTimer)
            safetyPollTimer = null
          }
        } catch {}
      }, 5000)
    } else {
      if (safetyPollTimer) {
        clearInterval(safetyPollTimer)
        safetyPollTimer = null
      }
    }
  })

  onUnmounted(() => {
    stopDurationTimer()
    stopProcessingTimer()
    stopRefiningTimer()
    if (safetyPollTimer) { clearInterval(safetyPollTimer); safetyPollTimer = null }
    if (unlistenStatus) unlistenStatus()
    if (unlistenResult) unlistenResult()
    if (unlistenToggle) unlistenToggle()
    if (unlistenPartial) unlistenPartial()
    if (unlistenAudioLevel) unlistenAudioLevel()
    if (unlistenRefineChunk) unlistenRefineChunk()
    if (unlistenRefineStatus) unlistenRefineStatus()
    if (unlistenSilenceCountdown) unlistenSilenceCountdown()
    if (unlistenAutoStop) unlistenAutoStop()
  })

  const pipelineStage = computed(() => {
    if (isRefining.value) return 'refining'
    if (isProcessing.value) return 'processing'
    if (isRecording.value) return 'recording'
    if (lastResult.value) return 'done'
    return 'idle'
  })

  const sessionActive = computed(() => pipelineStage.value !== 'idle')

  return {
    isRecording,
    isProcessing,
    isRefining,
    lastResult,
    streamingText,
    refiningText,
    error,
    audioLevel,
    audioLevels,
    recordingDuration,
    processingDuration,
    refiningDuration,
    silenceCountdown,
    pipelineStage,
    sessionActive,
    startDictation,
    stopDictation,
    toggleDictation,
    getStatus
  }
}
