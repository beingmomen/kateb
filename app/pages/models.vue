<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { modelExists, modelInfo, isDownloading, downloadProgress, downloadedBytes, totalBytes, error, isLoading, getModelInfo, downloadModel, deleteModel, formatBytes } = useModels()
const toast = useToast()

const showDeleteConfirm = ref(false)
const isDeleting = ref(false)

onMounted(() => {
  getModelInfo()
})

async function handleDownload() {
  try {
    await downloadModel()
    await getModelInfo()
    toast.add({
      title: 'تم تحميل النموذج بنجاح',
      icon: 'i-lucide-check',
      color: 'success'
    })
  } catch (e) {
    toast.add({
      title: 'فشل تحميل النموذج',
      description: e.toString(),
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  }
}

async function handleDelete() {
  isDeleting.value = true
  try {
    await deleteModel()
    await getModelInfo()
    showDeleteConfirm.value = false
    toast.add({
      title: 'تم حذف النموذج',
      icon: 'i-lucide-check',
      color: 'success'
    })
  } catch (e) {
    toast.add({
      title: 'فشل حذف النموذج',
      description: e.toString(),
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  } finally {
    isDeleting.value = false
  }
}

const progressText = computed(() => {
  if (!isDownloading.value) return ''
  return `${formatBytes(downloadedBytes.value)} / ${formatBytes(totalBytes.value)}`
})
</script>

<template>
  <UDashboardPanel id="models">
    <template #header>
      <UDashboardNavbar title="إدارة النموذج">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="max-w-2xl mx-auto space-y-6 py-6">
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-brain"
                class="size-5"
              />
              <span class="font-semibold">نموذج Whisper Large V3 Turbo</span>
            </div>
          </template>

          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-muted">الحالة</span>
              <UBadge
                :color="modelExists ? 'success' : 'warning'"
                :label="modelExists ? 'مُثبّت' : 'غير مُثبّت'"
              />
            </div>

            <template v-if="modelInfo">
              <div class="flex items-center justify-between">
                <span class="text-muted">الحجم المتوقع</span>
                <span>{{ formatBytes(modelInfo.expected_size) }}</span>
              </div>

              <div
                v-if="modelExists"
                class="flex items-center justify-between"
              >
                <span class="text-muted">الحجم الحالي</span>
                <span>{{ formatBytes(modelInfo.size) }}</span>
              </div>

              <div class="flex items-center justify-between">
                <span class="text-muted">المسار</span>
                <span
                  class="text-xs text-muted truncate max-w-[250px]"
                  dir="ltr"
                >{{ modelInfo.path }}</span>
              </div>
            </template>
          </div>

          <template #footer>
            <div class="flex justify-end gap-2">
              <template v-if="modelExists">
                <UButton
                  color="error"
                  variant="soft"
                  icon="i-lucide-trash-2"
                  @click="showDeleteConfirm = true"
                >
                  حذف النموذج
                </UButton>
              </template>
              <template v-else>
                <UButton
                  icon="i-lucide-download"
                  :loading="isDownloading"
                  @click="handleDownload"
                >
                  {{ isDownloading ? 'جاري التحميل...' : 'تحميل النموذج' }}
                </UButton>
              </template>
            </div>
          </template>
        </UCard>

        <template v-if="isDownloading">
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-download"
                  class="size-5"
                />
                <span class="font-semibold">جاري التحميل</span>
              </div>
            </template>

            <div class="space-y-4">
              <UProgress
                :value="downloadProgress"
                color="primary"
                size="lg"
              />
              <div class="flex justify-between text-sm text-muted">
                <span>{{ progressText }}</span>
                <span>{{ Math.round(downloadProgress) }}%</span>
              </div>
            </div>
          </UCard>
        </template>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-info"
                class="size-5"
              />
              <span class="font-semibold">معلومات النموذج</span>
            </div>
          </template>

          <div class="space-y-3 text-sm text-muted">
            <p>
              <strong>Whisper Large V3 Turbo</strong> هو نموذج للتعرف على الكلام من OpenAI.
            </p>
            <p>
              يوفر دقة عالية في تحويل الكلام العربي إلى نص مع سرعة محسّنة.
            </p>
            <p>
              يتم تحميل النموذج من HuggingFace ويُحفظ محلياً على جهازك.
            </p>
          </div>
        </UCard>
      </div>
    </template>
  </UDashboardPanel>

  <UModal v-model:open="showDeleteConfirm">
    <template #content>
      <UCard>
        <template #header>
          <div class="flex items-center gap-2 text-red-600 dark:text-red-400">
            <UIcon
              name="i-lucide-alert-triangle"
              class="size-5"
            />
            <span class="font-semibold">تأكيد الحذف</span>
          </div>
        </template>

        <p>
          هل أنت متأكد من حذف نموذج Whisper؟ ستحتاج إلى إعادة تحميله لاستخدام التطبيق.
        </p>

        <template #footer>
          <div class="flex justify-end gap-2">
            <UButton
              variant="ghost"
              @click="showDeleteConfirm = false"
            >
              إلغاء
            </UButton>
            <UButton
              color="error"
              :loading="isDeleting"
              @click="handleDelete"
            >
              حذف
            </UButton>
          </div>
        </template>
      </UCard>
    </template>
  </UModal>
</template>
