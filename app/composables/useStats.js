import { tauriInvoke } from '~/utils/tauri'

export function useStats() {
  const dailyStats = ref([])
  const summary = ref(null)
  const loading = ref(false)
  const error = ref(null)

  async function fetchDailyStats(days = 30) {
    loading.value = true
    try {
      dailyStats.value = await tauriInvoke('get_usage_stats', { days }) || []
    } catch (e) {
      error.value = e
    } finally {
      loading.value = false
    }
  }

  async function fetchSummary() {
    try {
      summary.value = await tauriInvoke('get_summary_stats')
    } catch (e) {
      error.value = e
    }
  }

  function formatDuration(seconds) {
    if (seconds < 60) return `${seconds} ثانية`
    if (seconds < 3600) return `${Math.floor(seconds / 60)} دقيقة`
    const hours = Math.floor(seconds / 3600)
    const mins = Math.floor((seconds % 3600) / 60)
    return `${hours} ساعة ${mins > 0 ? `و ${mins} دقيقة` : ''}`
  }

  return {
    dailyStats,
    summary,
    loading,
    error,
    fetchDailyStats,
    fetchSummary,
    formatDuration
  }
}
