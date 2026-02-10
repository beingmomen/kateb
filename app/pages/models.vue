<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { t } = useI18n()
const { availableModels, activeModel, isDownloading, downloadingModelId, downloadProgress, downloadedBytes, totalBytes, getAvailableModels, getActiveModel, downloadModel, setActiveModel, deleteModel, formatBytes } = useModels()
const toast = useToast()

const deleteTargetId = ref(null)
const showDeleteConfirm = ref(false)
const isDeleting = ref(false)
const isActivating = ref(false)

onMounted(async () => {
  await Promise.all([getAvailableModels(), getActiveModel()])
})

async function handleDownload(modelId) {
  try {
    await downloadModel(modelId)
    toast.add({
      title: t('models.downloadSuccess'),
      icon: 'i-lucide-check',
      color: 'success'
    })
  } catch (e) {
    toast.add({
      title: t('models.downloadFailed'),
      description: e.toString(),
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  }
}

async function handleActivate(modelId) {
  isActivating.value = true
  try {
    await setActiveModel(modelId)
    toast.add({
      title: t('models.activateSuccess'),
      icon: 'i-lucide-check',
      color: 'success'
    })
  } catch (e) {
    toast.add({
      title: t('models.activateFailed'),
      description: e.toString(),
      icon: 'i-lucide-alert-circle',
      color: 'error'
    })
  } finally {
    isActivating.value = false
  }
}

function confirmDelete(modelId) {
  deleteTargetId.value = modelId
  showDeleteConfirm.value = true
}

async function handleDelete() {
  isDeleting.value = true
  try {
    await deleteModel(deleteTargetId.value)
    showDeleteConfirm.value = false
    deleteTargetId.value = null
    toast.add({
      title: t('models.deleteSuccess'),
      icon: 'i-lucide-check',
      color: 'success'
    })
  } catch (e) {
    toast.add({
      title: t('models.deleteFailed'),
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

function accuracyStars(level) {
  return '\u2605'.repeat(level) + '\u2606'.repeat(5 - level)
}

function speedLabel(level) {
  const labels = ['', t('welcome.speedVerySlow'), t('welcome.speedSlow'), t('welcome.speedMedium'), t('welcome.speedFast'), t('welcome.speedVeryFast')]
  return labels[level] || ''
}

function isActive(modelId) {
  return activeModel.value?.id === modelId
}
</script>

<template>
  <UDashboardPanel id="models">
    <template #header>
      <UDashboardNavbar :title="$t('models.title')">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="max-w-5xl mx-auto space-y-6 py-6 px-4">
        <div
          v-if="activeModel"
          class="flex items-center gap-3 p-4 bg-primary-50 dark:bg-primary-900/20 rounded-lg"
        >
          <UIcon
            name="i-lucide-check-circle"
            class="size-5 text-primary-500"
          />
          <span class="text-sm">
            {{ $t('models.activeModel') }}
            <strong>{{ activeModel.name }}</strong>
          </span>
        </div>

        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          <UCard
            v-for="m in availableModels"
            :key="m.id"
            :class="[
              'relative transition-all',
              isActive(m.id) ? 'ring-2 ring-primary-500' : '',
              isDownloading && downloadingModelId === m.id ? 'ring-2 ring-blue-500' : ''
            ]"
          >
            <div
              v-if="isActive(m.id)"
              class="absolute -top-2 left-1/2 -translate-x-1/2 bg-primary-500 text-white text-xs px-3 py-0.5 rounded-full"
            >
              {{ $t('common.active') }}
            </div>

            <div
              v-else-if="m.recommended"
              class="absolute -top-2 left-1/2 -translate-x-1/2 bg-amber-500 text-white text-xs px-3 py-0.5 rounded-full"
            >
              {{ $t('common.recommended') }}
            </div>

            <div class="space-y-3 pt-2">
              <div class="text-center">
                <h3 class="font-bold">
                  {{ m.name }}
                </h3>
                <p class="text-xs text-muted mt-1">
                  {{ m.description_ar }}
                </p>
              </div>

              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span class="text-muted">{{ $t('common.size') }}</span>
                  <span class="font-medium">{{ m.size_display }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted">{{ $t('common.accuracy') }}</span>
                  <span class="text-amber-500">{{ accuracyStars(m.accuracy) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted">{{ $t('common.speed') }}</span>
                  <span class="font-medium">{{ speedLabel(m.speed) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-muted">{{ $t('common.memory') }}</span>
                  <span class="font-medium">{{ m.ram_mb }} MB</span>
                </div>
              </div>

              <div class="space-y-1">
                <div
                  v-for="pro in m.pros_ar"
                  :key="pro"
                  class="flex items-center gap-1 text-xs text-green-600 dark:text-green-400"
                >
                  <UIcon
                    name="i-lucide-check"
                    class="size-3"
                  />
                  <span>{{ pro }}</span>
                </div>
                <div
                  v-for="con in m.cons_ar"
                  :key="con"
                  class="flex items-center gap-1 text-xs text-red-500 dark:text-red-400"
                >
                  <UIcon
                    name="i-lucide-x"
                    class="size-3"
                  />
                  <span>{{ con }}</span>
                </div>
              </div>

              <template v-if="isDownloading && downloadingModelId === m.id">
                <div class="space-y-2">
                  <UProgress
                    :model-value="downloadProgress"
                    color="primary"
                    size="sm"
                  />
                  <p class="text-xs text-muted text-center">
                    {{ progressText }} ({{ Math.round(downloadProgress) }}%)
                  </p>
                </div>
              </template>

              <template v-else-if="m.installed">
                <div class="flex gap-2">
                  <UButton
                    v-if="!isActive(m.id)"
                    block
                    size="sm"
                    icon="i-lucide-play"
                    :loading="isActivating"
                    @click="handleActivate(m.id)"
                  >
                    {{ $t('common.activate') }}
                  </UButton>
                  <UButton
                    v-else
                    block
                    size="sm"
                    color="primary"
                    variant="soft"
                    icon="i-lucide-check"
                    disabled
                  >
                    {{ $t('common.activated') }}
                  </UButton>
                  <UButton
                    size="sm"
                    color="error"
                    variant="soft"
                    icon="i-lucide-trash-2"
                    :disabled="isActive(m.id)"
                    @click="confirmDelete(m.id)"
                  />
                </div>
              </template>

              <template v-else>
                <UButton
                  block
                  size="sm"
                  icon="i-lucide-download"
                  :disabled="isDownloading"
                  @click="handleDownload(m.id)"
                >
                  {{ $t('models.downloadSize', { size: m.size_display }) }}
                </UButton>
              </template>
            </div>
          </UCard>
        </div>

        <p class="text-xs text-muted text-center">
          {{ $t('models.modelsFromHf') }}
        </p>
      </div>
    </template>
    <UModal v-model:open="showDeleteConfirm">
      <template #content>
        <UCard>
          <template #header>
            <div class="flex items-center gap-2 text-red-600 dark:text-red-400">
              <UIcon
                name="i-lucide-alert-triangle"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('models.confirmDelete') }}</span>
            </div>
          </template>

          <p>
            {{ $t('models.confirmDeleteMsg') }}
          </p>

          <template #footer>
            <div class="flex justify-end gap-2">
              <UButton
                variant="ghost"
                @click="showDeleteConfirm = false"
              >
                {{ $t('common.cancel') }}
              </UButton>
              <UButton
                color="error"
                :loading="isDeleting"
                @click="handleDelete"
              >
                {{ $t('common.delete') }}
              </UButton>
            </div>
          </template>
        </UCard>
      </template>
    </UModal>
  </UDashboardPanel>
</template>
