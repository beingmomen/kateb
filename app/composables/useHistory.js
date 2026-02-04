import { tauriInvoke } from '~/utils/tauri'

export function useHistory() {
  const history = ref([])
  const loading = ref(false)
  const error = ref(null)

  async function fetchHistory(search = null, limit = 50, offset = 0) {
    loading.value = true
    try {
      history.value = await tauriInvoke('get_history', { search, limit, offset }) || []
    } catch (e) {
      error.value = e
    } finally {
      loading.value = false
    }
  }

  async function deleteItem(id) {
    try {
      await tauriInvoke('delete_history_item', { id })
      history.value = history.value.filter(item => item.id !== id)
    } catch (e) {
      error.value = e
    }
  }

  async function clearAll() {
    try {
      await tauriInvoke('clear_history')
      history.value = []
    } catch (e) {
      error.value = e
    }
  }

  return {
    history,
    loading,
    error,
    fetchHistory,
    deleteItem,
    clearAll
  }
}
