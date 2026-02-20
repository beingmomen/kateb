<div dir="rtl" align="right">

# تقرير شامل: نماذج التعرف على الكلام العربي (Arabic ASR Models)

**التاريخ**: 2026-02-18
**المشروع**: إملاء صوتي عربي (arabic-voice-dictation)
**السياق**: مشروعنا يستخدم `whisper-rs` مع Tauri 2

---

## 1. ترتيب النماذج العالمي (Open Universal Arabic ASR Leaderboard)

المصدر: [Arabic ASR Leaderboard](https://huggingface.co/spaces/elmresearchcenter/open_universal_arabic_asr_leaderboard)
يقيّم النماذج على 5 مجموعات بيانات متعددة اللهجات: SADA, Common Voice 18.0, MASC Clean, MASC Noisy, MGB-2

</div>

| الترتيب | النموذج | متوسط WER | الحجم | ملاحظات |
|:-------:|---------|:---------:|:-----:|---------|
| 1 | nvidia/conformer-ctc-large-ar (+LM) | **25.71%** | 115M | الأفضل إطلاقاً |
| 2 | nvidia/conformer-ctc-large-ar (greedy) | 27.46% | 115M | بدون language model |
| 3 | **openai/whisper-large-v3** | **29.87%** | 1,550M | أفضل نموذج Whisper |
| 4 | facebook/seamless-m4t-v2-large | 32.55% | ~2,300M | متعدد المهام |
| 5 | **openai/whisper-large-v3-turbo** | **33.30%** | 809M | أسرع 6x من v3 |
| 6 | openai/whisper-large-v2 | 34.04% | 1,550M | - |
| 7 | openai/whisper-large | 36.65% | 1,550M | - |
| 8 | asafaya/hubert-large-arabic-ft | 39.29% | ~315M | أقل WER على Common Voice |
| 9 | openai/whisper-medium | 39.60% | 769M | - |
| 10 | facebook/mms-1b-all | 47.86% | 1,000M | 1,107 لغة |
| 11 | whitefox123/w2v-bert-2.0-ft | 48.62% | ~600M | - |
| 12 | openai/whisper-small | 52.18% | 244M | - |
| 13 | jonatasgrosman/wav2vec2-large-xlsr-ft | 54.63% | 315M | - |
| 14 | speechbrain/wav2vec2-large-ft | 60.15% | 315M | - |

---

<div dir="rtl" align="right">

## 2. نماذج Whisper الأصلية - الأداء على العربية

### أ. المقارنة حسب الحجم

</div>

| النموذج | المعاملات | حجم الملف | RAM | WER متوسط (كل اللهجات) | WER على FLEURS (فصحى) |
|---------|:---------:|:---------:|:---:|:---------------------:|:--------------------:|
| whisper-tiny | 39M | 75MB | ~400MB | ~65%+ (تقدير) | غير مختبر |
| whisper-base | 74M | 142MB | ~500MB | ~50%+ (تقدير) | غير مختبر |
| whisper-small | 244M | 466MB | ~1GB | 52.18% | 30.11% |
| whisper-medium | 769M | 1.5GB | ~2.5GB | 39.60% | 19.10% |
| whisper-large-v2 | 1,550M | 3GB | ~5GB | 34.04% | 17.14% |
| **whisper-large-v3** | **1,550M** | **3GB** | **~5GB** | **29.87%** | **~11.5%** |
| **whisper-large-v3-turbo** | **809M** | **1.5GB** | **~2.6GB** | **33.30%** | - |

<div dir="rtl" align="right">

### ب. ملاحظات مهمة

- **القفزة الأكبر**: من small إلى medium (~24-36% تحسن نسبي) - أعلى قيمة مقابل الحجم
- **large-v3-turbo**: قريب جداً من large-v2 (33.30% vs 34.04%) رغم أن لديه 4 طبقات decoder فقط بدلاً من 32
- **مشكلة معروفة في v3-turbo**: كلمة "نعم" تُكتب أحياناً كـ "Naah"/"Naahe"
- **Fine-tuning أهم من الحجم**: whisper-small المُدرَّب على MGB2 بـ WER 16.69% يتفوق على whisper-large-v2 بدون تدريب بـ WER 17.14%

---

## 3. نماذج Whisper المُدرَّبة على العربية (Fine-Tuned)

</div>

| النموذج | الأساس | WER | بيانات التدريب | اللهجة |
|---------|--------|:---:|---------------|--------|
| ARBML/whisper-largev2.1 | large-v2 | **11.60%** (FLEURS) | MGB2 (1200 ساعة) | فصحى |
| mohammed/whisper-large-arabic-cv-11 | large | **12.62%** | Common Voice 11 | فصحى |
| MohammedNasri/WHISPER-LARGE-ARABIC | large | 15.93% | Common Voice 11 | فصحى |
| speechbrain/asr-whisper-large-v2-commonvoice-ar | large-v2 | 16.96% | Common Voice 10 | فصحى |
| anaszil/whisper-large-v3-turbo-darija | large-v3-turbo | 24.88% | خاص | **دارجة مغربية** |
| HebArabNlpProject/WhisperLevantine | large-v3 | 33.00% | 1200 ساعة | **شامية** |
| MohamedRashad/Arabic-Whisper-CodeSwitching-Edition | large-v2 | غير منشور | 12.5k عينة | عربي-إنجليزي |
| MAdel121/whisper-small-egyptian-arabic | small | غير منشور | بيانات مصرية | **مصرية** |
| MAdel121/whisper-medium-egy | medium | غير منشور | 72 ساعة | **مصرية** |
| Seyfelislem/whisper-medium-arabic | medium | 18.29% | غير محدد | فصحى |
| ayoubkirouane/whisper-small-ar | small | غير منشور | Common Voice 11 | فصحى |

---

<div dir="rtl" align="right">

## 4. نماذج غير Whisper

### أ. NVIDIA FastConformer - الأفضل مطلقاً

</div>

| الخاصية | القيمة |
|---------|--------|
| **النموذج** | `nvidia/stt_ar_fastconformer_hybrid_large_pc_v1.0` |
| **الحجم** | 115M فقط (أصغر 13x من Whisper Large!) |
| **WER** | 8-12% على المعايير القياسية، 25.71% متوسط على كل اللهجات |
| **الترخيص** | CC-BY-4.0 |
| **التدريب** | ~760 ساعة (MASC 690h + CV17 65h + FLEURS 5h) |
| **العيب** | يحتاج NeMo framework، غير متوافق مع whisper-rs |

<div dir="rtl" align="right">

### ب. Meta SeamlessM4T v2

</div>

| الخاصية | القيمة |
|---------|--------|
| **النموذج** | `facebook/seamless-m4t-v2-large` |
| **الحجم** | ~2,300M |
| **WER** | 32.55% متوسط |
| **الميزة** | يدعم الترجمة + التحويل + التوليد الصوتي، ~100 لغة |
| **العيب** | ضخم جداً |

<div dir="rtl" align="right">

### ج. MBZUAI ArTST - متخصص بالعربية

</div>

| الخاصية | القيمة |
|---------|--------|
| **النماذج** | `MBZUAI/artst_asr_v1` / `v2` / `v3` |
| **الحجم** | 200M فقط |
| **البنية** | SpeechT5 |
| **الميزة** | 17 نموذج مخصص للهجات، نصف معدل خطأ Whisper |
| **يدعم** | مصري، خليجي، شامي، مغاربي + فصحى |
| **MGB-2 WER** | ~12.8% (v1) |

<div dir="rtl" align="right">

### د. Meta MMS (Massively Multilingual Speech)

</div>

| الخاصية | القيمة |
|---------|--------|
| **النموذج** | `facebook/mms-1b-all` |
| **الحجم** | 1B |
| **WER** | 47.86% متوسط |
| **الميزة** | يدعم 1,107 لغة عبر محولات خاصة بكل لغة |
| **العيب** | جودة عربية متوسطة |

<div dir="rtl" align="right">

### هـ. Meta OmniASR (جديد 2025)

</div>

| الخاصية | القيمة |
|---------|--------|
| **النماذج** | `facebook/omniASR-CTC-7B` / `LLM-7B` / `CTC-1B` / `LLM-1B` / `CTC-300M` / `LLM-300M` |
| **اللغات** | 1,600+ لغة |
| **الميزة** | أكثر نظام ASR تنوعاً لغوياً |

<div dir="rtl" align="right">

### و. Qwen3-ASR (جديد 2026)

</div>

| الخاصية | القيمة |
|---------|--------|
| **النموذج** | `Qwen/Qwen3-ASR-1.7B` / `0.6B` |
| **اللغات** | 52 لغة |
| **دقة تعريف اللغة** | 97.9% |
| **الميزة** | منافس للحلول التجارية |

<div dir="rtl" align="right">

### ز. Voxtral من Mistral (جديد 2025)

</div>

| الخاصية | القيمة |
|---------|--------|
| **النماذج** | `mistralai/Voxtral-Mini-4B-Realtime-2602` / `Small-24B` / `Mini-3B` |
| **الترخيص** | Apache 2.0 |
| **اللغات** | 13 لغة (العربية من ضمنها) |
| **الميزة** | يوجد تطبيق Rust خالص (Burn framework) |
| **العيب** | جديد، 4B كبير نسبياً |

---

<div dir="rtl" align="right">

## 5. أداء اللهجات العربية بالتفصيل

### أداء Whisper Large-v3 على اللهجات المختلفة

</div>

| اللهجة | WER | مستوى الموارد | ملاحظات |
|--------|:---:|:------------:|---------|
| **الفصحى (MSA)** | **11.56%** | عالي | أفضل أداء |
| **الأردنية** | **39.68%** | متوسط | الأقرب للفصحى صوتياً |
| **المصرية** | 48.95% | متوسط | أكثر لهجة موارداً |
| **الفلسطينية** | 50.20% | منخفض | - |
| **الإماراتية** | 52.88% | منخفض جداً | - |
| **اليمنية** | 59.45% | منخفض جداً | - |
| **المغربية** | 83.05% | منخفض | تأثير الفرنسية |
| **الجزائرية** | 84.14% | منخفض | تأثير الفرنسية |
| **الموريتانية** | **85.68%** | منخفض جداً | الأسوأ أداءً |

<div dir="rtl" align="right">

> **ملاحظة**: Code-switching (خلط اللغات) يسبب فشل كامل - Whisper يسجل 90%+ WER على المقاطع المختلطة عربي-فرنسي

---

## 6. نماذج مخصصة لكل لهجة

### المصرية (EGY) - الأفضل تغطية
- `MAdel121/whisper-small-egyptian-arabic` - Whisper Small مُدرَّب
- `MAdel121/whisper-medium-egy` - Whisper Medium، 72 ساعة بيانات
- ARBML/klaam - مكتبة Python تدعم المصرية (`lang='egy'`)
- FastConformer Egyptian - فاز بمسابقة MTC-AIC 2024

### الشامية (LEV) - تغطية جيدة
- `HebArabNlpProject/WhisperLevantine` - 1200 ساعة، WER 33%
- ArTST v2 - يغطي سوريا، الأردن، لبنان، فلسطين
- IWSLT 2024 - مسار مخصص للشامية الشمالية

### الخليجية (GLF) - تغطية ضعيفة
- **لا يوجد نموذج مخصص مفتوح المصدر** على HuggingFace
- ArTST v2 يغطي السعودية، الكويت، الإمارات، عُمان، قطر، العراق، اليمن
- Munsit (تجاري) - أفضل خيار للخليجية
- Lisan + DeepVA - يغطي الكويتية، البحرينية، القطرية، الإماراتية، السعودية

### المغاربية (NOR) - الأصعب
- `anaszil/whisper-large-v3-turbo-darija` - WER 24.88% (دارجة مغربية)
- `ychafiqui/whisper-medium-darija` - دارجة مغربية
- MGB-5 Challenge - أفضل نظام حقق 59.4% WER على المغربية
- مشكلة code-switching مع الفرنسية تسبب فشل كبير

---

## 7. مقارنة شاملة: المميزات والعيوب

### نماذج Whisper (متوافقة مع مشروعنا عبر whisper-rs)

</div>

| النموذج | المميزات | العيوب |
|---------|---------|--------|
| **whisper-large-v3** | أفضل WER للعربية (29.87%)، يدعم كل اللهجات | ضخم (3GB)، بطيء، هلوسات في الصمت |
| **whisper-large-v3-turbo** | سريع 6x، حجم معقول (1.5GB)، WER قريب من v2 | أضعف قليلاً من v3، مشكلة "نعم" |
| whisper-medium | حجم معقول، أداء مقبول للفصحى | ضعيف على اللهجات (39.60%) |
| whisper-small | خفيف (466MB) وسريع | WER عالي (52%)، غير عملي للهجات |
| whisper-tiny/base | صغير جداً | **غير صالح للعربية** |

<div dir="rtl" align="right">

### نماذج غير Whisper (غير متوافقة مباشرة مع whisper-rs)

</div>

| النموذج | المميزات | العيوب |
|---------|---------|--------|
| **NVIDIA FastConformer** | الأفضل مطلقاً (25.71%)، صغير جداً (115M) | يحتاج NeMo، غير متوافق مع whisper-rs |
| **ArTST** | متخصص عربي، 17 لهجة، صغير (200M) | يحتاج SpeechT5، غير متوافق مع whisper-rs |
| **Voxtral** | ترخيص Apache 2.0، تطبيق Rust موجود | جديد، 4B كبير |
| **Qwen3-ASR** | 52 لغة، دقة عالية | 1.7B، جديد |
| SeamlessM4T | متعدد المهام، 100 لغة | ضخم (2.3B)، ليس الأفضل للعربية |
| MMS | 1,107 لغة | جودة عربية متوسطة (47.86%) |

---

<div dir="rtl" align="right">

## 8. التوصية النهائية لمشروعنا

### للاستخدام الحالي (whisper-rs)

</div>

| الأولوية | النموذج | السبب |
|:--------:|---------|-------|
| **الأفضل جودة** | `openai/whisper-large-v3` | أقل WER (29.87%)، لكن 3GB وبطيء |
| **أفضل توازن** ⭐ | `openai/whisper-large-v3-turbo` | 1.5GB فقط، أسرع 6x، WER 33.30% |
| **للفصحى فقط** | `mohammed/whisper-large-arabic-cv-11` | WER 12.62% على الفصحى |
| **خفيف** | `openai/whisper-medium` | 1.5GB، WER 39.60% |

<div dir="rtl" align="right">

### للمستقبل (يستحق المتابعة)

1. **Voxtral** - يوجد تطبيق Rust (Burn framework)، Apache 2.0
2. **NVIDIA FastConformer** - إذا أمكن تحويله لـ ONNX واستخدامه من Rust
3. **ArTST** - 17 لهجة عربية مخصصة

### النتيجة الأهم

> **whisper-large-v3-turbo** هو الخيار الأفضل لتطبيقنا حالياً - يوفر توازن ممتاز بين الجودة (33.30% WER) والسرعة (6x أسرع) والحجم (1.5GB). وهو بالفعل من ضمن النماذج المدعومة في مشروعنا.

> **لتحسين جودة اللهجات**: الحل الأفضل هو استخدام AI Refiner (Claude/OpenAI/Gemini) لتصحيح أخطاء Whisper، خاصة مع سياق اللهجة.

---

## المصادر

- [Open Universal Arabic ASR Leaderboard](https://huggingface.co/spaces/elmresearchcenter/open_universal_arabic_asr_leaderboard)
- [ArXiv Paper: Arabic ASR Leaderboard](https://arxiv.org/abs/2412.13788)
- [N-Shot Benchmarking of Whisper on Arabic (Interspeech 2023)](https://arxiv.org/abs/2306.02902)
- [Casablanca: Multidialectal Arabic ASR (EMNLP 2024)](https://arxiv.org/abs/2410.04527)
- [ArTST: Arabic Text-Speech Transformer](https://arxiv.org/abs/2411.05872)
- [ARBML/whisperar Benchmarks](https://github.com/ARBML/whisperar)
- [Munsit Arabic ASR](https://arxiv.org/abs/2504.12254)
- [Overcoming Data Scarcity in Multi-Dialectal Arabic ASR](https://arxiv.org/html/2506.02627v1)
- [Munsit at NADI 2025](https://arxiv.org/abs/2508.08912)
- [NVIDIA FastConformer Arabic](https://huggingface.co/nvidia/stt_ar_fastconformer_hybrid_large_pc_v1.0)
- [Qwen3-ASR](https://huggingface.co/Qwen/Qwen3-ASR-1.7B)
- [Voxtral Rust Implementation](https://github.com/TrevorS/voxtral-mini-realtime-rs)
- [ARBML/klaam](https://github.com/ARBML/klaam)

</div>
