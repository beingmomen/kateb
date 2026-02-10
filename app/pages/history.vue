<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { t, locale } = useI18n()
const { history, loading, fetchHistory, deleteItem, clearAll } = useHistory()
const toast = useToast()
const searchQuery = ref('')
const showClearModal = ref(false)

await fetchHistory()

async function handleSearch() {
  await fetchHistory(searchQuery.value || null)
}

watch(searchQuery, (val) => {
  if (!val) fetchHistory()
})

async function handleCopy(text) {
  await navigator.clipboard.writeText(text)
  toast.add({ title: t('common.copied'), icon: 'i-lucide-check' })
}

async function handleDelete(id) {
  await deleteItem(id)
  toast.add({ title: t('common.deleted'), icon: 'i-lucide-trash-2' })
}

async function handleClearAll() {
  await clearAll()
  showClearModal.value = false
  toast.add({ title: t('history.allCleared'), icon: 'i-lucide-trash-2' })
}

function formatDate(dateStr) {
  const date = new Date(dateStr)
  return new Intl.DateTimeFormat(locale.value, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(date)
}

function formatDuration(seconds) {
  if (seconds < 60) return `${seconds} ${t('common.sec')}`
  return `${Math.floor(seconds / 60)} ${t('common.min')}`
}

function formatProcessingTime(ms) {
  if (!ms) return ''
  if (ms < 1000) return `${ms} ${t('common.ms')}`
  return `${(ms / 1000).toFixed(1)} ${t('common.sec')}`
}

function getProviderLabel(provider) {
  const map = {
    claude: 'Claude',
    openai: 'OpenAI',
    gemini: 'Gemini',
    grok: 'Grok',
    local: t('common.local')
  }
  return map[provider] || provider
}
</script>

<template>
  <UDashboardPanel id="history">
    <template #header>
      <UDashboardNavbar :title="$t('history.title')">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>

        <template #right>
          <UButton
            v-if="history.length"
            icon="i-lucide-trash-2"
            color="error"
            variant="ghost"
            size="sm"
            :label="$t('common.deleteAll')"
            @click="showClearModal = true"
          />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="space-y-4 py-4">
        <UInput
          v-model="searchQuery"
          icon="i-lucide-search"
          :placeholder="$t('history.searchPlaceholder')"
          size="lg"
          @keyup.enter="handleSearch"
        />

        <div
          v-if="loading"
          class="flex justify-center py-12"
        >
          <UIcon
            name="i-lucide-loader-2"
            class="size-8 animate-spin text-muted"
          />
        </div>

        <div
          v-else-if="!history.length"
          class="text-center py-12"
        >
          <UIcon
            name="i-lucide-inbox"
            class="size-12 text-muted mx-auto mb-4"
          />
          <p class="text-muted">
            {{ $t('history.noHistory') }}
          </p>
        </div>

        <div
          v-else
          class="space-y-3"
        >
          <UCard
            v-for="item in history"
            :key="item.id"
          >
            <div class="flex items-start justify-between gap-4">
              <div class="flex-1 min-w-0">
                <p class="text-base leading-relaxed whitespace-pre-wrap">
                  {{ item.text }}
                </p>

                <UCollapsible
                  v-if="item.raw_text"
                  class="mt-2"
                >
                  <UButton
                    :label="$t('history.originalText')"
                    color="neutral"
                    variant="ghost"
                    size="xs"
                    icon="i-lucide-file-text"
                    trailing-icon="i-lucide-chevron-down"
                  />
                  <template #content>
                    <div class="mt-1 p-3 bg-neutral-50 dark:bg-neutral-900 rounded-lg text-sm text-muted whitespace-pre-wrap">
                      {{ item.raw_text }}
                    </div>
                  </template>
                </UCollapsible>

                <div class="flex items-center gap-3 mt-2 text-sm text-muted flex-wrap">
                  <span class="flex items-center gap-1">
                    <UIcon
                      name="i-lucide-calendar"
                      class="size-3.5"
                    />
                    {{ formatDate(item.created_at) }}
                  </span>
                  <UBadge
                    :label="item.language === 'ar' ? $t('common.arabic') : 'English'"
                    variant="subtle"
                    size="xs"
                  />
                  <span class="flex items-center gap-1">
                    <UIcon
                      name="i-lucide-mic"
                      class="size-3.5"
                    />
                    {{ formatDuration(item.duration) }}
                  </span>
                  <UBadge
                    v-if="item.ai_provider"
                    :label="getProviderLabel(item.ai_provider)"
                    color="primary"
                    variant="subtle"
                    size="xs"
                    icon="i-lucide-sparkles"
                  />
                  <span
                    v-if="item.processing_time_ms"
                    class="flex items-center gap-1"
                  >
                    <UIcon
                      name="i-lucide-timer"
                      class="size-3.5"
                    />
                    {{ formatProcessingTime(item.processing_time_ms) }}
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-1 shrink-0">
                <UButton
                  icon="i-lucide-copy"
                  color="neutral"
                  variant="ghost"
                  size="xs"
                  @click="handleCopy(item.text)"
                />
                <UButton
                  icon="i-lucide-trash-2"
                  color="error"
                  variant="ghost"
                  size="xs"
                  @click="handleDelete(item.id)"
                />
              </div>
            </div>
          </UCard>
        </div>
      </div>

      <UModal v-model:open="showClearModal">
        <template #content>
          <div class="p-6 text-center">
            <UIcon
              name="i-lucide-alert-triangle"
              class="size-12 text-error mx-auto mb-4"
            />
            <h3 class="text-lg font-semibold mb-2">
              {{ $t('history.clearAll') }}
            </h3>
            <p class="text-muted mb-6">
              {{ $t('history.clearAllConfirm') }}
            </p>
            <div class="flex justify-center gap-3">
              <UButton
                :label="$t('common.cancel')"
                color="neutral"
                variant="outline"
                @click="showClearModal = false"
              />
              <UButton
                :label="$t('common.deleteAll')"
                color="error"
                @click="handleClearAll"
              />
            </div>
          </div>
        </template>
      </UModal>
    </template>
  </UDashboardPanel>
</template>
