<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { isRecording, isProcessing, lastResult, error, toggleDictation } = useDictation()

const statusText = computed(() => {
  if (isProcessing.value) return 'جارٍ المعالجة...'
  if (isRecording.value) return 'جارٍ التسجيل...'
  return 'اضغط للبدء'
})

const statusColor = computed(() => {
  if (isProcessing.value) return 'warning'
  if (isRecording.value) return 'error'
  return 'primary'
})

const toast = useToast()

async function handleCopy() {
  if (!lastResult.value) return
  await navigator.clipboard.writeText(lastResult.value)
  toast.add({ title: 'تم النسخ', icon: 'i-lucide-check' })
}
</script>

<template>
  <UDashboardPanel id="main">
    <template #header>
      <UDashboardNavbar title="الرئيسية">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="flex flex-col items-center justify-center gap-8 py-12">
        <div class="text-center">
          <h1 class="text-3xl font-bold mb-2">إملاء صوتي عربي</h1>
          <p class="text-muted">تحويل الصوت إلى نص باستخدام Whisper</p>
        </div>

        <div class="flex flex-col items-center gap-4">
          <UButton
            :icon="isRecording ? 'i-lucide-mic-off' : 'i-lucide-mic'"
            :color="statusColor"
            :loading="isProcessing"
            size="xl"
            class="rounded-full p-6!"
            @click="toggleDictation"
          />

          <UBadge
            :color="statusColor"
            variant="subtle"
            size="lg"
          >
            {{ statusText }}
          </UBadge>
        </div>

        <div
          v-if="error"
          class="w-full max-w-lg"
        >
          <UAlert
            color="error"
            icon="i-lucide-alert-circle"
            :title="String(error)"
          />
        </div>

        <div
          v-if="lastResult"
          class="w-full max-w-2xl"
        >
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-semibold">آخر نتيجة</span>
                <UButton
                  icon="i-lucide-copy"
                  color="neutral"
                  variant="ghost"
                  size="sm"
                  @click="handleCopy"
                />
              </div>
            </template>

            <p class="text-lg leading-relaxed whitespace-pre-wrap">{{ lastResult }}</p>
          </UCard>
        </div>

        <div class="text-center text-sm text-muted">
          <UKbd>Ctrl</UKbd> + <UKbd>Shift</UKbd> + <UKbd>D</UKbd>
          <span class="mr-2">لبدء/إيقاف الإملاء</span>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
