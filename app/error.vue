<script setup>
const props = defineProps({
  error: Object
})

const handleError = () => clearError({ redirect: '/' })
</script>

<template>
  <div dir="rtl" class="h-screen flex items-center justify-center p-8 bg-gray-50 dark:bg-gray-950">
    <div class="max-w-md w-full text-center space-y-6">
      <div class="flex justify-center">
        <div class="w-20 h-20 bg-red-100 dark:bg-red-900/30 rounded-full flex items-center justify-center">
          <UIcon
            name="i-lucide-alert-triangle"
            class="size-10 text-red-500"
          />
        </div>
      </div>

      <div class="space-y-2">
        <h1 class="text-6xl font-bold text-red-500">
          {{ error?.statusCode || 500 }}
        </h1>
        <h2 class="text-xl font-semibold">
          حدث خطأ غير متوقع
        </h2>
        <p class="text-muted">
          {{ error?.message || 'عذراً، حدث خطأ أثناء تحميل الصفحة.' }}
        </p>
      </div>

      <div class="flex justify-center gap-3">
        <UButton
          icon="i-lucide-home"
          size="lg"
          @click="handleError"
        >
          العودة للرئيسية
        </UButton>
        <UButton
          icon="i-lucide-refresh-cw"
          variant="soft"
          size="lg"
          @click="clearError()"
        >
          إعادة المحاولة
        </UButton>
      </div>
    </div>
  </div>
</template>
