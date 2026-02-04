<script setup>
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
    label: 'الإعدادات',
    icon: 'i-lucide-settings',
    to: '/settings'
  }
]]
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

    <slot />
  </UDashboardGroup>
</template>
