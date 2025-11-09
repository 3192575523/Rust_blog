<template>
  <div class="app">
    <header class="nav">
      <div class="nav-left">
        <router-link class="brand" to="/">My Blog</router-link>
        <router-link class="link" to="/">首页</router-link>

        <!-- ✅ 已登录时展示：作者主页、新建 -->
        <router-link v-if="isAuthed" class="link" to="/me">作者主页</router-link>
        <router-link v-if="isAuthed" class="link" to="/new">新建</router-link>
      </div>

      <div class="nav-right">
        <span v-if="isAuthed && userId" class="uid">UID: {{ userId }}</span>
        <button v-if="isAuthed" class="btn" @click="logout">退出</button>
        <router-link v-else class="btn" to="/login">登录</router-link>
      </div>
    </header>

    <main class="page">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { getToken, clearToken, getCurrentUserId } from './stores/auth'

const router = useRouter()
const route = useRoute()

const isAuthed = ref<boolean>(!!getToken())
const userId = ref<string | null>(getCurrentUserId())

function syncAuth() {
  isAuthed.value = !!getToken()
  userId.value = getCurrentUserId()
}

function logout() {
  clearToken()
  syncAuth()
  // 退出后回到首页（也可跳 /login）
  router.push('/')
}

// 路由变化时，同步一次（localStorage 非响应式）
watch(() => route.fullPath, () => syncAuth())

// 监听跨标签页 token 变化
function onStorage(e: StorageEvent) {
  if (e.key === 'token') syncAuth()
}
onMounted(() => window.addEventListener('storage', onStorage))
onBeforeUnmount(() => window.removeEventListener('storage', onStorage))
</script>

<style scoped>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}
.nav {
  position: sticky;
  top: 0;
  z-index: 10;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 16px;
  border-bottom: 1px solid #e5e7eb;
  background: #fff;
}
.nav-left, .nav-right {
  display: flex;
  align-items: center;
  gap: 10px;
}
.brand {
  font-weight: 700;
  margin-right: 12px;
  text-decoration: none;
  color: #111827;
}
.link {
  text-decoration: none;
  color: #374151;
  padding: 4px 6px;
  border-radius: 6px;
}
.link.router-link-active {
  background: #f3f4f6;
}
.btn {
  appearance: none;
  background: #111827;
  color: #fff;
  border: none;
  padding: 6px 10px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}
.btn:hover { background: #0b1220; }
.uid {
  color: #6b7280;
  font-size: 12px;
}
.page {
  max-width: 960px;
  width: 100%;
  margin: 0 auto;
  padding: 16px;
  flex: 1;
}
</style>
