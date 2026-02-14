<script setup lang="ts">
import { ar, en } from '@nuxt/ui/locale'

const route = useRoute()
const { locale } = useI18n()

const uiLocale = computed(() => (locale.value === 'ar' ? ar : en))
const isOverlay = computed(() => route.path === '/overlay')

useHead(computed(() => ({
  meta: [{ name: 'viewport', content: 'width=device-width, initial-scale=1' }],
  link: [{ rel: 'icon', href: '/favicon.ico' }],
  htmlAttrs: {
    lang: locale.value,
    dir: locale.value === 'ar' ? 'rtl' : 'ltr'
  }
})))

const title = 'إملاء صوتي عربي'
const description = 'تطبيق تحويل الصوت إلى نص عربي وإنجليزي باستخدام Whisper'

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description
})
</script>

<template>
  <NuxtPage v-if="isOverlay" />
  <UApp v-else :locale="uiLocale">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
