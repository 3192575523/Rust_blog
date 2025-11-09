<template>
  <div class="max-w-3xl mx-auto p-6">
    <h1 class="text-3xl font-bold mb-2">{{ post?.title }}</h1>

    <div class="text-sm text-gray-500 mb-4">
      <span v-if="post?.published_at">发布时间：{{ post?.published_at }}</span>
      <span class="ml-2">可见性：{{ post?.visibility }}</span>
      <span class="ml-2">状态：{{ post?.status }}</span>
    </div>

    <div v-if="isOwner" class="mb-4 space-x-2">
      <router-link :to="`/edit/${post?.id}`" class="px-3 py-1 rounded bg-blue-600 text-white">编辑</router-link>
      <button @click="doDelete" class="px-3 py-1 rounded bg-red-600 text-white">删除</button>
    </div>

    <div v-if="err" class="text-red-600 mb-3">{{ err }}</div>

    <!-- 渲染正文；渲染后会触发 MathJax typeset -->
    <div ref="contentRef" v-html="post?.body_html" class="prose"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { getPublicBySlug, deletePost } from "../services/posts";
import type { Post } from "../services/posts";
import { getCurrentUserId, getToken } from "../stores/auth";

const route = useRoute();
const router = useRouter();

const post = ref<Post | null>(null);
const err = ref("");

const contentRef = ref<HTMLElement | null>(null);

const isOwner = computed(() => {
  const uid = getCurrentUserId();
  return !!(post.value && uid && post.value.author_id === uid);
});

async function typesetMath() {
  // MathJax 在 index.html 以全局脚本形式加载
  const MJ = (window as any).MathJax;
  if (!MJ) return; // 还未加载则跳过
  try {
    if (MJ.typesetPromise) {
      await MJ.typesetPromise([contentRef.value]); // 只排版正文容器
    } else if (MJ.typeset) {
      MJ.typeset([contentRef.value]);
    }
  } catch (e) {
    // 静默失败，避免影响页面
    console.warn("MathJax typeset failed:", e);
  }
}

async function load() {
  err.value = "";
  try {
    post.value = await getPublicBySlug(route.params.slug as string);
    await nextTick();
    await typesetMath();
  } catch (e: any) {
    err.value = e?.message || "加载失败";
  }
}

async function doDelete() {
  if (!post.value) return;
  if (!getToken()) return alert("请先登录");
  if (!confirm("确定删除？")) return;
  await deletePost(post.value.id);
  router.push("/");
}

onMounted(load);

// 如果你在同一个组件里切换 slug（如从 A 文章点进 B 文章）需要重载并重新 typeset
watch(
  () => route.params.slug,
  async () => {
    await load();
  }
);
</script>

<style scoped>
.prose :where(img) {
  max-width: 100%;
  height: auto;
}
</style>
