<script setup lang="ts">
import { ar } from "@nuxt/ui/locale";
useHead({
  meta: [{ name: "viewport", content: "width=device-width, initial-scale=1" }],
  link: [{ rel: "icon", href: "/favicon.ico" }],
  htmlAttrs: {
    lang: "ar",
    dir: "rtl",
  },
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
  <UApp :locale="ar">
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </UApp>
</template>
