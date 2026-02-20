<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { t, setLocale } = useI18n()
const { fetchSettings, updateSetting, getSettingValue } = useSettings()
const { providers, isTestingConnection, getProviders, testSpecificProvider, detectGpu } = useAI()
const { getActiveModel, reloadModel } = useModels()
const {
  updateAvailable: settingsUpdateAvailable,
  updateVersion: settingsUpdateVersion,
  isChecking: settingsIsChecking,
  isDownloading: settingsIsDownloading,
  downloadProgress: settingsDownloadProgress,
  downloadedBytes: settingsDownloadedBytes,
  totalBytes: settingsTotalBytes,
  error: settingsUpdateError,
  appVersion: settingsAppVersion,
  loadAppVersion: settingsLoadAppVersion,
  checkForUpdatesManual: settingsCheckForUpdates,
  downloadAndInstall: settingsDownloadAndInstall,
  formatBytes: settingsFormatBytes
} = useUpdater()
const toast = useToast()

const activeModel = ref(null)
const isSaving = ref(false)
const hasChanges = ref(false)
const gpuAvailable = ref(false)
const audioDevices = ref([])
const selectedAudioDevice = ref('default')

const form = reactive({
  shortcut: 'Z+Z',
  language: 'ar',
  auto_punctuation: true,
  sound_notifications: true,
  auto_start: false,
  auto_type: true,
  max_recording_duration: 300,
  ai_refinement: false,
  ai_provider: 'local',
  claude_api_key: '',
  openai_api_key: '',
  gemini_api_key: '',
  grok_api_key: '',
  local_api_key: '',
  claude_api_url: '',
  openai_api_url: '',
  gemini_api_url: '',
  grok_api_url: '',
  local_api_url: '',
  use_gpu: false,
  auto_stop_silence: false,
  auto_stop_seconds: 10,
  custom_vocabulary: '',
  noise_suppression: false,
  voice_commands: true
})

const original = reactive({ ...form })

function loadFormFromSettings() {
  const rawShortcut = getSettingValue('shortcut', 'Z+Z')
  const cleanShortcut = rawShortcut.replace(/\s/g, '').replace(/^"(.*)"$/, '$1')
  const predefined = ['Z+Z', 'Ctrl+Shift+D', 'Ctrl+Shift+R', 'Alt+S']
  if (predefined.includes(cleanShortcut)) {
    form.shortcut = cleanShortcut
  } else {
    form.shortcut = 'custom'
    customShortcutDisplay.value = cleanShortcut
  }
  form.language = getSettingValue('language', 'ar')
  form.auto_punctuation = getSettingValue('auto_punctuation', true)
  form.sound_notifications = getSettingValue('sound_notifications', true)
  form.auto_start = getSettingValue('auto_start', false)
  form.auto_type = getSettingValue('auto_type', true)
  form.max_recording_duration = getSettingValue('max_recording_duration', 300)
  form.ai_refinement = getSettingValue('ai_refinement', false)
  form.ai_provider = getSettingValue('ai_provider', 'local')
  form.claude_api_key = getSettingValue('claude_api_key', '')
  form.openai_api_key = getSettingValue('openai_api_key', '')
  form.gemini_api_key = getSettingValue('gemini_api_key', '')
  form.grok_api_key = getSettingValue('grok_api_key', '')
  form.local_api_key = getSettingValue('local_api_key', '')
  form.claude_api_url = getSettingValue('claude_api_url', '')
  form.openai_api_url = getSettingValue('openai_api_url', '')
  form.gemini_api_url = getSettingValue('gemini_api_url', '')
  form.grok_api_url = getSettingValue('grok_api_url', '')
  form.local_api_url = getSettingValue('local_api_url', '')
  const gpuVal = getSettingValue('use_gpu', false)
  form.use_gpu = gpuVal === true || gpuVal === 'true'
  const autoStopVal = getSettingValue('auto_stop_silence', false)
  form.auto_stop_silence = autoStopVal === true || autoStopVal === 'true'
  form.auto_stop_seconds = Number(getSettingValue('auto_stop_seconds', 10))
  form.custom_vocabulary = getSettingValue('custom_vocabulary', '')
  const noiseSup = getSettingValue('noise_suppression', true)
  form.noise_suppression = noiseSup === true || noiseSup === 'true'
  const voiceCmd = getSettingValue('voice_commands', true)
  form.voice_commands = voiceCmd === true || voiceCmd === 'true'
  Object.assign(original, form)
  original._actualShortcut = form.shortcut === 'custom' ? customShortcutDisplay.value : form.shortcut
}

watch(form, () => {
  hasChanges.value = JSON.stringify(form) !== JSON.stringify(original)
}, { deep: true })

async function loadAudioDevices() {
  try {
    const devices = await tauriInvoke('get_audio_devices')
    audioDevices.value = devices || []
    const savedDevice = getSettingValue('audio_device', '')
    selectedAudioDevice.value = savedDevice || 'default'
  } catch { /* ignore */ }
}

async function handleDeviceChange(deviceName) {
  selectedAudioDevice.value = deviceName
  try {
    await tauriInvoke('set_audio_device', { deviceName: deviceName === 'default' ? null : deviceName })
  } catch { /* ignore */ }
}

onMounted(async () => {
  await fetchSettings()
  await getProviders()
  loadFormFromSettings()
  settingsLoadAppVersion()
  try {
    activeModel.value = await getActiveModel()
  } catch { /* ignore */ }
  try {
    const gpu = await detectGpu()
    gpuAvailable.value = gpu.cuda_available
    if (!gpuAvailable.value) {
      form.use_gpu = false
    }
  } catch { /* ignore */ }
  await loadAudioDevices()
})

const isRecordingShortcut = ref(false)
const customShortcutDisplay = ref('')

const shortcutOptions = computed(() => [
  { label: t('settings.shortcutZZ'), value: 'Z+Z' },
  { label: 'Ctrl + Shift + D', value: 'Ctrl+Shift+D' },
  { label: 'Ctrl + Shift + R', value: 'Ctrl+Shift+R' },
  { label: 'Alt + S', value: 'Alt+S' },
  { label: t('settings.shortcutCustom'), value: 'custom' }
])

function handleShortcutChange(val) {
  if (val === 'custom') {
    isRecordingShortcut.value = false
    customShortcutDisplay.value = ''
  }
}

function startRecording() {
  isRecordingShortcut.value = true
  customShortcutDisplay.value = t('settings.shortcutRecording')
}

function handleKeyCapture(event) {
  if (!isRecordingShortcut.value) return
  event.preventDefault()

  const key = event.key
  if (['Control', 'Shift', 'Alt', 'Meta'].includes(key)) return

  const parts = []
  if (event.ctrlKey) parts.push('Ctrl')
  if (event.shiftKey) parts.push('Shift')
  if (event.altKey) parts.push('Alt')
  if (event.metaKey) parts.push('Meta')
  parts.push(key.length === 1 ? key.toUpperCase() : key)

  if (parts.length < 2) {
    customShortcutDisplay.value = t('settings.shortcutModifierRequired')
    return
  }

  const shortcut = parts.join('+')
  customShortcutDisplay.value = shortcut
  form.shortcut = shortcut
  isRecordingShortcut.value = false
}

const languageOptions = computed(() => [
  { label: t('settings.langArabic'), value: 'ar' },
  { label: t('common.english'), value: 'en' }
])

const durationOptions = computed(() => [
  { label: t('settings.duration1m'), value: 60 },
  { label: t('settings.duration3m'), value: 180 },
  { label: t('settings.duration5m'), value: 300 },
  { label: t('settings.duration10m'), value: 600 }
])

const autoStopOptions = computed(() => [
  { label: t('settings.silence3s'), value: 3 },
  { label: t('settings.silence5s'), value: 5 },
  { label: t('settings.silence10s'), value: 10 },
  { label: t('settings.silence15s'), value: 15 }
])

const providerOptions = computed(() => {
  return providers.value.map(p => ({
    label: p.name,
    value: p.id
  }))
})

const currentApiKey = computed(() => {
  switch (form.ai_provider) {
    case 'claude':
      return form.claude_api_key
    case 'openai':
      return form.openai_api_key
    case 'gemini':
      return form.gemini_api_key
    case 'grok':
      return form.grok_api_key
    case 'local':
      return form.local_api_key
    default:
      return ''
  }
})

const currentApiUrl = computed(() => {
  switch (form.ai_provider) {
    case 'claude':
      return form.claude_api_url
    case 'openai':
      return form.openai_api_url
    case 'gemini':
      return form.gemini_api_url
    case 'grok':
      return form.grok_api_url
    case 'local':
      return form.local_api_url
    default:
      return ''
  }
})

const defaultUrlPlaceholder = computed(() => {
  switch (form.ai_provider) {
    case 'claude':
      return 'https://api.anthropic.com'
    case 'openai':
      return 'https://api.openai.com'
    case 'gemini':
      return 'https://generativelanguage.googleapis.com'
    case 'grok':
      return 'https://api.x.ai'
    case 'local':
      return 'http://localhost:8000'
    default:
      return ''
  }
})

async function testConnection() {
  const result = await testSpecificProvider(form.ai_provider, currentApiKey.value, currentApiUrl.value)
  if (result.success) {
    toast.add({
      title: result.message,
      icon: 'i-lucide-check',
      color: 'success'
    })
  } else {
    toast.add({
      title: t('common.connectionFailed'),
      description: result.message,
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  }
}

const isExporting = ref(false)
const isImporting = ref(false)

async function handleExport() {
  isExporting.value = true
  try {
    const json = await tauriInvoke('export_settings')
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `kateb-settings-${new Date().toISOString().slice(0, 10)}.json`
    a.click()
    URL.revokeObjectURL(url)
    toast.add({
      title: t('settings.exportSuccess'),
      icon: 'i-lucide-check',
      color: 'success'
    })
  } catch (e) {
    toast.add({
      title: t('settings.exportError'),
      description: String(e),
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  } finally {
    isExporting.value = false
  }
}

async function handleImport() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'
  input.onchange = async (e) => {
    const file = e.target.files?.[0]
    if (!file) return

    isImporting.value = true
    try {
      const text = await file.text()
      JSON.parse(text)
      const count = await tauriInvoke('import_settings', { json: text })
      toast.add({
        title: t('settings.importSuccess', { count }),
        icon: 'i-lucide-check',
        color: 'success'
      })
      await fetchSettings()
      loadFormFromSettings()
    } catch (e) {
      toast.add({
        title: t('settings.importError'),
        description: String(e),
        icon: 'i-lucide-alert-circle',
        color: 'error'
      })
    } finally {
      isImporting.value = false
    }
  }
  input.click()
}

async function handleSave() {
  isSaving.value = true
  try {
    const gpuChanged = String(form.use_gpu) !== String(original.use_gpu)
    const actualShortcut = form.shortcut === 'custom' ? customShortcutDisplay.value : form.shortcut
    const shortcutChanged = actualShortcut !== (original._actualShortcut || original.shortcut)

    const settingsMap = {
      shortcut: actualShortcut,
      language: form.language,
      auto_punctuation: String(form.auto_punctuation),
      sound_notifications: String(form.sound_notifications),
      auto_start: String(form.auto_start),
      auto_type: String(form.auto_type),
      max_recording_duration: String(form.max_recording_duration),
      ai_refinement: String(form.ai_refinement),
      ai_provider: form.ai_provider,
      claude_api_key: form.claude_api_key,
      openai_api_key: form.openai_api_key,
      gemini_api_key: form.gemini_api_key,
      grok_api_key: form.grok_api_key,
      local_api_key: form.local_api_key,
      claude_api_url: form.claude_api_url,
      openai_api_url: form.openai_api_url,
      gemini_api_url: form.gemini_api_url,
      grok_api_url: form.grok_api_url,
      local_api_url: form.local_api_url,
      use_gpu: String(form.use_gpu),
      auto_stop_silence: String(form.auto_stop_silence),
      auto_stop_seconds: String(form.auto_stop_seconds),
      custom_vocabulary: form.custom_vocabulary,
      noise_suppression: String(form.noise_suppression),
      voice_commands: String(form.voice_commands)
    }

    for (const [key, value] of Object.entries(settingsMap)) {
      await updateSetting(key, value)
    }

    if (shortcutChanged) {
      try {
        await tauriInvoke('update_shortcut', { shortcut: actualShortcut })
      } catch { /* ignore */ }
    }

    if (gpuChanged) {
      try {
        await reloadModel()
        toast.add({
          title: t('settings.savedWithReload'),
          icon: 'i-lucide-check',
          color: 'success'
        })
      } catch {
        toast.add({
          title: t('settings.savedSuccess'),
          description: t('settings.savedNoReload'),
          icon: 'i-lucide-alert-triangle',
          color: 'warning'
        })
      }
    } else {
      toast.add({
        title: t('settings.savedSuccess'),
        icon: 'i-lucide-check',
        color: 'success'
      })
    }

    await setLocale(form.language)

    Object.assign(original, form)
    hasChanges.value = false
  } catch (e) {
    toast.add({
      title: t('settings.saveError'),
      description: String(e),
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  } finally {
    isSaving.value = false
  }
}
</script>

<template>
  <UDashboardPanel id="settings">
    <template #header>
      <UDashboardNavbar :title="$t('settings.title')">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
        <template #trailing>
          <UButton
            icon="i-lucide-save"
            :loading="isSaving"
            :disabled="!hasChanges"
            @click="handleSave"
          >
            {{ $t('settings.saveSettings') }}
          </UButton>
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="max-w-2xl mx-auto space-y-8 py-6">
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-keyboard"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.shortcutSection') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <UFormField :label="$t('settings.shortcutLabel')">
              <USelect
                v-model="form.shortcut"
                :items="shortcutOptions"
                value-key="value"
                @update:model-value="handleShortcutChange"
              />
            </UFormField>

            <div
              v-if="form.shortcut === 'custom'"
              class="space-y-3"
            >
              <div class="flex gap-2">
                <UInput
                  :model-value="customShortcutDisplay || $t('settings.shortcutPlaceholder')"
                  icon="i-lucide-command"
                  readonly
                  class="flex-1"
                  :class="isRecordingShortcut ? 'ring-2 ring-primary-500' : ''"
                  @keydown="handleKeyCapture"
                />
                <UButton
                  :variant="isRecordingShortcut ? 'solid' : 'soft'"
                  :color="isRecordingShortcut ? 'error' : 'primary'"
                  :icon="isRecordingShortcut ? 'i-lucide-circle-stop' : 'i-lucide-circle-dot'"
                  @click="isRecordingShortcut ? (isRecordingShortcut = false) : startRecording()"
                >
                  {{ isRecordingShortcut ? $t('common.cancel') : $t('settings.shortcutRecord') }}
                </UButton>
              </div>
              <p class="text-xs text-muted">
                {{ $t('settings.shortcutHint') }}
              </p>
            </div>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-languages"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.langSection') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <UFormField :label="$t('settings.langLabel')">
              <USelect
                v-model="form.language"
                :items="languageOptions"
                value-key="value"
              />
            </UFormField>

            <UFormField :label="$t('settings.maxDuration')">
              <USelect
                v-model="form.max_recording_duration"
                :items="durationOptions"
                value-key="value"
              />
            </UFormField>

            <UFormField :label="$t('settings.whisperModel')">
              <UInput
                :model-value="activeModel?.name || $t('settings.noModelSelected')"
                icon="i-lucide-brain"
                readonly
                disabled
              />
            </UFormField>

            <UFormField :label="$t('settings.micDevice')">
              <div class="flex gap-2">
                <USelect
                  :model-value="selectedAudioDevice"
                  :items="[
                    { label: $t('settings.micDefault'), value: 'default' },
                    ...audioDevices.map(d => ({ label: d.name + (d.is_default ? ` (${$t('settings.micDefaultTag')})` : ''), value: d.name }))
                  ]"
                  value-key="value"
                  class="flex-1"
                  @update:model-value="handleDeviceChange"
                />
                <UButton
                  variant="soft"
                  icon="i-lucide-refresh-cw"
                  @click="loadAudioDevices"
                />
              </div>
            </UFormField>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-book-open"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.vocabSection') }}</span>
            </div>
          </template>
          <div class="space-y-4">
            <UFormField :label="$t('settings.vocabLabel')">
              <UTextarea
                v-model="form.custom_vocabulary"
                :placeholder="$t('settings.vocabPlaceholder')"
                :rows="3"
              />
              <p class="text-xs text-muted mt-1">
                {{ $t('settings.vocabHint') }}
              </p>
            </UFormField>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-cpu"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.perfSection') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.gpuLabel') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.gpuDesc') }}
                </p>
              </div>
              <USwitch
                v-model="form.use_gpu"
                :disabled="!gpuAvailable"
              />
            </div>

            <div
              v-if="!gpuAvailable"
              class="bg-gray-50 dark:bg-gray-800 text-gray-600 dark:text-gray-400 p-3 rounded-lg text-sm flex items-start gap-2"
            >
              <UIcon
                name="i-lucide-info"
                class="size-4 mt-0.5 shrink-0"
              />
              <span>{{ $t('settings.gpuUnavailable') }}</span>
            </div>

            <div
              v-else-if="form.use_gpu"
              class="bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-200 p-3 rounded-lg text-sm flex items-start gap-2"
            >
              <UIcon
                name="i-lucide-check-circle"
                class="size-4 mt-0.5 shrink-0"
              />
              <span>{{ $t('settings.gpuEnabled') }}</span>
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.noiseSuppression') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.noiseSuppressionDesc') }}
                </p>
              </div>
              <USwitch v-model="form.noise_suppression" />
            </div>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-sliders-horizontal"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.generalSection') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.autoPunctuation') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.autoPunctuationDesc') }}
                </p>
              </div>
              <USwitch v-model="form.auto_punctuation" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.autoType') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.autoTypeDesc') }}
                </p>
              </div>
              <USwitch v-model="form.auto_type" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.soundNotifications') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.soundNotificationsDesc') }}
                </p>
              </div>
              <USwitch v-model="form.sound_notifications" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.autoStart') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.autoStartDesc') }}
                </p>
              </div>
              <USwitch v-model="form.auto_start" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.autoStopSilence') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.autoStopSilenceDesc') }}
                </p>
              </div>
              <USwitch v-model="form.auto_stop_silence" />
            </div>

            <UFormField
              v-if="form.auto_stop_silence"
              :label="$t('settings.silenceDuration')"
            >
              <USelect
                v-model="form.auto_stop_seconds"
                :items="autoStopOptions"
                value-key="value"
              />
            </UFormField>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.voiceCommands') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.voiceCommandsDesc') }}
                </p>
              </div>
              <USwitch v-model="form.voice_commands" />
            </div>
          </div>
        </UCard>

        <UCard v-if="form.voice_commands">
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-terminal"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.vcGuideTitle') }}</span>
            </div>
          </template>

          <div class="space-y-5">
            <p class="text-sm text-muted">
              {{ $t('settings.vcGuideDesc') }}
            </p>

            <div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
              <div class="grid grid-cols-[1fr_auto_1fr] bg-gray-50 dark:bg-gray-800 px-4 py-2.5 text-sm font-semibold">
                <span>{{ $t('settings.vcCommand') }}</span>
                <span class="text-center px-4">{{ $t('settings.vcSayThis') }}</span>
                <span class="text-end">{{ $t('settings.vcResult') }}</span>
              </div>

              <div class="divide-y divide-gray-200 dark:divide-gray-700">
                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-wrap-text"
                      class="size-4 text-blue-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcNewLine') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcNewLine') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcNewLineResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-pilcrow"
                      class="size-4 text-blue-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcNewParagraph') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcNewParagraph') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcNewParagraphResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-circle-dot"
                      class="size-4 text-green-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcPeriod') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcPeriod') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcPeriodResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-comma"
                      class="size-4 text-green-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcComma') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcComma') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcCommaResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-circle-help"
                      class="size-4 text-amber-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcQuestion') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcQuestion') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcQuestionResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-circle-alert"
                      class="size-4 text-amber-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcExclamation') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcExclamation') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcExclamationResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-space"
                      class="size-4 text-gray-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcSpace') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcSpace') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcSpaceResult') }}</span>
                </div>

                <div class="grid grid-cols-[1fr_auto_1fr] px-4 py-3 text-sm items-center">
                  <div class="flex items-center gap-2">
                    <UIcon
                      name="i-lucide-delete"
                      class="size-4 text-red-500 shrink-0"
                    />
                    <span class="font-medium">{{ $t('settings.vcDelete') }}</span>
                  </div>
                  <UBadge
                    variant="subtle"
                    color="neutral"
                    class="mx-4"
                  >
                    {{ $t('settings.vcDelete') }}
                  </UBadge>
                  <span class="text-muted text-end">{{ $t('settings.vcDeleteResult') }}</span>
                </div>
              </div>
            </div>

            <div class="bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-200 p-3 rounded-lg text-sm flex items-start gap-2">
              <UIcon
                name="i-lucide-lightbulb"
                class="size-4 mt-0.5 shrink-0"
              />
              <span>{{ $t('settings.vcTip') }}</span>
            </div>

            <div class="bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 p-3 rounded-lg text-sm space-y-2">
              <div class="flex items-start gap-2">
                <UIcon
                  name="i-lucide-message-square-quote"
                  class="size-4 mt-0.5 shrink-0"
                />
                <span>{{ $t('settings.vcExample') }}</span>
              </div>
              <div class="bg-white/50 dark:bg-gray-800/50 rounded px-3 py-2 font-mono text-xs whitespace-pre-line">{{ $t('settings.vcExampleResult') }}</div>
            </div>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-sparkles"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('settings.aiSection') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.aiEnable') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.aiEnableDesc') }}
                </p>
              </div>
              <USwitch v-model="form.ai_refinement" />
            </div>

            <template v-if="form.ai_refinement">
              <USeparator />

              <UFormField :label="$t('settings.aiProvider')">
                <USelect
                  v-model="form.ai_provider"
                  :items="providerOptions"
                  value-key="value"
                />
              </UFormField>

              <UFormField
                v-if="form.ai_provider === 'claude'"
                :label="$t('settings.aiKeyLabel', { provider: 'Claude' })"
              >
                <UInput
                  v-model="form.claude_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="sk-ant-..."
                />
              </UFormField>

              <UFormField
                v-else-if="form.ai_provider === 'openai'"
                :label="$t('settings.aiKeyLabel', { provider: 'OpenAI' })"
              >
                <UInput
                  v-model="form.openai_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="sk-..."
                />
              </UFormField>

              <UFormField
                v-else-if="form.ai_provider === 'gemini'"
                :label="$t('settings.aiKeyLabel', { provider: 'Gemini' })"
              >
                <UInput
                  v-model="form.gemini_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="AIza..."
                />
              </UFormField>

              <UFormField
                v-else-if="form.ai_provider === 'grok'"
                :label="$t('settings.aiKeyLabel', { provider: 'Grok' })"
              >
                <UInput
                  v-model="form.grok_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="xai-..."
                />
              </UFormField>

              <UFormField :label="$t('settings.aiCustomUrl')">
                <UInput
                  v-if="form.ai_provider === 'claude'"
                  v-model="form.claude_api_url"
                  icon="i-lucide-globe"
                  :placeholder="defaultUrlPlaceholder"
                />
                <UInput
                  v-else-if="form.ai_provider === 'openai'"
                  v-model="form.openai_api_url"
                  icon="i-lucide-globe"
                  :placeholder="defaultUrlPlaceholder"
                />
                <UInput
                  v-else-if="form.ai_provider === 'gemini'"
                  v-model="form.gemini_api_url"
                  icon="i-lucide-globe"
                  :placeholder="defaultUrlPlaceholder"
                />
                <UInput
                  v-else-if="form.ai_provider === 'grok'"
                  v-model="form.grok_api_url"
                  icon="i-lucide-globe"
                  :placeholder="defaultUrlPlaceholder"
                />
                <UInput
                  v-else-if="form.ai_provider === 'local'"
                  v-model="form.local_api_url"
                  icon="i-lucide-globe"
                  :placeholder="defaultUrlPlaceholder"
                />
                <p class="text-xs text-muted mt-1">
                  {{ $t('settings.aiCustomUrlHint') }}
                </p>
              </UFormField>

              <UFormField
                v-if="form.ai_provider === 'local' && form.local_api_url"
                :label="$t('settings.aiLocalKey')"
              >
                <UInput
                  v-model="form.local_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="skip"
                />
                <p class="text-xs text-muted mt-1">
                  {{ $t('settings.aiLocalKeyHint') }}
                </p>
              </UFormField>

              <UButton
                variant="soft"
                icon="i-lucide-plug"
                :loading="isTestingConnection"
                @click="testConnection"
              >
                {{ $t('settings.testConnection') }}
              </UButton>
            </template>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-hard-drive"
                class="text-lg"
              />
              <h3 class="font-semibold">
                {{ $t('settings.backupSection') }}
              </h3>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.exportSettings') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.exportDesc') }}
                </p>
              </div>
              <UButton
                variant="soft"
                icon="i-lucide-download"
                :loading="isExporting"
                @click="handleExport"
              >
                {{ $t('settings.exportSettings') }}
              </UButton>
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('settings.importSettings') }}
                </p>
                <p class="text-sm text-muted">
                  {{ $t('settings.importDesc') }}
                </p>
              </div>
              <UButton
                variant="soft"
                icon="i-lucide-upload"
                :loading="isImporting"
                @click="handleImport"
              >
                {{ $t('settings.importSettings') }}
              </UButton>
            </div>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-download"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('updater.updateSection') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">
                  {{ $t('updater.currentVersion') }}
                </p>
                <p class="text-sm text-muted">
                  v{{ settingsAppVersion || '...' }}
                </p>
              </div>
              <UButton
                variant="soft"
                icon="i-lucide-refresh-cw"
                :loading="settingsIsChecking"
                @click="settingsCheckForUpdates"
              >
                {{ $t('updater.checkNow') }}
              </UButton>
            </div>

            <USeparator v-if="settingsUpdateAvailable || settingsUpdateError" />

            <div v-if="settingsUpdateAvailable && !settingsIsDownloading && !settingsUpdateError">
              <div class="bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 p-3 rounded-lg text-sm flex items-start gap-2">
                <UIcon
                  name="i-lucide-download"
                  class="size-4 mt-0.5 shrink-0"
                />
                <div class="flex-1">
                  <p class="font-medium">
                    {{ $t('updater.newVersion', { version: settingsUpdateVersion }) }}
                  </p>
                  <p class="mt-1">
                    {{ $t('updater.clickToInstall') }}
                  </p>
                </div>
              </div>
              <UButton
                class="mt-3"
                icon="i-lucide-download"
                @click="settingsDownloadAndInstall"
              >
                {{ $t('updater.updateNow') }}
              </UButton>
            </div>

            <div v-else-if="settingsIsDownloading">
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span>{{ $t('updater.downloading', { progress: settingsDownloadProgress }) }}</span>
                  <span
                    v-if="settingsTotalBytes > 0"
                    class="text-muted"
                  >
                    {{ settingsFormatBytes(settingsDownloadedBytes) }} / {{ settingsFormatBytes(settingsTotalBytes) }}
                  </span>
                </div>
                <UProgress
                  :model-value="settingsDownloadProgress"
                  :max="100"
                  size="sm"
                  :color="settingsDownloadProgress >= 100 ? 'success' : 'primary'"
                />
                <p
                  v-if="settingsDownloadProgress >= 100"
                  class="text-sm text-muted"
                >
                  {{ $t('updater.installing') }}
                </p>
              </div>
            </div>

            <div v-else-if="settingsUpdateError">
              <div class="bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-200 p-3 rounded-lg text-sm flex items-start gap-2">
                <UIcon
                  name="i-lucide-alert-circle"
                  class="size-4 mt-0.5 shrink-0"
                />
                <div class="flex-1">
                  <p class="font-medium">
                    {{ $t('updater.checkFailed') }}
                  </p>
                  <p class="mt-1 text-xs break-all">
                    {{ settingsUpdateError }}
                  </p>
                </div>
              </div>
              <UButton
                class="mt-3"
                variant="soft"
                icon="i-lucide-refresh-cw"
                @click="settingsCheckForUpdates"
              >
                {{ $t('updater.retry') }}
              </UButton>
            </div>

            <div v-else-if="!settingsIsChecking && !settingsUpdateAvailable && !settingsUpdateError">
              <div class="bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-200 p-3 rounded-lg text-sm flex items-start gap-2">
                <UIcon
                  name="i-lucide-shield-check"
                  class="size-4 mt-0.5 shrink-0"
                />
                <div>
                  <p class="font-medium">
                    {{ $t('updater.upToDate') }}
                  </p>
                  <p class="mt-0.5">
                    {{ $t('updater.upToDateDesc') }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </UCard>

        <div
          v-if="hasChanges"
          class="sticky bottom-4 flex justify-center"
        >
          <UButton
            size="lg"
            icon="i-lucide-save"
            :loading="isSaving"
            class="shadow-lg"
            @click="handleSave"
          >
            {{ $t('settings.saveSettings') }}
          </UButton>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
