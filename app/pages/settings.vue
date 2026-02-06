<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { settings, loading, fetchSettings, updateSetting, getSettingValue } = useSettings()
const { providers, isTestingConnection, connectionStatus, getProviders, testSpecificProvider } = useAI()
const toast = useToast()

onMounted(async () => {
  await fetchSettings()
  await getProviders()
})

const shortcut = computed({
  get: () => getSettingValue('shortcut', 'Z + Z'),
  set: (val) => updateSetting('shortcut', JSON.stringify(val))
})

const language = computed({
  get: () => getSettingValue('language', 'ar'),
  set: (val) => updateSetting('language', JSON.stringify(val))
})

const autoPunctuation = computed({
  get: () => getSettingValue('auto_punctuation', true),
  set: (val) => updateSetting('auto_punctuation', String(val))
})

const soundNotifications = computed({
  get: () => getSettingValue('sound_notifications', true),
  set: (val) => updateSetting('sound_notifications', String(val))
})

const autoStart = computed({
  get: () => getSettingValue('auto_start', false),
  set: (val) => updateSetting('auto_start', String(val))
})

const autoType = computed({
  get: () => getSettingValue('auto_type', true),
  set: (val) => updateSetting('auto_type', String(val))
})

const maxDuration = computed({
  get: () => getSettingValue('max_recording_duration', 300),
  set: (val) => updateSetting('max_recording_duration', String(val))
})

const aiRefinement = computed({
  get: () => getSettingValue('ai_refinement', false),
  set: (val) => updateSetting('ai_refinement', String(val))
})

const aiProvider = computed({
  get: () => getSettingValue('ai_provider', 'local'),
  set: (val) => updateSetting('ai_provider', val)
})

const claudeApiKey = computed({
  get: () => getSettingValue('claude_api_key', ''),
  set: (val) => updateSetting('claude_api_key', val)
})

const openaiApiKey = computed({
  get: () => getSettingValue('openai_api_key', ''),
  set: (val) => updateSetting('openai_api_key', val)
})

const geminiApiKey = computed({
  get: () => getSettingValue('gemini_api_key', ''),
  set: (val) => updateSetting('gemini_api_key', val)
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

const currentProviderRequiresKey = computed(() => {
  const provider = providers.value.find(p => p.id === aiProvider.value)
  return provider?.requires_key ?? false
})

const currentApiKey = computed(() => {
  switch (aiProvider.value) {
    case 'claude':
      return claudeApiKey.value
    case 'openai':
      return openaiApiKey.value
    case 'gemini':
      return geminiApiKey.value
    default:
      return ''
  }
})

async function testConnection() {
  const result = await testSpecificProvider(aiProvider.value, currentApiKey.value)
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

function handleSave() {
  toast.add({ title: 'تم حفظ الإعدادات', icon: 'i-lucide-check', color: 'success' })
}
</script>

<template>
  <UDashboardPanel id="settings">
    <template #header>
      <UDashboardNavbar title="الإعدادات">
        <template #leading>
          <UDashboardSidebarCollapse />
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
              :model-value="shortcut"
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
                :model-value="language"
                :items="languageOptions"
                value-key="value"
                @update:model-value="language = $event"
              />
            </UFormField>

            <UFormField label="مدة التسجيل القصوى">
              <USelect
                :model-value="maxDuration"
                :items="durationOptions"
                value-key="value"
                @update:model-value="maxDuration = $event"
              />
            </UFormField>

            <UFormField label="نموذج Whisper">
              <UInput
                model-value="Whisper Large V3 Turbo"
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
              <USwitch
                :model-value="autoPunctuation"
                @update:model-value="autoPunctuation = $event"
              />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">الكتابة التلقائية</p>
                <p class="text-sm text-muted">كتابة النص مباشرة في البرنامج النشط</p>
              </div>
              <USwitch
                :model-value="autoType"
                @update:model-value="autoType = $event"
              />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">إشعارات صوتية</p>
                <p class="text-sm text-muted">تشغيل صوت عند بدء وإيقاف الإملاء</p>
              </div>
              <USwitch
                :model-value="soundNotifications"
                @update:model-value="soundNotifications = $event"
              />
            </div>

            <USeparator />

            <div class="flex items-center justify-between">
              <div>
                <p class="font-medium">التشغيل التلقائي</p>
                <p class="text-sm text-muted">تشغيل التطبيق عند بدء النظام</p>
              </div>
              <USwitch
                :model-value="autoStart"
                @update:model-value="autoStart = $event"
              />
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
              <USwitch
                :model-value="aiRefinement"
                @update:model-value="aiRefinement = $event"
              />
            </div>

            <template v-if="aiRefinement">
              <USeparator />

              <UFormField label="مزود الذكاء الاصطناعي">
                <USelect
                  :model-value="aiProvider"
                  :items="providerOptions"
                  value-key="value"
                  @update:model-value="aiProvider = $event"
                />
              </UFormField>

              <template v-if="aiProvider === 'claude'">
                <UFormField label="مفتاح Claude API">
                  <UInput
                    :model-value="claudeApiKey"
                    type="password"
                    icon="i-lucide-key"
                    placeholder="sk-ant-..."
                    @update:model-value="claudeApiKey = $event"
                  />
                </UFormField>
              </template>

              <template v-else-if="aiProvider === 'openai'">
                <UFormField label="مفتاح OpenAI API">
                  <UInput
                    :model-value="openaiApiKey"
                    type="password"
                    icon="i-lucide-key"
                    placeholder="sk-..."
                    @update:model-value="openaiApiKey = $event"
                  />
                </UFormField>
              </template>

              <template v-else-if="aiProvider === 'gemini'">
                <UFormField label="مفتاح Gemini API">
                  <UInput
                    :model-value="geminiApiKey"
                    type="password"
                    icon="i-lucide-key"
                    placeholder="AIza..."
                    @update:model-value="geminiApiKey = $event"
                  />
                </UFormField>
              </template>

              <template v-else-if="aiProvider === 'local'">
                <div class="bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 p-4 rounded-lg text-sm">
                  <p>
                    يتطلب تشغيل خادم محلي على المنفذ <code class="bg-blue-100 dark:bg-blue-800 px-1 rounded">8000</code>
                  </p>
                </div>
              </template>

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
      </div>
    </template>
  </UDashboardPanel>
</template>
