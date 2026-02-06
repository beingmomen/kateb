import { tauriInvoke } from '~/utils/tauri'

export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path === '/welcome' || to.path === '/models') {
    return
  }

  try {
    const modelExists = await tauriInvoke('check_model_exists')
    if (!modelExists) {
      return navigateTo('/welcome')
    }
  } catch (e) {
    console.error('Failed to check model:', e)
  }
})
