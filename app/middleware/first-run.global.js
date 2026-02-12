import { tauriInvoke } from '~/utils/tauri'

export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path === '/welcome' || to.path === '/overlay') {
    return
  }

  try {
    const hasModel = await tauriInvoke('has_active_model')
    if (!hasModel) {
      return navigateTo('/welcome')
    }
  } catch (e) {
    console.error('Failed to check active model:', e)
    return navigateTo('/welcome')
  }
})
