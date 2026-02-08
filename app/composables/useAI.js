import { tauriInvoke } from '~/utils/tauri'

export function useAI() {
  const providers = ref([])
  const currentProvider = ref(null)
  const isEnabled = ref(false)
  const isTestingConnection = ref(false)
  const connectionStatus = ref(null)
  const error = ref(null)

  async function getProviders() {
    try {
      providers.value = await tauriInvoke('get_ai_providers')
      return providers.value
    } catch (e) {
      error.value = e
      return []
    }
  }

  async function getCurrentProvider() {
    try {
      const result = await tauriInvoke('get_current_ai_provider')
      currentProvider.value = result.provider
      isEnabled.value = result.enabled
      return result
    } catch (e) {
      error.value = e
      return null
    }
  }

  async function testConnection() {
    isTestingConnection.value = true
    connectionStatus.value = null
    try {
      const result = await tauriInvoke('test_ai_connection')
      connectionStatus.value = result
      return result
    } catch (e) {
      error.value = e
      connectionStatus.value = {
        success: false,
        message: e.toString()
      }
      return connectionStatus.value
    } finally {
      isTestingConnection.value = false
    }
  }

  async function testSpecificProvider(provider, apiKey) {
    isTestingConnection.value = true
    connectionStatus.value = null
    try {
      const result = await tauriInvoke('test_specific_provider', {
        provider,
        apiKey: apiKey || ''
      })
      connectionStatus.value = result
      return result
    } catch (e) {
      error.value = e
      connectionStatus.value = {
        success: false,
        message: e.toString()
      }
      return connectionStatus.value
    } finally {
      isTestingConnection.value = false
    }
  }

  function getProviderName(id) {
    const provider = providers.value.find(p => p.id === id)
    return provider?.name || id
  }

  function requiresApiKey(id) {
    const provider = providers.value.find(p => p.id === id)
    return provider?.requires_key ?? true
  }

  async function detectGpu() {
    try {
      return await tauriInvoke('detect_gpu')
    } catch {
      return { cuda_available: false, recommended: 'cpu' }
    }
  }

  return {
    providers,
    currentProvider,
    isEnabled,
    isTestingConnection,
    connectionStatus,
    error,
    getProviders,
    getCurrentProvider,
    testConnection,
    testSpecificProvider,
    getProviderName,
    requiresApiKey,
    detectGpu
  }
}
