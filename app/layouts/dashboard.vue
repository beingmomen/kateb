<script setup>
const { t, locale, setLocale } = useI18n()
const {
  updateAvailable, updateVersion, isDownloading,
  downloadProgress, downloadedBytes, totalBytes, error,
  appVersion, loadAppVersion, checkForUpdates, downloadAndInstall,
  formatBytes, dismiss
} = useUpdater()

function toggleLocale() {
  setLocale(locale.value === 'ar' ? 'en' : 'ar')
}

const items = computed(() => [[
  {
    label: t('nav.home'),
    icon: 'i-lucide-mic',
    to: '/'
  },
  {
    label: t('nav.history'),
    icon: 'i-lucide-history',
    to: '/history'
  },
  {
    label: t('nav.stats'),
    icon: 'i-lucide-bar-chart-3',
    to: '/stats'
  }
], [
  {
    label: t('nav.models'),
    icon: 'i-lucide-brain',
    to: '/models'
  },
  {
    label: t('nav.settings'),
    icon: 'i-lucide-settings',
    to: '/settings'
  },
  {
    label: t('nav.support'),
    icon: 'i-lucide-heart',
    to: '/support'
  }
]])

const updateDescription = computed(() => {
  if (error.value && !isDownloading.value) {
    return error.value
  }
  if (isDownloading.value && downloadProgress.value >= 100) {
    return t('updater.installing')
  }
  if (isDownloading.value && totalBytes.value > 0) {
    return t('updater.downloadProgress', {
      downloaded: formatBytes(downloadedBytes.value),
      total: formatBytes(totalBytes.value)
    })
  }
  if (isDownloading.value) {
    return t('updater.downloading', { progress: downloadProgress.value })
  }
  return t('updater.clickToInstall')
})

const updateActions = computed(() => {
  if (isDownloading.value) return []
  if (error.value) {
    return [{ label: t('updater.retry'), click: downloadAndInstall }]
  }
  return [{ label: t('updater.updateNow'), click: downloadAndInstall }]
})

const updateColor = computed(() => {
  if (error.value && !isDownloading.value) return 'error'
  return 'info'
})

onMounted(() => {
  loadAppVersion()
  checkForUpdates()
})
</script>

<template>
  <UDashboardGroup>
    <UDashboardSidebar
      id="sidebar"
      collapsible
      resizable
    >
      <template #header="{ collapsed }">
        <div class="flex items-center gap-2">
          <UIcon
            name="i-lucide-mic"
            class="size-6 text-primary"
          />
          <span
            v-if="!collapsed"
            class="font-bold text-lg"
          >{{ $t('nav.appName') }}</span>
        </div>
      </template>

      <template #default="{ collapsed }">
        <UNavigationMenu
          :collapsed="collapsed"
          :items="items[0]"
          orientation="vertical"
        />

        <UNavigationMenu
          :collapsed="collapsed"
          :items="items[1]"
          orientation="vertical"
          class="mt-auto"
        />

        <div class="px-3 pb-3 space-y-2">
          <UButton
            icon="i-lucide-languages"
            variant="ghost"
            :block="!collapsed"
            :square="collapsed"
            size="sm"
            @click="toggleLocale"
          >
            <span v-if="!collapsed">{{ locale === 'ar' ? 'English' : 'العربية' }}</span>
          </UButton>

          <p
            v-if="!collapsed && appVersion"
            class="text-xs text-center text-muted"
          >
            v{{ appVersion }}
          </p>
        </div>
      </template>
    </UDashboardSidebar>

    <div class="flex flex-col flex-1 min-w-0">
      <div v-if="updateAvailable">
        <UAlert
          icon="i-lucide-download"
          :color="updateColor"
          variant="subtle"
          :title="$t('updater.newVersion', { version: updateVersion })"
          :description="updateDescription"
          :actions="updateActions"
          orientation="horizontal"
          :close="!isDownloading"
          @update:open="dismiss"
        />
        <UProgress
          v-if="isDownloading"
          :model-value="downloadProgress"
          :max="100"
          size="xs"
          :color="downloadProgress >= 100 ? 'success' : 'info'"
        />
      </div>

      <slot />
    </div>
  </UDashboardGroup>
</template>
