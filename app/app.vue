<script setup lang="ts">
import { ar, en } from "@nuxt/ui/locale";

const { locale } = useI18n();

const uiLocale = computed(() => (locale.value === "ar" ? ar : en));

useHead({
  meta: [{ name: "viewport", content: "width=device-width, initial-scale=1" }],
  link: [{ rel: "icon", href: "/favicon.ico" }],
  htmlAttrs: computed(() => ({
    lang: locale.value,
    dir: locale.value === "ar" ? "rtl" : "ltr",
  })),
});

const title = "إملاء صوتي عربي";
const description = "تطبيق تحويل الصوت إلى نص عربي وإنجليزي باستخدام Whisper";

useSeoMeta({
  title,
  description,
  ogTitle: title,
  ogDescription: description,
});
</script>

<template>
  <UApp :locale="uiLocale">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
