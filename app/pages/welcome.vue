<script setup>
definePageMeta({
  layout: false
})

const { t } = useI18n()
const { availableModels, isDownloading, downloadingModelId, downloadProgress, downloadedBytes, totalBytes, getAvailableModels, downloadModel, setActiveModel, hasActiveModel, formatBytes } = useModels()
const { providers, getProviders, detectGpu } = useAI()
const { updateSetting } = useSettings()
const router = useRouter()

const step = ref(1)
const selectedModelId = ref(null)
const downloadError = ref(null)
const downloadComplete = ref(false)
const isChecking = ref(true)
const isActivating = ref(false)
const isRedirecting = ref(false)

const gpuChoice = ref('cpu')
const gpuDetected = ref(null)
const isDetectingGpu = ref(false)

const aiChoice = ref('none')
const selectedProvider = ref('claude')
const apiKeyInput = ref('')
const customAiUrl = ref('')

onMounted(async () => {
  try {
    const has = await hasActiveModel()
    if (has) {
      router.push('/')
      return
    }
  } catch (e) {
    console.error('Failed to check active model:', e)
  } finally {
    isChecking.value = false
  }
  await getAvailableModels()
  await getProviders()

  isDetectingGpu.value = true
  try {
    gpuDetected.value = await detectGpu()
    gpuChoice.value = gpuDetected.value.recommended
  } catch {
    gpuDetected.value = { cuda_available: false, recommended: 'cpu' }
  } finally {
    isDetectingGpu.value = false
  }
})

function goBack() {
  if (step.value > 1 && !isDownloading.value) {
    step.value -= 1
    downloadError.value = null
  }
}

async function goToStep3() {
  await updateSetting('use_gpu', gpuChoice.value === 'gpu' ? 'true' : 'false')
  step.value = 3
}

async function goToStep4() {
  if (aiChoice.value === 'none') {
    await updateSetting('ai_refinement', 'false')
    await updateSetting('ai_provider', 'local')
  } else {
    await updateSetting('ai_refinement', 'true')
    await updateSetting('ai_provider', selectedProvider.value)
    if (apiKeyInput.value) {
      await updateSetting(`${selectedProvider.value}_api_key`, apiKeyInput.value)
    }
    if (customAiUrl.value) {
      await updateSetting(`${selectedProvider.value}_api_url`, customAiUrl.value)
    }
  }
  step.value = 4
}

function performRedirect() {
  isRedirecting.value = true
  setTimeout(() => {
    router.push('/').catch(() => {
      isRedirecting.value = false
    })
  }, 2000)
}

async function handleDownload(modelId) {
  selectedModelId.value = modelId
  downloadError.value = null
  downloadComplete.value = false
  try {
    await downloadModel(modelId)
    await setActiveModel(modelId)
    downloadComplete.value = true
    step.value = 5
    performRedirect()
  } catch (e) {
    downloadError.value = e.toString()
  }
}

async function handleActivate(modelId) {
  selectedModelId.value = modelId
  isActivating.value = true
  downloadError.value = null
  try {
    await setActiveModel(modelId)
    step.value = 5
    performRedirect()
  } catch (e) {
    downloadError.value = e.toString()
  } finally {
    isActivating.value = false
  }
}

function handleRetry() {
  if (selectedModelId.value) {
    handleDownload(selectedModelId.value)
  }
}

const progressText = computed(() => {
  if (!isDownloading.value) return ''
  return `${formatBytes(downloadedBytes.value)} / ${formatBytes(totalBytes.value)}`
})

const successMessage = computed(() => {
  return downloadComplete.value
    ? t('welcome.modelDownloaded')
    : t('welcome.modelActivated')
})

const allProviders = computed(() => {
  return providers.value || []
})

const apiKeyPlaceholder = computed(() => {
  switch (selectedProvider.value) {
    case 'claude': return 'sk-ant-...'
    case 'openai': return 'sk-...'
    case 'gemini': return 'AIza...'
    case 'grok': return 'xai-...'
    case 'local': return t('common.optional')
    default: return ''
  }
})

function accuracyStars(level) {
  return '\u2605'.repeat(level) + '\u2606'.repeat(5 - level)
}

function speedLabel(level) {
  const labels = ['', t('welcome.speedVerySlow'), t('welcome.speedSlow'), t('welcome.speedMedium'), t('welcome.speedFast'), t('welcome.speedVeryFast')]
  return labels[level] || ''
}

function stepClass(s) {
  if (step.value === s) return 'bg-primary-500 text-white shadow-lg scale-110'
  if (step.value > s) return 'bg-primary-200 dark:bg-primary-800 text-primary-700 dark:text-primary-300'
  return 'bg-gray-200 dark:bg-gray-700 text-gray-400 dark:text-gray-500'
}

function lineClass(afterStep) {
  return step.value > afterStep ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100 dark:from-gray-900 dark:to-gray-800 p-6">
    <template v-if="isChecking">
      <div class="flex items-center gap-2">
        <UIcon
          name="i-lucide-loader-2"
          class="size-5 animate-spin"
        />
        <span>{{ $t('welcome.checking') }}</span>
      </div>
    </template>

    <template v-else>
      <div class="flex items-center gap-2 mb-8">
        <div
          class="flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold transition-all duration-300"
          :class="stepClass(1)"
        >
          <UIcon
            v-if="step > 1"
            name="i-lucide-check"
            class="size-3.5"
          />
          <span v-else>1</span>
        </div>
        <div
          class="w-6 h-0.5 rounded transition-all duration-300"
          :class="lineClass(1)"
        />
        <div
          class="flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold transition-all duration-300"
          :class="stepClass(2)"
        >
          <UIcon
            v-if="step > 2"
            name="i-lucide-check"
            class="size-3.5"
          />
          <span v-else>2</span>
        </div>
        <div
          class="w-6 h-0.5 rounded transition-all duration-300"
          :class="lineClass(2)"
        />
        <div
          class="flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold transition-all duration-300"
          :class="stepClass(3)"
        >
          <UIcon
            v-if="step > 3"
            name="i-lucide-check"
            class="size-3.5"
          />
          <span v-else>3</span>
        </div>
        <div
          class="w-6 h-0.5 rounded transition-all duration-300"
          :class="lineClass(3)"
        />
        <div
          class="flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold transition-all duration-300"
          :class="stepClass(4)"
        >
          <UIcon
            v-if="step > 4"
            name="i-lucide-check"
            class="size-3.5"
          />
          <span v-else>4</span>
        </div>
        <div
          class="w-6 h-0.5 rounded transition-all duration-300"
          :class="lineClass(4)"
        />
        <div
          class="flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold transition-all duration-300"
          :class="stepClass(5)"
        >
          <span>5</span>
        </div>
      </div>

      <!-- Step 1: Welcome -->
      <template v-if="step === 1">
        <UCard class="w-full max-w-lg text-center">
          <div class="py-10 space-y-6">
            <div class="flex justify-center">
              <div class="w-20 h-20 bg-primary-500 rounded-2xl flex items-center justify-center">
                <UIcon
                  name="i-lucide-mic"
                  class="size-10 text-white"
                />
              </div>
            </div>

            <div class="space-y-2">
              <h1 class="text-2xl font-bold">
                {{ $t('welcome.title') }}
              </h1>
              <p class="text-muted">
                {{ $t('welcome.subtitle') }}
              </p>
            </div>

            <div class="space-y-3 text-sm text-muted text-right">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 bg-primary-100 dark:bg-primary-900 rounded-lg flex items-center justify-center shrink-0">
                  <UIcon
                    name="i-lucide-zap"
                    class="size-4 text-primary-500"
                  />
                </div>
                <span>{{ $t('welcome.feature1') }}</span>
              </div>
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 bg-primary-100 dark:bg-primary-900 rounded-lg flex items-center justify-center shrink-0">
                  <UIcon
                    name="i-lucide-shield"
                    class="size-4 text-primary-500"
                  />
                </div>
                <span>{{ $t('welcome.feature2') }}</span>
              </div>
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 bg-primary-100 dark:bg-primary-900 rounded-lg flex items-center justify-center shrink-0">
                  <UIcon
                    name="i-lucide-sparkles"
                    class="size-4 text-primary-500"
                  />
                </div>
                <span>{{ $t('welcome.feature3') }}</span>
              </div>
            </div>

            <UButton
              size="lg"
              block
              icon="i-lucide-arrow-left"
              @click="step = 2"
            >
              {{ $t('welcome.startNow') }}
            </UButton>
          </div>
        </UCard>
      </template>

      <!-- Step 2: GPU/CPU -->
      <template v-else-if="step === 2">
        <div class="w-full max-w-xl space-y-6">
          <div class="flex items-center justify-between">
            <UButton
              variant="ghost"
              icon="i-lucide-arrow-right"
              @click="goBack"
            >
              {{ $t('common.back') }}
            </UButton>
            <div class="text-center flex-1">
              <h2 class="text-xl font-bold">
                {{ $t('welcome.step2Title') }}
              </h2>
              <p class="text-muted text-sm mt-1">
                {{ $t('welcome.step2Subtitle') }}
              </p>
            </div>
            <div class="w-20" />
          </div>

          <div
            v-if="isDetectingGpu"
            class="flex items-center justify-center gap-2 text-sm text-muted"
          >
            <UIcon
              name="i-lucide-loader-2"
              class="size-4 animate-spin"
            />
            <span>{{ $t('welcome.detectingGpu') }}</span>
          </div>

          <template v-else>
            <div
              v-if="gpuDetected"
              class="text-center text-sm mb-2"
              :class="gpuDetected.cuda_available ? 'text-green-600 dark:text-green-400' : 'text-muted'"
            >
              <template v-if="gpuDetected.cuda_available">
                {{ $t('welcome.gpuDetected') }}
              </template>
              <template v-else>
                {{ $t('welcome.gpuNotDetected') }}
              </template>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div
                class="cursor-pointer rounded-xl border-2 p-5 text-center transition-all space-y-3"
                :class="gpuChoice === 'cpu'
                  ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                  : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'"
                @click="gpuChoice = 'cpu'"
              >
                <div class="flex justify-center">
                  <div
                    class="w-14 h-14 rounded-xl flex items-center justify-center"
                    :class="gpuChoice === 'cpu' ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'"
                  >
                    <UIcon
                      name="i-lucide-cpu"
                      class="size-7"
                      :class="gpuChoice === 'cpu' ? 'text-white' : ''"
                    />
                  </div>
                </div>
                <h3 class="font-bold">
                  {{ $t('welcome.cpuLabel') }}
                </h3>
                <p class="text-xs text-muted">
                  {{ $t('welcome.cpuDesc') }}
                </p>
                <UBadge
                  v-if="!gpuDetected?.cuda_available"
                  color="primary"
                  variant="subtle"
                  size="xs"
                >
                  {{ $t('common.recommended') }}
                </UBadge>
              </div>

              <div
                class="rounded-xl border-2 p-5 text-center transition-all space-y-3"
                :class="[
                  !gpuDetected?.cuda_available ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
                  gpuChoice === 'gpu'
                    ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                    : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
                ]"
                @click="gpuDetected?.cuda_available && (gpuChoice = 'gpu')"
              >
                <div class="flex justify-center">
                  <div
                    class="w-14 h-14 rounded-xl flex items-center justify-center"
                    :class="gpuChoice === 'gpu' ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'"
                  >
                    <UIcon
                      name="i-lucide-monitor"
                      class="size-7"
                      :class="gpuChoice === 'gpu' ? 'text-white' : ''"
                    />
                  </div>
                </div>
                <h3 class="font-bold">
                  {{ $t('welcome.gpuLabel') }}
                </h3>
                <p class="text-xs text-muted">
                  {{ $t('welcome.gpuDesc') }}
                </p>
                <UBadge
                  v-if="gpuDetected?.cuda_available"
                  color="primary"
                  variant="subtle"
                  size="xs"
                >
                  {{ $t('common.recommended') }}
                </UBadge>
                <UBadge
                  v-else
                  color="error"
                  variant="subtle"
                  size="xs"
                >
                  {{ $t('common.unavailable') }}
                </UBadge>
              </div>
            </div>

            <div
              v-if="gpuChoice === 'gpu' && !gpuDetected?.cuda_available"
              class="bg-amber-50 dark:bg-amber-900/20 text-amber-800 dark:text-amber-200 p-3 rounded-lg text-sm flex items-start gap-2"
            >
              <UIcon
                name="i-lucide-alert-triangle"
                class="size-4 mt-0.5 shrink-0"
              />
              <span>{{ $t('welcome.gpuWarning') }}</span>
            </div>

            <UButton
              size="lg"
              block
              icon="i-lucide-arrow-left"
              @click="goToStep3"
            >
              {{ $t('common.next') }}
            </UButton>
          </template>
        </div>
      </template>

      <!-- Step 3: AI -->
      <template v-else-if="step === 3">
        <div class="w-full max-w-xl space-y-6">
          <div class="flex items-center justify-between">
            <UButton
              variant="ghost"
              icon="i-lucide-arrow-right"
              @click="goBack"
            >
              {{ $t('common.back') }}
            </UButton>
            <div class="text-center flex-1">
              <h2 class="text-xl font-bold">
                {{ $t('welcome.step3Title') }}
              </h2>
              <p class="text-muted text-sm mt-1">
                {{ $t('welcome.step3Subtitle') }}
              </p>
            </div>
            <div class="w-20" />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div
              class="cursor-pointer rounded-xl border-2 p-5 text-center transition-all space-y-3"
              :class="aiChoice === 'none'
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'"
              @click="aiChoice = 'none'"
            >
              <div class="flex justify-center">
                <div
                  class="w-14 h-14 rounded-xl flex items-center justify-center"
                  :class="aiChoice === 'none' ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'"
                >
                  <UIcon
                    name="i-lucide-shield-check"
                    class="size-7"
                    :class="aiChoice === 'none' ? 'text-white' : ''"
                  />
                </div>
              </div>
              <h3 class="font-bold">
                {{ $t('welcome.noRefinement') }}
              </h3>
              <p class="text-xs text-muted">
                {{ $t('welcome.noRefinementDesc') }}
              </p>
              <UBadge
                color="primary"
                variant="subtle"
                size="xs"
              >
                {{ $t('common.recommended') }}
              </UBadge>
            </div>

            <div
              class="cursor-pointer rounded-xl border-2 p-5 text-center transition-all space-y-3"
              :class="aiChoice === 'with_ai'
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'"
              @click="aiChoice = 'with_ai'"
            >
              <div class="flex justify-center">
                <div
                  class="w-14 h-14 rounded-xl flex items-center justify-center"
                  :class="aiChoice === 'with_ai' ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'"
                >
                  <UIcon
                    name="i-lucide-sparkles"
                    class="size-7"
                    :class="aiChoice === 'with_ai' ? 'text-white' : ''"
                  />
                </div>
              </div>
              <h3 class="font-bold">
                {{ $t('welcome.withAi') }}
              </h3>
              <p class="text-xs text-muted">
                {{ $t('welcome.withAiDesc') }}
              </p>
            </div>
          </div>

          <template v-if="aiChoice === 'with_ai'">
            <div class="space-y-4">
              <div class="grid grid-cols-2 sm:grid-cols-5 gap-2">
                <div
                  v-for="p in allProviders"
                  :key="p.id"
                  class="cursor-pointer rounded-lg border-2 p-3 text-center text-sm transition-all"
                  :class="selectedProvider === p.id
                    ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                    : 'border-gray-200 dark:border-gray-700'"
                  @click="selectedProvider = p.id; apiKeyInput = ''; customAiUrl = ''"
                >
                  <p class="font-medium text-xs">
                    {{ p.id === 'local' ? $t('welcome.customServer') : p.name }}
                  </p>
                </div>
              </div>

              <UInput
                v-if="selectedProvider !== 'local'"
                v-model="apiKeyInput"
                type="password"
                icon="i-lucide-key"
                :placeholder="apiKeyPlaceholder"
                size="lg"
              />

              <template v-if="selectedProvider === 'local'">
                <UInput
                  v-model="customAiUrl"
                  icon="i-lucide-globe"
                  placeholder="https://your-domain.com"
                  size="lg"
                />
                <p class="text-xs text-muted text-center">
                  {{ $t('welcome.enterDomainOnly') }}
                </p>
                <UInput
                  v-model="apiKeyInput"
                  type="password"
                  icon="i-lucide-key"
                  :placeholder="$t('welcome.apiKeyOptional')"
                  size="lg"
                />
              </template>

              <p class="text-xs text-muted text-center">
                {{ $t('welcome.canChangeLater') }}
              </p>
            </div>
          </template>

          <UButton
            size="lg"
            block
            icon="i-lucide-arrow-left"
            @click="goToStep4"
          >
            {{ $t('common.next') }}
          </UButton>
        </div>
      </template>

      <!-- Step 4: Model Selection -->
      <template v-else-if="step === 4">
        <div class="w-full max-w-4xl space-y-6">
          <div class="flex items-center justify-between">
            <UButton
              variant="ghost"
              icon="i-lucide-arrow-right"
              :disabled="isDownloading"
              @click="goBack"
            >
              {{ $t('common.back') }}
            </UButton>
            <div class="text-center flex-1">
              <h2 class="text-xl font-bold">
                {{ $t('welcome.step4Title') }}
              </h2>
              <p class="text-muted text-sm mt-1">
                {{ $t('welcome.step4Subtitle') }}
              </p>
            </div>
            <div class="w-20" />
          </div>

          <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <UCard
              v-for="m in availableModels"
              :key="m.id"
              :class="[
                'relative transition-all',
                m.recommended ? 'ring-2 ring-primary-500' : '',
                selectedModelId === m.id && isDownloading ? 'ring-2 ring-blue-500' : ''
              ]"
            >
              <div
                v-if="m.recommended"
                class="absolute -top-2 left-1/2 -translate-x-1/2 bg-primary-500 text-white text-xs px-3 py-0.5 rounded-full"
              >
                {{ $t('common.recommended') }}
              </div>

              <div class="space-y-3 pt-2">
                <div class="text-center">
                  <h3 class="font-bold">
                    {{ m.name }}
                  </h3>
                  <p class="text-xs text-muted mt-1">
                    {{ m.description_ar }}
                  </p>
                </div>

                <div class="space-y-2 text-sm">
                  <div class="flex justify-between">
                    <span class="text-muted">{{ $t('common.size') }}</span>
                    <span class="font-medium">{{ m.size_display }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-muted">{{ $t('common.accuracy') }}</span>
                    <span class="text-amber-500">{{ accuracyStars(m.accuracy) }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-muted">{{ $t('common.speed') }}</span>
                    <span class="font-medium">{{ speedLabel(m.speed) }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-muted">{{ $t('common.memory') }}</span>
                    <span class="font-medium">{{ m.ram_mb }} MB</span>
                  </div>
                </div>

                <div class="space-y-1">
                  <div
                    v-for="pro in m.pros_ar"
                    :key="pro"
                    class="flex items-center gap-1 text-xs text-green-600 dark:text-green-400"
                  >
                    <UIcon
                      name="i-lucide-check"
                      class="size-3"
                    />
                    <span>{{ pro }}</span>
                  </div>
                  <div
                    v-for="con in m.cons_ar"
                    :key="con"
                    class="flex items-center gap-1 text-xs text-red-500 dark:text-red-400"
                  >
                    <UIcon
                      name="i-lucide-x"
                      class="size-3"
                    />
                    <span>{{ con }}</span>
                  </div>
                </div>

                <template v-if="isDownloading && downloadingModelId === m.id">
                  <div class="space-y-2">
                    <UProgress
                      :model-value="downloadProgress"
                      color="primary"
                      size="sm"
                    />
                    <p class="text-xs text-muted text-center">
                      {{ progressText }} ({{ Math.round(downloadProgress) }}%)
                    </p>
                  </div>
                </template>

                <template v-else-if="m.installed">
                  <UButton
                    block
                    size="sm"
                    color="primary"
                    icon="i-lucide-play"
                    :loading="isActivating && selectedModelId === m.id"
                    :disabled="isActivating || isDownloading"
                    @click="handleActivate(m.id)"
                  >
                    {{ $t('welcome.useThisModel') }}
                  </UButton>
                </template>

                <template v-else>
                  <UButton
                    block
                    size="sm"
                    icon="i-lucide-download"
                    :disabled="isDownloading"
                    @click="handleDownload(m.id)"
                  >
                    {{ $t('welcome.downloadSize', { size: m.size_display }) }}
                  </UButton>
                </template>
              </div>
            </UCard>
          </div>

          <div
            v-if="downloadError"
            class="bg-red-50 dark:bg-red-900/20 text-red-800 dark:text-red-200 p-4 rounded-lg text-sm text-center space-y-3"
          >
            <div class="flex items-center justify-center gap-2">
              <UIcon
                name="i-lucide-alert-circle"
                class="size-4"
              />
              <span>{{ downloadError }}</span>
            </div>
            <UButton
              v-if="selectedModelId"
              size="sm"
              variant="soft"
              color="error"
              icon="i-lucide-refresh-cw"
              @click="handleRetry"
            >
              {{ $t('common.retry') }}
            </UButton>
          </div>

          <p class="text-xs text-muted text-center">
            {{ $t('welcome.modelsFromHf') }}
          </p>
        </div>
      </template>

      <!-- Step 5: Success -->
      <template v-else-if="step === 5">
        <UCard class="w-full max-w-lg text-center">
          <div class="py-10 space-y-6">
            <div class="flex justify-center">
              <div class="w-20 h-20 bg-green-500 rounded-2xl flex items-center justify-center">
                <UIcon
                  name="i-lucide-check"
                  class="size-10 text-white"
                />
              </div>
            </div>

            <div class="space-y-2">
              <h1 class="text-2xl font-bold">
                {{ $t('welcome.appReady') }}
              </h1>
              <p class="text-muted">
                {{ successMessage }}
              </p>
            </div>

            <div
              v-if="isRedirecting"
              class="flex items-center justify-center gap-2 text-sm text-muted"
            >
              <UIcon
                name="i-lucide-loader-2"
                class="size-4 animate-spin"
              />
              <span>{{ $t('welcome.redirecting') }}</span>
            </div>

            <UButton
              size="lg"
              block
              icon="i-lucide-arrow-left"
              @click="router.push('/')"
            >
              {{ $t('welcome.goToApp') }}
            </UButton>
          </div>
        </UCard>
      </template>
    </template>
  </div>
</template>
