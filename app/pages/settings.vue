<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { settings, loading, fetchSettings, updateSetting, getSettingValue } = useSettings()
const toast = useToast()

onMounted(() => {
  fetchSettings()
})

const shortcut = computed({
  get: () => getSettingValue('shortcut', 'Ctrl+Shift+D'),
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
                model-value="Whisper Large V3"
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
      </div>
    </template>
  </UDashboardPanel>
</template>
