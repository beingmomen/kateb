<script setup>
const { t, locale, setLocale } = useI18n()
const { updateAvailable, updateVersion, isDownloading, downloadProgress, checkForUpdates, downloadAndInstall, dismiss } = useUpdater()

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

onMounted(() => {
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

        <div class="px-3 pb-3">
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
        </div>
      </template>
    </UDashboardSidebar>

    <div class="flex flex-col flex-1 min-w-0">
      <UAlert
        v-if="updateAvailable"
        icon="i-lucide-download"
        color="info"
        variant="subtle"
        :title="$t('updater.newVersion', { version: updateVersion })"
        :description="isDownloading ? $t('updater.downloading', { progress: downloadProgress }) : $t('updater.clickToInstall')"
        :actions="isDownloading ? [] : [{ label: $t('updater.updateNow'), click: downloadAndInstall }]"
        orientation="horizontal"
        :close="!isDownloading"
        @update:open="dismiss"
      />

      <slot />
    </div>
  </UDashboardGroup>
</template>
