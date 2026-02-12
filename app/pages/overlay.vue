<script setup>
import { tauriInvoke, tauriListen } from '~/utils/tauri'

definePageMeta({
  layout: false
})

const { t } = useI18n()

const isRecording = ref(false)
const isProcessing = ref(false)
const isRefining = ref(false)
const lastResult = ref('')
const refiningText = ref('')
const streamingText = ref('')
const audioLevel = ref(0)
const recordingDuration = ref(0)
const processingDuration = ref(0)
const refiningDuration = ref(0)
const visible = ref(true)

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

const stage = computed(() => {
  if (isRefining.value) return 'refining'
  if (isProcessing.value) return 'processing'
  if (isRecording.value) return 'recording'
  if (lastResult.value) return 'done'
  return 'idle'
})

const formatDuration = (seconds) => {
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return `${m}:${String(s).padStart(2, '0')}`
}

const formatMs = (ms) => {
  const seconds = Math.floor(ms / 1000)
  const m = Math.floor(seconds / 60)
  const s = seconds % 60
  return `${m}:${String(s).padStart(2, '0')}`
}

async function hideOverlay() {
  visible.value = false
  await tauriInvoke('hide_overlay')
}

onMounted(async () => {
  await tauriListen('dictation-status', (event) => {
    const { is_recording, is_processing } = event.payload
    isRecording.value = is_recording
    isProcessing.value = is_processing

    if (is_recording) {
      visible.value = true
      lastResult.value = ''
      refiningText.value = ''
      streamingText.value = ''
      startDurationTimer()
    } else if (is_processing) {
      stopDurationTimer()
      startProcessingTimer()
    } else {
      stopDurationTimer()
      stopProcessingTimer()
    }
  })

  await tauriListen('audio-level', (event) => {
    audioLevel.value = event.payload.level
  })

  await tauriListen('dictation-partial', (event) => {
    const { text, is_final } = event.payload
    if (is_final) {
      lastResult.value = text
      streamingText.value = ''
    } else {
      streamingText.value = (streamingText.value ? streamingText.value + ' ' : '') + text
    }
  })

  await tauriListen('ai-refine-status', (event) => {
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

  await tauriListen('ai-refine-chunk', (event) => {
    refiningText.value = event.payload.accumulated
  })

  await tauriListen('dictation-result', (event) => {
    lastResult.value = event.payload.text
    isProcessing.value = false
    isRefining.value = false
    stopProcessingTimer()
    stopRefiningTimer()
  })
})

onUnmounted(() => {
  stopDurationTimer()
  stopProcessingTimer()
  stopRefiningTimer()
})
</script>

<template>
  <div
    v-if="visible && stage !== 'idle'"
    class="overlay-root"
    data-tauri-drag-region
  >
    <div class="overlay-content">
      <div class="overlay-header">
        <div class="overlay-status">
          <div
            v-if="stage === 'recording'"
            class="status-indicator recording"
          />
          <div
            v-else-if="stage === 'processing'"
            class="status-indicator processing"
          />
          <div
            v-else-if="stage === 'refining'"
            class="status-indicator refining"
          />
          <div
            v-else-if="stage === 'done'"
            class="status-indicator done"
          />

          <span class="status-text">
            <template v-if="stage === 'recording'">
              {{ t('home.recording') }}
            </template>
            <template v-else-if="stage === 'processing'">
              {{ t('home.processing') }}
            </template>
            <template v-else-if="stage === 'refining'">
              {{ t('home.refining') }}
            </template>
            <template v-else-if="stage === 'done'">
              {{ t('common.done') || '✓' }}
            </template>
          </span>
        </div>

        <div class="overlay-meta">
          <span
            v-if="stage === 'recording'"
            class="timer"
          >
            {{ formatDuration(recordingDuration) }}
          </span>
          <span
            v-else-if="stage === 'processing'"
            class="timer"
          >
            {{ formatMs(processingDuration) }}
          </span>
          <span
            v-else-if="stage === 'refining'"
            class="timer"
          >
            {{ formatMs(refiningDuration) }}
          </span>

          <button
            class="close-btn"
            @click.stop="hideOverlay"
          >
            ✕
          </button>
        </div>
      </div>

      <div
        v-if="stage === 'recording'"
        class="audio-bar-container"
      >
        <div
          class="audio-bar"
          :style="{ width: Math.min(audioLevel * 100, 100) + '%' }"
        />
      </div>

      <div
        v-if="stage === 'recording' && streamingText"
        class="overlay-text"
      >
        {{ streamingText.length > 80 ? '...' + streamingText.slice(-80) : streamingText }}
      </div>

      <div
        v-if="stage === 'refining' && refiningText"
        class="overlay-text"
      >
        {{ refiningText.length > 80 ? '...' + refiningText.slice(-80) : refiningText }}
      </div>

      <div
        v-if="stage === 'done' && lastResult"
        class="overlay-text done-text"
      >
        {{ lastResult.length > 80 ? lastResult.slice(0, 80) + '...' : lastResult }}
      </div>
    </div>
  </div>
</template>

<style>
html, body {
  margin: 0;
  padding: 0;
  background: transparent !important;
  overflow: hidden;
}
</style>

<style scoped>
.overlay-root {
  font-family: system-ui, -apple-system, sans-serif;
  background: rgba(15, 23, 42, 0.92);
  backdrop-filter: blur(12px);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 10px 14px;
  color: #e2e8f0;
  cursor: grab;
  user-select: none;
  min-width: 280px;
  max-width: 300px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.overlay-content {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.overlay-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.overlay-status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.overlay-meta {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-indicator.recording {
  background: #ef4444;
  animation: pulse 1s ease-in-out infinite;
}

.status-indicator.processing {
  background: #f59e0b;
  animation: spin-dot 1s linear infinite;
}

.status-indicator.refining {
  background: #8b5cf6;
  animation: pulse 1.5s ease-in-out infinite;
}

.status-indicator.done {
  background: #10b981;
}

.status-text {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
}

.timer {
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  color: #94a3b8;
  font-weight: 500;
}

.close-btn {
  background: none;
  border: none;
  color: #64748b;
  cursor: pointer;
  font-size: 12px;
  padding: 2px 4px;
  border-radius: 4px;
  line-height: 1;
  transition: color 0.15s, background 0.15s;
}

.close-btn:hover {
  color: #e2e8f0;
  background: rgba(255, 255, 255, 0.1);
}

.audio-bar-container {
  height: 3px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
  overflow: hidden;
}

.audio-bar {
  height: 100%;
  background: linear-gradient(90deg, #10b981, #ef4444);
  border-radius: 2px;
  transition: width 0.1s ease-out;
}

.overlay-text {
  font-size: 11px;
  color: #94a3b8;
  line-height: 1.4;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  direction: rtl;
}

.done-text {
  color: #10b981;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.4; }
}

@keyframes spin-dot {
  0% { box-shadow: 2px 0 0 0 #f59e0b; }
  25% { box-shadow: 0 2px 0 0 #f59e0b; }
  50% { box-shadow: -2px 0 0 0 #f59e0b; }
  75% { box-shadow: 0 -2px 0 0 #f59e0b; }
  100% { box-shadow: 2px 0 0 0 #f59e0b; }
}
</style>
