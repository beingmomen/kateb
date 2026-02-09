<script setup>
definePageMeta({
  layout: "dashboard",
});

const {
  isRecording,
  isProcessing,
  isRefining,
  lastResult,
  refiningText,
  error,
  toggleDictation,
  audioLevels,
  recordingDuration,
  processingDuration,
  refiningDuration,
  silenceCountdown,
  pipelineStage,
  sessionActive,
} = useDictation();

const statusText = computed(() => {
  if (isRefining.value) return "جارٍ تحسين النص...";
  if (isProcessing.value) return "جارٍ المعالجة...";
  if (isRecording.value) return "جارٍ التسجيل...";
  return "اضغط للبدء";
});

const statusColor = computed(() => {
  if (isProcessing.value) return "warning";
  if (isRecording.value) return "error";
  return "primary";
});

const formattedDuration = computed(() => {
  const mins = Math.floor(recordingDuration.value / 60);
  const secs = recordingDuration.value % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
});

function formatMs(ms) {
  if (ms < 1000) return `${ms} مللي ث`;
  return `${(ms / 1000).toFixed(1)} ث`;
}

function formatSecs(secs) {
  if (secs < 60) return `${secs} ث`;
  const mins = Math.floor(secs / 60);
  const remaining = secs % 60;
  return `${mins} د ${remaining} ث`;
}

const stageIndex = { recording: 0, processing: 1, refining: 2, done: 3, idle: -1 };

const showRefiningStage = computed(() => {
  return pipelineStage.value === 'refining' || refiningDuration.value > 0;
});

const timelineStages = computed(() => {
  const stages = [
    { key: 'recording', label: 'التسجيل', icon: 'i-lucide-mic', duration: formatSecs(recordingDuration.value) },
    { key: 'processing', label: 'المعالجة', icon: 'i-lucide-cpu', duration: formatMs(processingDuration.value) },
  ];
  if (showRefiningStage.value) {
    stages.push({ key: 'refining', label: 'التحسين', icon: 'i-lucide-sparkles', duration: formatMs(refiningDuration.value) });
  }
  return stages;
});

function getStageState(stageKey) {
  const current = stageIndex[pipelineStage.value] ?? -1;
  const target = stageIndex[stageKey];
  if (pipelineStage.value === stageKey) return 'active';
  if (pipelineStage.value === 'done' || current > target) return 'completed';
  return 'pending';
}

const toast = useToast();

async function handleCopy() {
  if (!lastResult.value) return;
  await navigator.clipboard.writeText(lastResult.value);
  toast.add({ title: "تم النسخ", icon: "i-lucide-check" });
}
</script>

<template>
  <UDashboardPanel id="main">
    <template #header>
      <UDashboardNavbar title="الرئيسية">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <template #body>
      <div class="flex flex-col items-center justify-center gap-8 py-12">
        <div class="text-center">
          <h1 class="text-3xl font-bold mb-2">إملاء صوتي عربي</h1>
          <p class="text-muted">تحويل الصوت إلى نص باستخدام Whisper</p>
        </div>

        <div class="flex flex-col items-center gap-4">
          <UButton
            :icon="isRecording ? 'i-lucide-mic-off' : 'i-lucide-mic'"
            :color="statusColor"
            :loading="isProcessing"
            size="xl"
            class="rounded-full p-6!"
            @click="toggleDictation"
          />

          <UBadge :color="statusColor" variant="subtle" size="xl">
            {{ statusText }}
          </UBadge>
        </div>

        <div
          v-if="isRecording"
          class="flex flex-col items-center gap-4 w-full max-w-md"
        >
          <div class="flex items-center gap-3">
            <span class="relative flex h-3 w-3">
              <span
                class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"
              />
              <span
                class="relative inline-flex rounded-full h-3 w-3 bg-red-500"
              />
            </span>
            <span class="text-2xl font-mono font-bold tabular-nums">{{
              formattedDuration
            }}</span>
          </div>

          <div
            class="flex items-end justify-center gap-0.5 h-16 w-full max-w-xs"
          >
            <div
              v-for="(level, i) in audioLevels"
              :key="i"
              class="flex-1 min-w-0.5 max-w-1.5 bg-red-500 rounded-t transition-all duration-150"
              :style="{
                height: `${Math.max(4, level * 100)}%`,
                opacity: 0.4 + (i / audioLevels.length) * 0.6,
              }"
            />
          </div>
        </div>

        <div
          v-if="isRecording && silenceCountdown"
          class="flex flex-col items-center gap-2 w-full max-w-xs"
        >
          <div class="w-full bg-muted/20 rounded-full h-2 overflow-hidden">
            <div
              class="h-full bg-amber-500 rounded-full transition-all duration-300"
              :style="{ width: `${(silenceCountdown.remaining / silenceCountdown.total) * 100}%` }"
            />
          </div>
          <p class="text-sm text-amber-600 dark:text-amber-400 flex items-center gap-1.5">
            <UIcon
              name="i-lucide-timer"
              class="size-4"
            />
            <span>إيقاف تلقائي خلال {{ silenceCountdown.remaining }} ث</span>
          </p>
        </div>

        <div v-if="error" class="w-full max-w-lg">
          <UAlert
            color="error"
            icon="i-lucide-alert-circle"
            :title="String(error)"
          />
        </div>

        <div
          v-if="sessionActive"
          class="w-full max-w-2xl"
        >
          <div class="flex items-center justify-center gap-0">
            <div
              v-for="(stage, i) in timelineStages"
              :key="stage.key"
              class="flex items-center"
            >
              <div class="flex flex-col items-center gap-1.5 min-w-20">
                <div
                  class="size-9 rounded-full flex items-center justify-center border-2 transition-all duration-300"
                  :class="{
                    'border-primary bg-primary/10 text-primary': getStageState(stage.key) === 'active',
                    'border-success bg-success/10 text-success': getStageState(stage.key) === 'completed',
                    'border-muted bg-muted/5 text-muted': getStageState(stage.key) === 'pending',
                  }"
                >
                  <UIcon
                    v-if="getStageState(stage.key) === 'completed'"
                    name="i-lucide-check"
                    class="size-4"
                  />
                  <UIcon
                    v-else
                    :name="stage.icon"
                    class="size-4"
                    :class="{ 'animate-pulse': getStageState(stage.key) === 'active' }"
                  />
                </div>
                <span
                  class="text-xs font-medium transition-colors"
                  :class="{
                    'text-primary': getStageState(stage.key) === 'active',
                    'text-success': getStageState(stage.key) === 'completed',
                    'text-muted': getStageState(stage.key) === 'pending',
                  }"
                >
                  {{ stage.label }}
                </span>
                <span
                  v-if="getStageState(stage.key) !== 'pending'"
                  class="text-xs tabular-nums font-mono"
                  :class="{
                    'text-primary': getStageState(stage.key) === 'active',
                    'text-muted': getStageState(stage.key) === 'completed',
                  }"
                >
                  {{ stage.duration }}
                </span>
              </div>
              <div
                v-if="i < timelineStages.length - 1"
                class="w-12 h-0.5 mb-8 transition-colors duration-300"
                :class="{
                  'bg-success': getStageState(timelineStages[i + 1].key) === 'completed' || getStageState(timelineStages[i + 1].key) === 'active',
                  'bg-muted/30': getStageState(timelineStages[i + 1].key) === 'pending',
                }"
              />
            </div>
          </div>
        </div>

        <div v-if="isRefining && refiningText" class="w-full max-w-2xl">
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UIcon
                  name="i-lucide-sparkles"
                  class="size-4 animate-pulse text-yellow-500"
                />
                <span class="font-semibold">جارٍ تحسين النص...</span>
              </div>
            </template>

            <p class="text-lg leading-relaxed whitespace-pre-wrap">
              {{ refiningText }}
            </p>
          </UCard>
        </div>

        <div v-if="lastResult" class="w-full max-w-2xl">
          <UCard>
            <template #header>
              <div class="flex items-center justify-between">
                <span class="font-semibold">آخر نتيجة</span>
                <UButton
                  icon="i-lucide-copy"
                  color="neutral"
                  variant="ghost"
                  size="sm"
                  @click="handleCopy"
                />
              </div>
            </template>

            <p class="text-lg leading-relaxed whitespace-pre-wrap">
              {{ lastResult }}
            </p>
          </UCard>
        </div>

        <div class="text-center text-sm text-muted">
          <UKbd>Z</UKbd> + <UKbd>Z</UKbd>
          <span class="mr-2">اضغط مرتين بسرعة لبدء/إيقاف الإملاء</span>
        </div>
      </div>
    </template>
  </UDashboardPanel>
</template>
