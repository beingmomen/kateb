export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@nuxt/fonts',
    '@nuxtjs/i18n'
  ],

  ssr: false,

  devtools: {
    enabled: false
  },

  css: ['~/assets/css/main.css'],

  devServer: {
    port: 3654
  },

  compatibilityDate: '2025-01-15',

  eslint: {
    config: {
      stylistic: {
        commaDangle: 'never',
        braceStyle: '1tbs'
      }
    }
  },

  fonts: {
    families: [
      { name: 'IBM Plex Sans Arabic', provider: 'google' }
    ]
  },

  i18n: {
    locales: [
      { code: 'ar', name: 'العربية', dir: 'rtl', file: 'ar.json' },
      { code: 'en', name: 'English', dir: 'ltr', file: 'en.json' }
    ],
    defaultLocale: 'ar',
    langDir: 'locales',
    strategy: 'no_prefix',
    detectBrowserLanguage: false
  },

  icon: {
    clientBundle: {
      scan: true
    }
  }
})
