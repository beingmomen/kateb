<script setup>
definePageMeta({
  layout: false
})

const { modelExists, isDownloading, downloadProgress, downloadedBytes, totalBytes, error, checkModel, downloadModel, formatBytes } = useModels()
const router = useRouter()

const isChecking = ref(true)
const downloadError = ref(null)

onMounted(async () => {
  const exists = await checkModel()
  isChecking.value = false
  if (exists) {
    router.push('/')
  }
})

async function handleDownload() {
  downloadError.value = null
  try {
    await downloadModel()
    router.push('/')
  } catch (e) {
    downloadError.value = e.toString()
  }
}

const progressText = computed(() => {
  if (!isDownloading.value) return ''
  return `${formatBytes(downloadedBytes.value)} / ${formatBytes(totalBytes.value)}`
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100 dark:from-gray-900 dark:to-gray-800 p-6">
    <UCard class="w-full max-w-lg text-center">
      <div class="py-8 space-y-6">
        <div class="flex justify-center">
          <div class="w-20 h-20 bg-primary-500 rounded-2xl flex items-center justify-center">
            <UIcon
              name="i-lucide-mic"
              class="size-10 text-white"
            />
          </div>
        </div>

        <div class="space-y-2">
          <h1 class="text-2xl font-bold">مرحباً بك في إملاء صوتي عربي</h1>
          <p class="text-muted">تحويل الكلام إلى نص باستخدام الذكاء الاصطناعي</p>
        </div>

        <template v-if="isChecking">
          <div class="flex items-center justify-center gap-2 py-4">
            <UIcon
              name="i-lucide-loader-2"
              class="size-5 animate-spin"
            />
            <span>جاري التحقق من النموذج...</span>
          </div>
        </template>

        <template v-else-if="!modelExists">
          <div class="space-y-4 py-4">
            <div class="bg-amber-50 dark:bg-amber-900/20 text-amber-800 dark:text-amber-200 p-4 rounded-lg">
              <div class="flex items-center gap-2 justify-center mb-2">
                <UIcon
                  name="i-lucide-download"
                  class="size-5"
                />
                <span class="font-semibold">مطلوب تحميل النموذج</span>
              </div>
              <p class="text-sm">
                لاستخدام التطبيق، يجب تحميل نموذج Whisper (حوالي 1.6 جيجابايت)
              </p>
            </div>

            <template v-if="isDownloading">
              <div class="space-y-3">
                <UProgress
                  :value="downloadProgress"
                  color="primary"
                  size="lg"
                />
                <p class="text-sm text-muted">{{ progressText }} ({{ Math.round(downloadProgress) }}%)</p>
              </div>
            </template>

            <template v-else>
              <UButton
                size="lg"
                block
                icon="i-lucide-download"
                @click="handleDownload"
              >
                تحميل النموذج
              </UButton>
            </template>

            <div
              v-if="downloadError"
              class="bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-200 p-4 rounded-lg text-sm"
            >
              <UIcon
                name="i-lucide-alert-circle"
                class="size-4 inline ml-1"
              />
              {{ downloadError }}
            </div>
          </div>
        </template>

        <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
          <p class="text-xs text-muted">
            يتم تحميل النموذج من HuggingFace ويُحفظ محلياً
          </p>
        </div>
      </div>
    </UCard>
  </div>
</template>
