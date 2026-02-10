<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { fetchSettings, updateSetting, getSettingValue } = useSettings()
const { providers, isTestingConnection, getProviders, testSpecificProvider, detectGpu } = useAI()
const { getActiveModel, reloadModel } = useModels()
const toast = useToast()

const activeModel = ref(null)
const isSaving = ref(false)
const hasChanges = ref(false)
const gpuAvailable = ref(false)
const audioDevices = ref([])
const selectedAudioDevice = ref('')

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
  auto_stop_silence: true,
  auto_stop_seconds: 5
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
  const autoStopVal = getSettingValue('auto_stop_silence', true)
  form.auto_stop_silence = autoStopVal === true || autoStopVal === 'true'
  form.auto_stop_seconds = Number(getSettingValue('auto_stop_seconds', 5))
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
    selectedAudioDevice.value = savedDevice || ''
  } catch {}
}

async function handleDeviceChange(deviceName) {
  selectedAudioDevice.value = deviceName
  try {
    await tauriInvoke('set_audio_device', { deviceName: deviceName || null })
  } catch {}
}

onMounted(async () => {
  await fetchSettings()
  await getProviders()
  loadFormFromSettings()
  try {
    activeModel.value = await getActiveModel()
  } catch {}
  try {
    const gpu = await detectGpu()
    gpuAvailable.value = gpu.cuda_available
    if (!gpuAvailable.value) {
      form.use_gpu = false
    }
  } catch {}
  await loadAudioDevices()
})

const isRecordingShortcut = ref(false)
const customShortcutDisplay = ref('')

const shortcutOptions = [
  { label: 'Z + Z (ضغط مزدوج)', value: 'Z+Z' },
  { label: 'Ctrl + Shift + D', value: 'Ctrl+Shift+D' },
  { label: 'Ctrl + Shift + R', value: 'Ctrl+Shift+R' },
  { label: 'Alt + S', value: 'Alt+S' },
  { label: 'مخصص...', value: 'custom' }
]

function handleShortcutChange(val) {
  if (val === 'custom') {
    isRecordingShortcut.value = false
    customShortcutDisplay.value = ''
  }
}

function startRecording() {
  isRecordingShortcut.value = true
  customShortcutDisplay.value = 'اضغط الاختصار المطلوب...'
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
    customShortcutDisplay.value = 'يجب استخدام مفتاح تعديل (Ctrl, Shift, Alt) مع مفتاح آخر'
    return
  }

  const shortcut = parts.join('+')
  customShortcutDisplay.value = shortcut
  form.shortcut = shortcut
  isRecordingShortcut.value = false
}

const languageOptions = [
  { label: 'العربية', value: 'ar' },
  { label: 'English', value: 'en' }
]

const durationOptions = [
  { label: '١ دقيقة', value: 60 },
  { label: '٣ دقائق', value: 180 },
  { label: '٥ دقائق', value: 300 },
  { label: '١٠ دقائق', value: 600 }
]

const autoStopOptions = [
  { label: '٣ ثواني', value: 3 },
  { label: '٥ ثواني', value: 5 },
  { label: '١٠ ثواني', value: 10 },
  { label: '١٥ ثانية', value: 15 }
]

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
      title: 'فشل الاتصال',
      description: result.message,
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  }
}

async function handleSave() {
  isSaving.value = true
  try {
    const gpuChanged = String(form.use_gpu) !== String(original.use_gpu)
    const actualShortcut = form.shortcut === 'custom' ? customShortcutDisplay.value : form.shortcut
    const shortcutChanged = actualShortcut !== (original._actualShortcut || original.shortcut)

    const settingsMap = {
      shortcut: actualShortcut,
      language: JSON.stringify(form.language),
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
      auto_stop_seconds: String(form.auto_stop_seconds)
    }

    for (const [key, value] of Object.entries(settingsMap)) {
      await updateSetting(key, value)
    }

    if (shortcutChanged) {
      try {
        await tauriInvoke('update_shortcut', { shortcut: actualShortcut })
      } catch {}
    }

    if (gpuChanged) {
      try {
        await reloadModel()
        toast.add({
          title: 'تم حفظ الإعدادات وإعادة تحميل النموذج',
          icon: 'i-lucide-check',
          color: 'success'
        })
      } catch {
        toast.add({
          title: 'تم حفظ الإعدادات',
          description: 'لم يتم إعادة تحميل النموذج - تأكد من وجود نموذج نشط',
          icon: 'i-lucide-alert-triangle',
          color: 'warning'
        })
      }
    } else {
      toast.add({
        title: 'تم حفظ الإعدادات',
        icon: 'i-lucide-check',
        color: 'success'
      })
    }

    Object.assign(original, form)
    hasChanges.value = false
  } catch (e) {
    toast.add({
      title: 'خطأ في حفظ الإعدادات',
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
      <UDashboardNavbar title="الإعدادات">
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
            حفظ الإعدادات
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
              <span class="font-semibold">اختصار لوحة المفاتيح</span>
            </div>
          </template>

          <div class="space-y-4">
            <UFormField label="اختصار بدء/إيقاف الإملاء">
              <USelect
                v-model="form.shortcut"
                :items="shortcutOptions"
                value-key="value"
                @update:model-value="handleShortcutChange"
              />
            </UFormField>

            <div v-if="form.shortcut === 'custom'" class="space-y-3">
              <div class="flex gap-2">
                <UInput
                  :model-value="customShortcutDisplay || 'اضغط تسجيل ثم اختصارك'"
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
                  {{ isRecordingShortcut ? 'إلغاء' : 'تسجيل' }}
                </UButton>
              </div>
              <p class="text-xs text-muted">اضغط "تسجيل" ثم اضغط الاختصار المطلوب (مثلاً: Ctrl + Shift + M)</p>
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
              <span class="font-semibold">اللغة والنموذج</span>
            </div>
          </template>

          <div class="space-y-4">
            <UFormField label="لغة الإملاء">
              <USelect
                v-model="form.language"
                :items="languageOptions"
                value-key="value"
              />
            </UFormField>

            <UFormField label="مدة التسجيل القصوى">
              <USelect
                v-model="form.max_recording_duration"
                :items="durationOptions"
                value-key="value"
              />
            </UFormField>

            <UFormField label="نموذج Whisper">
              <UInput
                :model-value="activeModel?.name || 'لم يتم تحديد نموذج'"
                icon="i-lucide-brain"
                readonly
                disabled
              />
            </UFormField>

            <UFormField label="جهاز الميكروفون">
              <div class="flex gap-2">
                <USelect
                  :model-value="selectedAudioDevice"
                  :items="[
                    { label: 'الافتراضي', value: '' },
                    ...audioDevices.map(d => ({ label: d.name + (d.is_default ? ' (افتراضي)' : ''), value: d.name }))
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
                name="i-lucide-cpu"
                class="size-5"
              />
              <span class="font-semibold">الأداء</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">استخدام GPU (كرت الشاشة)</p>
                <p class="text-sm text-muted">تسريع المعالجة باستخدام NVIDIA CUDA - يحتاج كرت شاشة NVIDIA</p>
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
              <span>لم يتم اكتشاف كرت شاشة NVIDIA يدعم CUDA على جهازك. هذا الخيار غير متاح.</span>
            </div>

            <div
              v-else-if="form.use_gpu"
              class="bg-green-50 dark:bg-green-900/20 text-green-800 dark:text-green-200 p-3 rounded-lg text-sm flex items-start gap-2"
            >
              <UIcon
                name="i-lucide-check-circle"
                class="size-4 mt-0.5 shrink-0"
              />
              <span>تم اكتشاف كرت شاشة NVIDIA يدعم CUDA. سيتم استخدام GPU لتسريع المعالجة.</span>
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
              <span class="font-semibold">خيارات عامة</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">علامات الترقيم التلقائية</p>
                <p class="text-sm text-muted">إضافة علامات الترقيم تلقائيًا للنص</p>
              </div>
              <USwitch v-model="form.auto_punctuation" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">الكتابة التلقائية</p>
                <p class="text-sm text-muted">كتابة النص مباشرة في البرنامج النشط</p>
              </div>
              <USwitch v-model="form.auto_type" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">إشعارات صوتية</p>
                <p class="text-sm text-muted">تشغيل صوت عند بدء وإيقاف الإملاء</p>
              </div>
              <USwitch v-model="form.sound_notifications" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">التشغيل التلقائي</p>
                <p class="text-sm text-muted">تشغيل التطبيق عند بدء النظام</p>
              </div>
              <USwitch v-model="form.auto_start" />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">إيقاف تلقائي عند الصمت</p>
                <p class="text-sm text-muted">إيقاف التسجيل تلقائياً عند انتهاء الكلام</p>
              </div>
              <USwitch v-model="form.auto_stop_silence" />
            </div>

            <UFormField
              v-if="form.auto_stop_silence"
              label="مدة الصمت قبل الإيقاف"
            >
              <USelect
                v-model="form.auto_stop_seconds"
                :items="autoStopOptions"
                value-key="value"
              />
            </UFormField>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-sparkles"
                class="size-5"
              />
              <span class="font-semibold">تحسين النص بالذكاء الاصطناعي</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">تفعيل التحسين بالذكاء الاصطناعي</p>
                <p class="text-sm text-muted">تصحيح الأخطاء وإضافة علامات الترقيم تلقائياً</p>
              </div>
              <USwitch v-model="form.ai_refinement" />
            </div>

            <template v-if="form.ai_refinement">
              <USeparator />

              <UFormField label="مزود الذكاء الاصطناعي">
                <USelect
                  v-model="form.ai_provider"
                  :items="providerOptions"
                  value-key="value"
                />
              </UFormField>

              <UFormField
                v-if="form.ai_provider === 'claude'"
                label="مفتاح Claude API"
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
                label="مفتاح OpenAI API"
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
                label="مفتاح Gemini API"
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
                label="مفتاح Grok API"
              >
                <UInput
                  v-model="form.grok_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="xai-..."
                />
              </UFormField>

              <UFormField label="عنوان API مخصص (اختياري)">
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
                <p class="text-xs text-muted mt-1">أدخل الدومين فقط (مثال: https://your-domain.com) — المسار يُضاف تلقائياً</p>
              </UFormField>

              <UFormField
                v-if="form.ai_provider === 'local' && form.local_api_url"
                label="مفتاح API (اختياري)"
              >
                <UInput
                  v-model="form.local_api_key"
                  type="password"
                  icon="i-lucide-key"
                  placeholder="skip أو اتركه فارغاً"
                />
                <p class="text-xs text-muted mt-1">إذا كان السيرفر يتطلب مفتاح، أدخله هنا (مثل: skip)</p>
              </UFormField>

              <UButton
                variant="soft"
                icon="i-lucide-plug"
                :loading="isTestingConnection"
                @click="testConnection"
              >
                اختبار الاتصال
              </UButton>
            </template>
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
            حفظ الإعدادات
          </UButton>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
