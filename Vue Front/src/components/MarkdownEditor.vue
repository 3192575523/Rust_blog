<template>
  <!-- 用一个容器把编辑器包起来，方便只 typeset 这一块 -->
  <div ref="wrapRef">
    <v-md-editor v-model="model" height="60vh" :on-upload-img="onUploadImg" />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, defineProps, defineEmits, nextTick, onMounted } from 'vue'
import { api } from '../api'

const props = defineProps<{ modelValue: string }>()
const emit = defineEmits<{ (e: 'update:modelValue', v: string): void }>()
const model = ref(props.modelValue)
watch(() => props.modelValue, v => (model.value = v))
watch(model, v => emit('update:modelValue', v))

// ---- 图片上传（保持你现有逻辑）----
const onUploadImg = async (files: File[], callback: (urls: string[]) => void) => {
  try {
    const file = files?.[0]
    if (!file) return callback([])
    const form = new FormData()
    form.append('file', file, file.name)
    const res = await api.post<{ files: string[] }>('/api/media', form)
    callback(Array.isArray(res.data?.files) ? res.data.files : [])
  } catch (err: any) {
    console.error(err)
    alert(err?.message || '上传失败')
    callback([])
  }
}

// ---- 关键：对编辑器容器进行 MathJax typeset ----
const wrapRef = ref<HTMLElement | null>(null)

async function typeset() {
  const MJ = (window as any).MathJax
  if (!MJ || !wrapRef.value) return
  try {
    await nextTick()
    if (MJ.typesetPromise) await MJ.typesetPromise([wrapRef.value])
    else if (MJ.typeset) MJ.typeset([wrapRef.value])
  } catch (e) {
    console.warn('MathJax typeset failed in editor:', e)
  }
}

onMounted(typeset)
// 每次内容变化都重排版（节流需求不大，MathJax v3 性能可以）
watch(model, typeset)
</script>
