<script setup>
definePageMeta({
  layout: 'dashboard'
})

const { t } = useI18n()

const donationLinks = computed(() => [
  {
    name: 'Ko-fi',
    icon: 'i-lucide-coffee',
    url: 'https://ko-fi.com/beingmomen',
    color: 'bg-[#FF5E5B]',
    description: t('support.kofiDesc')
  },
  {
    name: 'Buy Me a Coffee',
    icon: 'i-lucide-heart',
    url: 'https://buymeacoffee.com/beingmomen',
    color: 'bg-[#FFDD00]',
    description: t('support.bmcDesc')
  }
])

const features = computed(() => [
  {
    icon: 'i-lucide-mic',
    title: t('support.feature1Title'),
    description: t('support.feature1Desc')
  },
  {
    icon: 'i-lucide-zap',
    title: t('support.feature2Title'),
    description: t('support.feature2Desc')
  },
  {
    icon: 'i-lucide-sparkles',
    title: t('support.feature3Title'),
    description: t('support.feature3Desc')
  },
  {
    icon: 'i-lucide-shield',
    title: t('support.feature4Title'),
    description: t('support.feature4Desc')
  }
])

function openLink(url) {
  window.open(url, '_blank')
}
</script>

<template>
  <UDashboardPanel id="support">
    <template #header>
      <UDashboardNavbar :title="$t('support.title')">
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
          <h2 class="text-xl font-bold">
            {{ $t('support.heading') }}
          </h2>
          <p class="text-muted max-w-md mx-auto">
            {{ $t('support.description') }}
          </p>
        </div>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon
                name="i-lucide-gift"
                class="size-5"
              />
              <span class="font-semibold">{{ $t('support.donationMethods') }}</span>
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
                <p class="font-semibold">
                  {{ link.name }}
                </p>
                <p class="text-sm text-muted">
                  {{ link.description }}
                </p>
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
              <span class="font-semibold">{{ $t('support.features') }}</span>
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
                <p class="font-medium">
                  {{ feature.title }}
                </p>
                <p class="text-sm text-muted">
                  {{ feature.description }}
                </p>
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
              <span class="font-semibold">{{ $t('support.contribute') }}</span>
            </div>
          </template>

          <div class="space-y-4">
            <p class="text-muted">
              {{ $t('support.contributeDesc') }}
            </p>
            <ul class="space-y-2 text-muted">
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-bug"
                  class="size-4"
                />
                {{ $t('support.reportBugs') }}
              </li>
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-lightbulb"
                  class="size-4"
                />
                {{ $t('support.suggestFeatures') }}
              </li>
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-code"
                  class="size-4"
                />
                {{ $t('support.contributeCode') }}
              </li>
              <li class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-languages"
                  class="size-4"
                />
                {{ $t('support.helpTranslate') }}
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
              {{ $t('support.visitGithub') }}
            </UButton>
          </template>
        </UCard>

        <div class="text-center text-sm text-muted py-4">
          <p>{{ $t('support.thanks') }}</p>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
