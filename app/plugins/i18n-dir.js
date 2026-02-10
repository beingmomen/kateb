export default defineNuxtPlugin(() => {
  const { locale } = useI18n()

  function applyDir(loc) {
    if (import.meta.client) {
      document.documentElement.dir = loc === 'ar' ? 'rtl' : 'ltr'
      document.documentElement.lang = loc
    }
  }

  applyDir(locale.value)

  watch(locale, (val) => {
    applyDir(val)
  })
})
