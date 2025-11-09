<!-- src/views/AuthorHome.vue -->
<template>
  <div class="max-w-5xl mx-auto p-6">
    <h1 class="text-2xl font-bold mb-4">作者主页</h1>

    <!-- 资料卡 -->
    <div class="flex gap-6 items-start mb-8">
      <div class="w-28 h-28 rounded-full overflow-hidden bg-gray-200 shrink-0">
        <img
          v-if="form.avatar_url"
          :src="form.avatar_url"
          alt="avatar"
          class="w-full h-full object-cover"
        />
        <div v-else class="w-full h-full flex items-center justify-center text-gray-400">
          No Avatar
        </div>
      </div>

      <div class="grow">
        <div class="mb-3">
          <label class="block text-sm text-gray-600 mb-1">头像</label>
          <input type="file" accept="image/*" @change="onPickAvatar" />
        </div>

        <div class="mb-3">
          <label class="block text-sm text-gray-600 mb-1">昵称</label>
          <input v-model="form.display_name" class="w-full border rounded px-3 py-2" placeholder="请输入昵称" />
        </div>

        <div class="mb-3">
          <label class="block text-sm text-gray-600 mb-1">座右铭</label>
          <input v-model="form.motto" class="w-full border rounded px-3 py-2" placeholder="Keep going." />
        </div>

        <button @click="saveProfile" class="px-4 py-2 rounded bg-blue-600 text-white">
          保存资料
        </button>
      </div>
    </div>

    <!-- 我的文章 -->
    <div class="mb-3 flex flex-wrap items-center gap-3">
      <div>
        <span class="text-sm text-gray-500 mr-2">状态</span>
        <select v-model="filters.status" class="border rounded px-2 py-1">
          <option value="all">全部</option>
          <option value="published">已发布</option>
          <option value="draft">草稿</option>
        </select>
      </div>
      <div>
        <span class="text-sm text-gray-500 mr-2">可见性</span>
        <select v-model="filters.visibility" class="border rounded px-2 py-1">
          <option value="all">全部</option>
          <option value="public">公开</option>
          <option value="private">私有</option>
        </select>
      </div>
      <button @click="loadPosts" class="px-3 py-1 rounded bg-gray-200">刷新</button>
      <router-link to="/new" class="ml-auto px-3 py-1 rounded bg-green-600 text-white">新建文章</router-link>
    </div>

    <div v-if="items.length === 0" class="text-gray-500">暂无文章</div>
    <ul v-else class="divide-y">
      <li v-for="it in items" :key="it.id" class="py-3 flex justify-between items-center">
        <div>
          <router-link :to="`/p/${it.slug}`" class="font-medium hover:underline">
            {{ it.title }}
          </router-link>
          <div class="text-xs text-gray-500 mt-1">
            <span class="mr-2">状态：{{ it.status }}</span>
            <span class="mr-2">可见性：{{ it.visibility }}</span>
            <span v-if="it.published_at">发布时间：{{ it.published_at }}</span>
          </div>
        </div>
        <div class="space-x-2">
          <router-link :to="`/edit/${it.id}`" class="px-2 py-1 rounded bg-blue-600 text-white">编辑</router-link>
        </div>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted, watch } from "vue";
import { getMe, updateMe, uploadAvatar, getMyPosts, type UserProfile, type MyPostItem } from "../services/me";

const profile = ref<UserProfile | null>(null);
const form = reactive<Partial<UserProfile>>({
  display_name: "",
  avatar_url: "",
  motto: "",
});

const filters = reactive<{ status: "all" | "published" | "draft"; visibility: "all" | "public" | "private" }>({
  status: "all",
  visibility: "all",
});

const items = ref<MyPostItem[]>([]);

async function init() {
  const me = await getMe();
  profile.value = me;
  form.display_name = me.display_name ?? "";
  form.avatar_url = me.avatar_url ?? "";
  form.motto = me.motto ?? "";
  await loadPosts();
}

async function loadPosts() {
  const { items: list } = await getMyPosts(filters);
  items.value = list;
}

async function onPickAvatar(e: Event) {
  const input = e.target as HTMLInputElement;
  const f = input.files?.[0];
  if (!f) return;
  const url = await uploadAvatar(f);
  if (url) form.avatar_url = url;
}

async function saveProfile() {
  await updateMe({
    display_name: form.display_name?.trim(),
    avatar_url: form.avatar_url,
    motto: form.motto?.trim(),
  });
  await init();
  alert("已保存");
}

onMounted(init);
watch(filters, loadPosts);
</script>
