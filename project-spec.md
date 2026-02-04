# Project Specification: إملاء صوتي عربي

## Quick Reference | مرجع سريع

| الحقل | القيمة |
|-------|--------|
| نوع المشروع | Fullstack Application |
| طبيعة المشروع | أداة |
| بيئات التشغيل | web, desktop, system |
| مستوى الذكاء | ذكاء أساسي |
| حجم المشروع | متوسط |
| الاسم التقني | arabic-voice-dictation |
| البنية | monolith |
| واجهات التواصل | local-ipc, tauri-commands |
| Frontend | Nuxt (Latest) |
| Backend | Tauri (Latest) |
| Database | SQLite (Latest) |
| UI Library | Nuxt UI (Latest) |
| Package Manager | pnpm |

---

## 1. Project Overview | نظرة عامة

### 1.1 اسم المشروع
```
الاسم: إملاء صوتي عربي
الاسم التقني: arabic-voice-dictation
```

### 1.2 المشكلة
```
الكتابة باللغة العربية بطيئة ومرهقة خاصة للنصوص الطويلة ولا توجد أداة إملاء عربي دقيقة تعمل بدون إنترنت على ويندوز
```

### 1.3 الحل
```
تطبيق سطح مكتب يعمل في الخلفية يحول الصوت إلى نص عربي وإنجليزي بدقة عالية باستخدام Whisper Large V3 محليًا مع اختصار كيبورد قابل للتخصيص للبدء والإيقاف ويكتب النص مباشرة في أي برنامج مفتوح مع سجل وإحصائيات وترقيم تلقائي
```

### 1.4 المستخدمين المستهدفين
```
كتاب المحتوى والصحفيون والطلاب والموظفون والمبرمجون والمطورون العرب الذين يحتاجون للكتابة بالعربية والإنجليزية بسرعة ودقة
- النوع: أفراد
- المستوى: مبتدئ
- اللغة: عربي (RTL - من اليمين لليسار)
```

---

## 2. User Stories | قصص المستخدم

### 2.1 مستخدم عادي
```
- كـ مستخدم عادي أريد الضغط على اختصار كيبورد قابل للتخصيص لبدء وإيقاف الإملاء لأجل الكتابة بسرعة بدون لمس الكيبورد
- كـ مستخدم عادي أريد أن يُكتب النص تلقائيًا في البرنامج المفتوح لأجل عدم الحاجة لنسخ ولصق
- كـ مستخدم عادي أريد أن يعمل التطبيق بدون إنترنت لأجل الخصوصية والاستخدام في أي مكان
- كـ مستخدم عادي أريد رؤية سجل الإملاءات السابقة لأجل نسخ نص سابق عند الحاجة
- كـ مستخدم عادي أريد رؤية إحصائيات استخدامي لأجل معرفة مدى استفادتي من التطبيق
- كـ مستخدم عادي أريد إضافة علامات الترقيم تلقائيًا لأجل الحصول على نص مكتمل ومنسق
```



---

## Technical Requirements | المتطلبات التقنية

### Tech Stack
```yaml
Frontend: Nuxt (Latest)
Backend: Tauri (Latest)
Database: SQLite (Latest)
UI Library: Nuxt UI (Latest)
```

### Architecture
```
البنية: Monolith - تطبيق واحد
```

---

## AI Configuration | إعدادات الذكاء الاصطناعي

### Intelligence Level: ذكاء أساسي

### AI Domains
```yaml
- speech-to-text
```

### AI Models
```yaml
- name: Whisper
  openSource: true
  api: false
  offline: true
```

### Supported Languages
```
العربية, English
```

### Hardware Preference: gpu-preferred

---

## Desktop/System Capabilities | إمكانيات النظام

### Enabled Capabilities
```yaml
- الوصول لنظام الملفات
- الميكروفون
- اختصارات لوحة المفاتيح
- العمل في الخلفية
- التشغيل التلقائي
```

---

## 4. Database Design | تصميم قاعدة البيانات

### 4.1 Database Schema
```sql
-- جدول الإعدادات العامة للتطبيق
CREATE TABLE settings (
  id INT PRIMARY KEY NOT NULL,
  key VARCHAR(255) UNIQUE NOT NULL,
  value JSON NOT NULL,
  updatedAt TIMESTAMP NOT NULL
);

-- سجل الإملاءات السابقة
CREATE TABLE dictation_history (
  id INT PRIMARY KEY NOT NULL,
  text TEXT NOT NULL,
  duration INT NOT NULL,
  language VARCHAR(255) NOT NULL,
  createdAt TIMESTAMP NOT NULL
);

-- إحصائيات الاستخدام اليومية
CREATE TABLE usage_stats (
  id INT PRIMARY KEY NOT NULL,
  date TIMESTAMP UNIQUE NOT NULL,
  totalDictations INT NOT NULL,
  totalWords INT NOT NULL,
  totalDuration INT NOT NULL
);

```

---

## API Design | تصميم الـ API

### API Style: REST
Base Prefix: /api

### Tauri Dictation Commands
> أوامر Tauri للتحكم بالإملاء الصوتي
```yaml
POST /dictation/start:
  description: بدء تسجيل الصوت والإملاء
  auth: none



POST /dictation/stop:
  description: إيقاف التسجيل وتحويل الصوت إلى نص
  auth: none



GET /dictation/status:
  description: جلب حالة الإملاء الحالية
  auth: none



```

### Settings Commands
> أوامر إدارة إعدادات التطبيق
```yaml
GET /settings/all:
  description: جلب جميع الإعدادات
  auth: none



POST /settings/update:
  description: تحديث إعداد معين
  auth: none



```

### History Commands
> أوامر سجل الإملاءات
```yaml
GET /history/list:
  description: جلب سجل الإملاءات السابقة
  auth: none



GET /history/stats:
  description: جلب إحصائيات الاستخدام
  auth: none



DELETE /history/:id:
  description: حذف إملاء من السجل
  auth: none



```


---

## Frontend Pages | صفحات الواجهة

### Nuxt UI Template: Dashboard
```yaml
Template: Dashboard
Framework: nuxt
Description: Multi-column admin dashboard interface
Preview: https://dashboard-template.nuxt.dev
GitHub: https://github.com/nuxt-ui-templates/dashboard
Features:
  - Works with SaaS template
  - Charts and date pickers
  - Multi-column layout
```

> استخدم هذا القالب كأساس للواجهة. قم بتشغيل `/nuxt-ui-remote:setup_project_with_template` واختر قالب "Dashboard".

### Modules إضافية

#### الإعدادات
> صفحة الإعدادات العامة للتطبيق شاملة اختصارات الكيبورد واختيار نموذج Whisper واللغة
> Base Path: /settings
> Module Type: view-edit-combined

```yaml
/settings:
  name: الإعدادات العامة
  description: عرض قائمة الإعدادات مع البحث والفلترة
  auth: public

/settings/create:
  name: إضافة الإعدادات
  description: نموذج إضافة عنصر جديد
  auth: public

/settings/:id:
  name: مشاهدة وتعديل الإعدادات
  description: عرض وتعديل عنصر واحد
  auth: public

```

#### السجل
> سجل جميع الإملاءات السابقة مع إمكانية النسخ والبحث
> Base Path: /history
> Module Type: index-only

```yaml
/history:
  name: سجل الإملاءات
  description: عرض سجل جميع الإملاءات السابقة مع البحث والنسخ
  auth: public

```

#### الإحصائيات
> إحصائيات الاستخدام اليومية والأسبوعية
> Base Path: /stats
> Module Type: index-only

```yaml
/stats:
  name: إحصائيات الاستخدام
  description: عرض إحصائيات الاستخدام اليومية والأسبوعية بالرسوم البيانية
  auth: public

```


---

## 7. Features List | قائمة المميزات

### 7.1 MVP Features
```
[ ] تحويل الصوت إلى نص عربي وإنجليزي باستخدام Whisper Large V3 محليًا
[ ] اختصار كيبورد عام قابل للتخصيص لبدء وإيقاف الإملاء
[ ] كتابة النص مباشرة في البرنامج النشط عبر محاكاة الكيبورد
[ ] العمل في الخلفية مع أيقونة في شريط المهام System Tray
[ ] واجهة إعدادات بسيطة بالعربي RTL
[ ] حفظ سجل الإملاءات السابقة مع إمكانية البحث والنسخ
[ ] إحصائيات الاستخدام اليومية عدد الكلمات والوقت
[ ] إضافة علامات الترقيم تلقائيًا
[ ] إشعارات صوتية عند بدء وإيقاف الإملاء
```

### 7.2 Future Features
```
[ ] دعم لهجات عربية متعددة مع تخصيص النموذج
[ ] تصحيح تلقائي للنص باستخدام نموذج لغوي
[ ] تصدير سجل الإملاءات إلى ملفات نصية
[ ] مزامنة الإعدادات بين أجهزة متعددة
```

---

## 9. Edge Cases | الحالات الاستثنائية

### 1. ضوضاء عالية في الخلفية
```
عرض مؤشر جودة الصوت وتحذير المستخدم
```

### 2. نفاد ذاكرة الجهاز أثناء تشغيل النموذج
```
تحديد مدة الإملاء القصوى وتنبيه المستخدم واقتراح نموذج أصغر
```

### 3. عدم وجود GPU في الجهاز
```
التبديل التلقائي لوضع CPU مع تحذير بأن السرعة ستكون أبطأ
```

### 4. يحاول المستخدم بدء الإملاء الصوتي، لكن لا يوجد ميكروفون متصل بالجهاز أو أن صلاحية الوصول للميكروفون مرفوضة من النظام.
```
إيقاف عملية الإملاء فورًا  عرض رسالة واضحة للمستخدم توضح المشكلة  اقتراح حل (توصيل ميكروفون / تفعيل الصلاحية من إعدادات النظام)  عدم تشغيل أي عملية في الخلفية بدون ميكروفون
```

### 5. يضغط المستخدم اختصار الإملاء أثناء وجود تسجيل صوتي يعمل بالفعل.
```
تجاهل الأمر الثاني  أو إيقاف التسجيل الحالي ثم بدء تسجيل جديد (حسب الإعداد)  تحديث مؤشر الحالة بوضوح (يسجل / متوقف)
```

### 6. تشغيل التطبيق على جهاز بطيء يؤدي إلى بطء في تحويل الصوت إلى نص.
```
الاستمرار في المعالجة بدون فشل  إظهار حالة “جارٍ المعالجة” للمستخدم  عدم تجميد الواجهة  السماح بإلغاء العملية يدويًا
```

### 7. يقوم المستخدم بالتسجيل ولكن الصوت منخفض جدًا أو لا يحتوي على كلام مفهوم.
```
إنهاء المعالجة بشكل طبيعي  إخطار المستخدم بأن الصوت غير واضح  عدم إدخال نص فارغ في المكان النشط  عدم اعتبار العملية فشل تقني
```

### 8. يقوم المستخدم بإغلاق التطبيق أو إيقاف النظام أثناء تسجيل الصوت أو أثناء تحويله إلى نص.
```
إيقاف التسجيل فورًا  تنظيف أي ملفات مؤقتة  عدم ترك عمليات معلقة في الخلفية  إعادة التطبيق إلى حالة مستقرة عند التشغيل التالي
```



---

## Dependencies | المتطلبات

### Package Manager: pnpm

### Backend Dependencies
```bash
pnpm add whisper-rs whisper.cpp better-sqlite3
```
```json
{
  "dependencies": {
    "whisper-rs": "latest",
    "whisper.cpp": "latest",
    "better-sqlite3": "latest"
  }
}
```

### Frontend Dependencies
```bash
pnpm add nuxt @nuxt/ui @tauri-apps/api chart.js
```
```json
{
  "dependencies": {
    "nuxt": "latest",
    "@nuxt/ui": "latest",
    "@tauri-apps/api": "latest",
    "chart.js": "latest"
  }
}
```

### AI Dependencies
```bash
pnpm add whisper-rs whisper.cpp
```

### System Dependencies
```bash
pnpm add tauri tauri-plugin-global-shortcut tauri-plugin-autostart tauri-plugin-shell tauri-plugin-notification
```

### Build Dependencies
```bash
pnpm add -D vite @vitejs/plugin-vue cargo
```

---

## 12. Environment Variables | متغيرات البيئة

### .env
```env
# Server port (required)
PORT=3001

# Database host (required)
DB_HOST=localhost

# Database name (required)
DB_NAME=myapp_db

# JWT secret key (required)
JWT_SECRET=your-secret-key

```

---

## Development Guidelines | إرشادات التطوير

### TypeScript Mode
```
استخدم JavaScript فقط (ما عدا ملفات *.config.ts)
```

### المحاذير والتأكيدات
```
1. قبل إنشاء أي صفحة أو component، قم بمراجعة MCP servers الخاصة بـ Nuxt & Nuxt UI
2. استخدم Zod schemas للتحقق من جميع المدخلات في Backend
3. اتبع نمط RTL في جميع عناصر الواجهة العربية
4. لا تستخدم TypeScript في ملفات Vue ما عدا إذا تم اختيار "TypeScript كامل"
5. استخدم composables بدلاً من mixins أو store مباشر
6. تجنب استخدام any في TypeScript - استخدم أنواع محددة
7. تأكد من معالجة الأخطاء (Error handling) في كل API call
8. أضف loading states لكل عملية async
9. استخدم دائماً أحدث إصدارات Nuxt & Nuxt UI & Zod
```