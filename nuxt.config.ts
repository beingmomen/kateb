export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@nuxt/fonts',
    '@nuxtjs/i18n'
  ],

  i18n: {
    locales: [
      { code: 'ar', name: 'العربية', dir: 'rtl', file: 'ar.json' },
      { code: 'en', name: 'English', dir: 'ltr', file: 'en.json' }
    ],
    defaultLocale: 'ar',
    langDir: 'locales',
    strategy: 'no_prefix'
  },

  ssr: false,

  devServer: {
    port: 3654
  },

  devtools: {
    enabled: false
  },

  css: ['~/assets/css/main.css'],

  fonts: {
    families: [
      { name: 'IBM Plex Sans Arabic', provider: 'google' }
    ]
  },

  compatibilityDate: '2025-01-15',

  eslint: {
    config: {
      stylistic: {
        commaDangle: 'never',
        braceStyle: '1tbs'
      }
    }
  }
})
