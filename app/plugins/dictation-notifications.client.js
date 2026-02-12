import { tauriListen } from '~/utils/tauri'

export default defineNuxtPlugin(async () => {
  const toast = useToast()
  const router = useRouter()
  const { $i18n } = useNuxtApp()

  function isHomePage() {
    return router.currentRoute.value.path === '/'
  }

  await tauriListen('dictation-status', (event) => {
    if (isHomePage()) return

    const { is_recording, is_processing } = event.payload
    if (is_recording) {
      toast.add({
        id: 'dictation-status',
        title: $i18n.t('home.recording'),
        icon: 'i-lucide-mic',
        color: 'error',
        duration: 3000
      })
    } else if (is_processing) {
      toast.add({
        id: 'dictation-status',
        title: $i18n.t('home.processing'),
        icon: 'i-lucide-cpu',
        color: 'warning',
        duration: 0
      })
    }
  })

  await tauriListen('ai-refine-status', (event) => {
    if (isHomePage()) return

    if (event.payload.status === 'started') {
      toast.add({
        id: 'dictation-status',
        title: $i18n.t('home.refining'),
        icon: 'i-lucide-sparkles',
        color: 'info',
        duration: 0
      })
    } else if (event.payload.status === 'done' || event.payload.status === 'error') {
      toast.remove('dictation-status')
    }
  })

  await tauriListen('dictation-result', (event) => {
    if (isHomePage()) return

    toast.remove('dictation-status')

    const text = event.payload.text
    if (text) {
      toast.add({
        id: 'dictation-result',
        title: $i18n.t('home.lastResult'),
        description: text.length > 100 ? text.substring(0, 100) + '...' : text,
        icon: 'i-lucide-check',
        color: 'success',
        duration: 5000
      })
    }
  })
})
