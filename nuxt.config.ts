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
      scan: true,
      include: [
        'i-lucide-mic',
        'i-lucide-history',
        'i-lucide-bar-chart-3',
        'i-lucide-brain',
        'i-lucide-settings',
        'i-lucide-heart',
        'i-lucide-languages',
        'i-lucide-save',
        'i-lucide-keyboard',
        'i-lucide-globe',
        'i-lucide-key',
        'i-lucide-plug',
        'i-lucide-sparkles',
        'i-lucide-cpu',
        'i-lucide-sliders-horizontal',
        'i-lucide-info',
        'i-lucide-check-circle',
        'i-lucide-check',
        'i-lucide-alert-circle',
        'i-lucide-alert-triangle',
        'i-lucide-download',
        'i-lucide-upload',
        'i-lucide-hard-drive',
        'i-lucide-command',
        'i-lucide-circle-dot',
        'i-lucide-circle-stop',
        'i-lucide-refresh-cw',
        'i-lucide-play',
        'i-lucide-square',
        'i-lucide-timer',
        'i-lucide-copy',
        'i-lucide-trash-2',
        'i-lucide-search',
        'i-lucide-calendar',
        'i-lucide-clock',
        'i-lucide-chevron-down',
        'i-lucide-x'
      ]
    }
  }
})
