<script setup>
definePageMeta({
  layout: 'dashboard'
})

const donationLinks = [
  {
    name: 'Ko-fi',
    icon: 'i-lucide-coffee',
    url: 'https://ko-fi.com/beingmomen',
    color: 'bg-[#FF5E5B]',
    description: 'ادعمنا بفنجان قهوة'
  },
  {
    name: 'Buy Me a Coffee',
    icon: 'i-lucide-heart',
    url: 'https://buymeacoffee.com/beingmomen',
    color: 'bg-[#FFDD00]',
    description: 'تبرع لدعم التطوير'
  }
]

const features = [
  {
    icon: 'i-lucide-mic',
    title: 'تحويل صوت عالي الجودة',
    description: 'استخدام Whisper Large V3 Turbo للحصول على أفضل دقة'
  },
  {
    icon: 'i-lucide-zap',
    title: 'أداء محلي سريع',
    description: 'يعمل على جهازك بدون الحاجة للإنترنت'
  },
  {
    icon: 'i-lucide-sparkles',
    title: 'تحسين بالذكاء الاصطناعي',
    description: 'تصحيح الأخطاء وإضافة علامات الترقيم تلقائياً'
  },
  {
    icon: 'i-lucide-shield',
    title: 'خصوصية تامة',
    description: 'بياناتك تبقى على جهازك فقط'
  }
]

function openLink(url) {
  window.open(url, '_blank')
}
</script>

<template>
  <UDashboardPanel id="support">
    <template #header>
      <UDashboardNavbar title="دعم المشروع">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="max-w-2xl mx-auto space-y-8 py-6">
        <div class="text-center space-y-4">
          <div class="flex justify-center">
            <div class="w-16 h-16 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center">
              <UIcon
                name="i-lucide-heart"
                class="size-8 text-primary-500"
              />
            </div>
          </div>
          <h2 class="text-xl font-bold">ادعم تطوير المشروع</h2>
          <p class="text-muted max-w-md mx-auto">
            هذا المشروع مجاني ومفتوح المصدر. دعمك يساعدنا على الاستمرار في التطوير وإضافة ميزات جديدة.
          </p>
        </div>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-gift"
                class="size-5"
              />
              <span class="font-semibold">طرق الدعم</span>
            </div>
          </template>

          <div class="grid gap-4">
            <button
              v-for="link in donationLinks"
              :key="link.name"
              class="flex items-center gap-4 p-4 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors text-right w-full"
              @click="openLink(link.url)"
            >
              <div
                class="w-12 h-12 rounded-lg flex items-center justify-center text-white"
                :class="link.color"
              >
                <UIcon
                  :name="link.icon"
                  class="size-6"
                />
              </div>
              <div class="flex-1">
                <p class="font-semibold">{{ link.name }}</p>
                <p class="text-sm text-muted">{{ link.description }}</p>
              </div>
              <UIcon
                name="i-lucide-external-link"
                class="size-5 text-muted"
              />
            </button>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-star"
                class="size-5"
              />
              <span class="font-semibold">مميزات المشروع</span>
            </div>
          </template>

          <div class="grid gap-4 sm:grid-cols-2">
            <div
              v-for="feature in features"
              :key="feature.title"
              class="flex items-start gap-3"
            >
              <div class="w-10 h-10 bg-primary-100 dark:bg-primary-900 rounded-lg flex items-center justify-center shrink-0">
                <UIcon
                  :name="feature.icon"
                  class="size-5 text-primary-500"
                />
              </div>
              <div>
                <p class="font-medium">{{ feature.title }}</p>
                <p class="text-sm text-muted">{{ feature.description }}</p>
              </div>
            </div>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-github"
                class="size-5"
              />
              <span class="font-semibold">المساهمة في المشروع</span>
            </div>
          </template>

          <div class="space-y-4">
            <p class="text-muted">
              المشروع مفتوح المصدر ونرحب بمساهماتكم! يمكنكم المساهمة عبر:
            </p>
            <ul class="space-y-2 text-muted">
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-bug"
                  class="size-4"
                />
                الإبلاغ عن الأخطاء
              </li>
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-lightbulb"
                  class="size-4"
                />
                اقتراح ميزات جديدة
              </li>
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-code"
                  class="size-4"
                />
                المساهمة في الكود
              </li>
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-languages"
                  class="size-4"
                />
                المساعدة في الترجمة
              </li>
            </ul>
          </div>

          <template #footer>
            <UButton
              icon="i-lucide-github"
              variant="soft"
              block
              @click="openLink('https://github.com/beingmomen/kateb')"
            >
              زيارة المشروع على GitHub
            </UButton>
          </template>
        </UCard>

        <div class="text-center text-sm text-muted py-4">
          <p>شكراً لدعمكم</p>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
