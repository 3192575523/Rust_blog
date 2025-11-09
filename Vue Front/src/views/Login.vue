<template>
  <div class="max-w-md mx-auto p-6 space-y-4">
    <h1 class="text-2xl font-bold">登录</h1>
    <form @submit.prevent="doLogin" class="space-y-3">
      <input v-model="username" class="border p-2 w-full" placeholder="用户名" />
      <input v-model="password" class="border p-2 w-full" type="password" placeholder="密码" />
      <button class="px-4 py-2 bg-black text-white rounded" :disabled="loading">
        {{ loading ? "登录中..." : "登录" }}
      </button>
      <p v-if="err" class="text-red-600">{{ err }}</p>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { login } from "../services/posts";
import { setToken } from "../stores/auth";

const router = useRouter();
const route = useRoute();

const username = ref("");
const password = ref("");
const loading = ref(false);
const err = ref("");

async function doLogin() {
  err.value = "";
  loading.value = true;
  try {
    const { access_token } = await login(username.value, password.value);
    setToken(access_token);
    router.replace((route.query.redirect as string) || "/");
  } catch (e: any) {
    err.value = e.message || "登录失败";
  } finally {
    loading.value = false;
  }
}
</script>
