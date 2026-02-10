<script setup>
const { updateAvailable, updateVersion, isDownloading, downloadProgress, checkForUpdates, downloadAndInstall, dismiss } = useUpdater()

const items = [[
  {
    label: 'الرئيسية',
    icon: 'i-lucide-mic',
    to: '/'
  },
  {
    label: 'السجل',
    icon: 'i-lucide-history',
    to: '/history'
  },
  {
    label: 'الإحصائيات',
    icon: 'i-lucide-bar-chart-3',
    to: '/stats'
  }
], [
  {
    label: 'النموذج',
    icon: 'i-lucide-brain',
    to: '/models'
  },
  {
    label: 'الإعدادات',
    icon: 'i-lucide-settings',
    to: '/settings'
  },
  {
    label: 'دعم المشروع',
    icon: 'i-lucide-heart',
    to: '/support'
  }
]]

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
          >إملاء صوتي</span>
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
      </template>
    </UDashboardSidebar>

    <div class="flex flex-col flex-1 min-w-0">
      <UAlert
        v-if="updateAvailable"
        icon="i-lucide-download"
        color="info"
        variant="subtle"
        :title="`تحديث جديد متاح: v${updateVersion}`"
        :description="isDownloading ? `جاري التحميل... ${downloadProgress}%` : 'اضغط لتحميل وتثبيت التحديث'"
        :actions="isDownloading ? [] : [{ label: 'تحديث الآن', click: downloadAndInstall }]"
        orientation="horizontal"
        :close="!isDownloading"
        @update:open="dismiss"
      />

      <slot />
    </div>
  </UDashboardGroup>
</template>
