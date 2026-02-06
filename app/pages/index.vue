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

        <div v-if="error" class="w-full max-w-lg">
          <UAlert
            color="error"
            icon="i-lucide-alert-circle"
            :title="String(error)"
          />
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
