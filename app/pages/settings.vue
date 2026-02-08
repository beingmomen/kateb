<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { fetchSettings, updateSetting, getSettingValue } = useSettings()
const { providers, isTestingConnection, getProviders, testSpecificProvider } = useAI()
const { getActiveModel, reloadModel } = useModels()
const toast = useToast()

const activeModel = ref(null)
const isSaving = ref(false)
const hasChanges = ref(false)

const form = reactive({
  shortcut: 'Z + Z',
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
  use_gpu: false
})

const original = reactive({ ...form })

function loadFormFromSettings() {
  form.shortcut = getSettingValue('shortcut', 'Z + Z')
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
  const gpuVal = getSettingValue('use_gpu', false)
  form.use_gpu = gpuVal === true || gpuVal === 'true'
  Object.assign(original, form)
}

watch(form, () => {
  hasChanges.value = JSON.stringify(form) !== JSON.stringify(original)
}, { deep: true })

onMounted(async () => {
  await fetchSettings()
  await getProviders()
  loadFormFromSettings()
  try {
    activeModel.value = await getActiveModel()
  } catch {}
})

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
    default:
      return ''
  }
})

async function testConnection() {
  const result = await testSpecificProvider(form.ai_provider, currentApiKey.value)
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

    const settingsMap = {
      shortcut: JSON.stringify(form.shortcut),
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
      use_gpu: String(form.use_gpu)
    }

    for (const [key, value] of Object.entries(settingsMap)) {
      await updateSetting(key, value)
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

          <UFormField label="اختصار بدء/إيقاف الإملاء">
            <UInput
              :model-value="form.shortcut"
              icon="i-lucide-command"
              readonly
              placeholder="Ctrl+Shift+D"
            />
          </UFormField>
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
              <USwitch v-model="form.use_gpu" />
            </div>

            <div
              v-if="form.use_gpu"
              class="bg-amber-50 dark:bg-amber-900/20 text-amber-800 dark:text-amber-200 p-3 rounded-lg text-sm flex items-start gap-2"
            >
              <UIcon
                name="i-lucide-alert-triangle"
                class="size-4 mt-0.5 shrink-0"
              />
              <span>تأكد من وجود كرت شاشة NVIDIA مع تعريفات CUDA مثبتة. إذا لم يكن لديك، قم بتعطيل هذا الخيار لتجنب الأخطاء.</span>
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

              <div
                v-else-if="form.ai_provider === 'local'"
                class="bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 p-4 rounded-lg text-sm"
              >
                <p>
                  يتطلب تشغيل خادم محلي على المنفذ <code class="bg-blue-100 dark:bg-blue-800 px-1 rounded">8000</code>
                </p>
              </div>

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
