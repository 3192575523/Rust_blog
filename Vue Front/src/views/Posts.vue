<!-- src/views/Posts.vue -->
<template>
  <div class="max-w-5xl mx-auto p-6">
    <!-- 登录后显示：我的私有文章 -->
    <section v-if="myPrivate.length">
      <h2 class="text-xl font-semibold mb-3">我的私有文章</h2>
      <ul class="divide-y mb-8">
        <li v-for="it in myPrivate" :key="it.id" class="py-3">
          <router-link :to="`/p/${it.slug}`" class="font-medium hover:underline">
            {{ it.title }}
          </router-link>
          <div class="text-xs text-gray-500 mt-1">
            <span class="mr-2">状态：{{ it.status }}</span>
            <span class="mr-2">可见性：{{ it.visibility }}</span>
            <span v-if="it.published_at">发布时间：{{ it.published_at }}</span>
          </div>
        </li>
      </ul>
    </section>

    <!-- 公开文章列表（原逻辑） -->
    <section>
      <h2 class="text-2xl font-bold mb-4">文章</h2>
      <div class="space-y-3">
        <div v-for="p in publicItems" :key="p.id" class="border p-3 rounded">
          <router-link :to="`/p/${p.slug}`" class="text-lg font-semibold hover:underline">
            {{ p.title }}
          </router-link>
          <div class="text-gray-600 text-sm mt-1" v-if="p.excerpt">{{ p.excerpt }}</div>
          <div class="text-gray-400 text-xs mt-1" v-if="p.published_at">发表于 {{ p.published_at }}</div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listPublic, type PublicListItem } from "../services/posts";
import { getToken } from "../stores/auth";
import { getMyPosts, type MyPostItem } from "../services/me";

// 公开文章
const publicItems = ref<PublicListItem[]>([]);

// 我的私有已发布文章（仅登录拉取）
const myPrivate = ref<MyPostItem[]>([]);

async function load() {
  // 公开列表：listPublic 返回的就是 PublicListResp
  const r = await listPublic(1, 20);
  publicItems.value = r.items;

  // 登录后再拉“我的私有已发布”
  if (getToken()) {
    const { items } = await getMyPosts({
      status: "published",
      visibility: "private",
      page: 1,
      page_size: 20
    });
    myPrivate.value = items;
  } else {
    myPrivate.value = [];
  }
}

onMounted(load);
</script>
