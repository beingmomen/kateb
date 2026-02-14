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

let appWindow = null

function startDrag() {
  if (appWindow) appWindow.startDragging()
}

async function hideOverlay() {
  visible.value = false
  await tauriInvoke('hide_overlay')
}

onMounted(async () => {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    appWindow = getCurrentWindow()
  } catch {}

  await tauriListen('dictation-status', (event) => {
    const { is_recording, is_processing } = event.payload
    isRecording.value = is_recording
    isProcessing.value = is_processing

    if (is_recording) {
      visible.value = true
      lastResult.value = ''
      startDurationTimer()
    } else if (is_processing) {
      stopDurationTimer()
      startProcessingTimer()
    } else {
      stopDurationTimer()
      stopProcessingTimer()
    }
  })

  await tauriListen('ai-refine-status', (event) => {
    if (event.payload.status === 'started') {
      isRefining.value = true
      stopProcessingTimer()
      startRefiningTimer()
    } else if (event.payload.status === 'done' || event.payload.status === 'error') {
      isRefining.value = false
      stopRefiningTimer()
    }
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
    class="bubble"
    data-tauri-drag-region
    @mousedown="startDrag"
  >
    <div :class="['dot', stage]" />

    <span class="label">
      <template v-if="stage === 'recording'">{{ t('home.recording') }}</template>
      <template v-else-if="stage === 'processing'">{{ t('home.processing') }}</template>
      <template v-else-if="stage === 'refining'">{{ t('home.refining') }}</template>
      <template v-else-if="stage === 'done'">{{ t('common.done') || '✓' }}</template>
    </span>

    <span class="timer">
      <template v-if="stage === 'recording'">{{ formatDuration(recordingDuration) }}</template>
      <template v-else-if="stage === 'processing'">{{ formatMs(processingDuration) }}</template>
      <template v-else-if="stage === 'refining'">{{ formatMs(refiningDuration) }}</template>
    </span>

    <button
      class="close"
      @click.stop="hideOverlay"
    >
      ✕
    </button>
  </div>
</template>

<style>
html, body, #__nuxt {
  margin: 0;
  padding: 0;
  background: transparent !important;
  overflow: hidden;
}
</style>

<style scoped>
.bubble {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  font-family: system-ui, -apple-system, sans-serif;
  background: rgba(15, 23, 42, 0.95);
  border-radius: 28px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 10px 18px;
  color: #e2e8f0;
  cursor: grab;
  user-select: none;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
  white-space: nowrap;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
}

.dot.recording {
  background: #ef4444;
  box-shadow: 0 0 8px rgba(239, 68, 68, 0.6);
  animation: pulse 1s ease-in-out infinite;
}

.dot.processing {
  background: #f59e0b;
  animation: spin-dot 1s linear infinite;
}

.dot.refining {
  background: #8b5cf6;
  box-shadow: 0 0 8px rgba(139, 92, 246, 0.6);
  animation: pulse 1.5s ease-in-out infinite;
}

.dot.done {
  background: #10b981;
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.6);
}

.label {
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0.02em;
}

.timer {
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  color: #94a3b8;
  font-weight: 500;
}

.close {
  background: none;
  border: none;
  color: #475569;
  cursor: pointer;
  font-size: 11px;
  padding: 2px 4px;
  border-radius: 50%;
  line-height: 1;
  margin-inline-start: 2px;
  transition: color 0.15s, background 0.15s;
}

.close:hover {
  color: #e2e8f0;
  background: rgba(255, 255, 255, 0.1);
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

@keyframes spin-dot {
  0% { box-shadow: 2px 0 0 0 #f59e0b; }
  25% { box-shadow: 0 2px 0 0 #f59e0b; }
  50% { box-shadow: -2px 0 0 0 #f59e0b; }
  75% { box-shadow: 0 -2px 0 0 #f59e0b; }
  100% { box-shadow: 2px 0 0 0 #f59e0b; }
}
</style>
