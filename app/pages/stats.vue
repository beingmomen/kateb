<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { t, locale } = useI18n()
const { dailyStats, summary, loading, fetchDailyStats, fetchSummary, formatDuration } = useStats()

onMounted(() => {
  fetchDailyStats()
  fetchSummary()
})

const summaryCards = computed(() => [
  {
    title: t('stats.totalDictations'),
    value: summary.value?.total_dictations ?? 0,
    icon: 'i-lucide-mic',
    color: 'primary'
  },
  {
    title: t('stats.totalWords'),
    value: summary.value?.total_words ?? 0,
    icon: 'i-lucide-type',
    color: 'info'
  },
  {
    title: t('stats.totalTime'),
    value: formatDuration(summary.value?.total_duration ?? 0),
    icon: 'i-lucide-clock',
    color: 'warning'
  },
  {
    title: t('stats.activeDays'),
    value: summary.value?.days_active ?? 0,
    icon: 'i-lucide-calendar',
    color: 'success'
  }
])

const chartLabels = computed(() =>
  dailyStats.value.map(s => {
    const date = new Date(s.date)
    return new Intl.DateTimeFormat(locale.value, { month: 'short', day: 'numeric' }).format(date)
  })
)

const dictationsData = computed(() => dailyStats.value.map(s => s.total_dictations))
const wordsData = computed(() => dailyStats.value.map(s => s.total_words))

const activePeriod = ref(30)
const periodOptions = computed(() => [
  { label: t('stats.period7'), value: 7 },
  { label: t('stats.period30'), value: 30 },
  { label: t('stats.period90'), value: 90 }
])

watch(activePeriod, (val) => {
  fetchDailyStats(val)
})
</script>

<template>
  <UDashboardPanel id="stats">
    <template #header>
      <UDashboardNavbar :title="$t('stats.title')">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>

        <template #right>
          <USelect
            v-model="activePeriod"
            :items="periodOptions"
            value-key="value"
            size="sm"
          />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="space-y-6 py-4">
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <UCard
            v-for="card in summaryCards"
            :key="card.title"
          >
            <div class="flex items-center gap-3">
              <div class="rounded-lg bg-primary/10 p-2">
                <UIcon
                  :name="card.icon"
                  class="size-5 text-primary"
                />
              </div>
              <div>
                <p class="text-sm text-muted">{{ card.title }}</p>
                <p class="text-2xl font-bold">{{ card.value }}</p>
              </div>
            </div>
          </UCard>
        </div>

        <div
          v-if="loading"
          class="flex justify-center py-12"
        >
          <UIcon
            name="i-lucide-loader-2"
            class="size-8 animate-spin text-muted"
          />
        </div>

        <div v-else-if="!dailyStats.length" class="text-center py-12">
          <UIcon
            name="i-lucide-bar-chart-3"
            class="size-12 text-muted mx-auto mb-4"
          />
          <p class="text-muted">{{ $t('stats.noStats') }}</p>
        </div>

        <template v-else>
          <UCard>
            <template #header>
              <span class="font-semibold">{{ $t('stats.dailyDictations') }}</span>
            </template>
            <div class="h-64 flex items-end gap-1">
              <div
                v-for="(count, index) in dictationsData"
                :key="index"
                class="flex-1 bg-primary/80 rounded-t transition-all hover:bg-primary"
                :style="{ height: `${Math.max(4, (count / Math.max(...dictationsData, 1)) * 100)}%` }"
                :title="`${chartLabels[index]}: ${count} ${$t('stats.dictation')}`"
              />
            </div>
            <div class="flex justify-between text-xs text-muted mt-2">
              <span>{{ chartLabels[0] }}</span>
              <span>{{ chartLabels[chartLabels.length - 1] }}</span>
            </div>
          </UCard>

          <UCard>
            <template #header>
              <span class="font-semibold">{{ $t('stats.dailyWords') }}</span>
            </template>
            <div class="h-64 flex items-end gap-1">
              <div
                v-for="(count, index) in wordsData"
                :key="index"
                class="flex-1 bg-info/80 rounded-t transition-all hover:bg-info"
                :style="{ height: `${Math.max(4, (count / Math.max(...wordsData, 1)) * 100)}%` }"
                :title="`${chartLabels[index]}: ${count} ${$t('stats.word')}`"
              />
            </div>
            <div class="flex justify-between text-xs text-muted mt-2">
              <span>{{ chartLabels[0] }}</span>
              <span>{{ chartLabels[chartLabels.length - 1] }}</span>
            </div>
          </UCard>
        </template>
      </div>
    </template>
  </UDashboardPanel>
</template>
