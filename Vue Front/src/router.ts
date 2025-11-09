// src/router.ts
import { createRouter, createWebHistory } from "vue-router";
import Posts from "./views/Posts.vue";
import PostPublic from "./views/PostPublic.vue";
import EditPost from "./views/EditPost.vue";
import Login from "./views/Login.vue";
import AuthorHome from "./views/AuthorHome.vue"; // ✅ 新增
import { getToken } from "./stores/auth";

const router = createRouter({
  // 如需支持子路径部署，可用 createWebHistory(import.meta.env.BASE_URL)
  history: createWebHistory(),
  routes: [
    { path: "/", name: "home", component: Posts },
    { path: "/p/:slug", name: "post", component: PostPublic },
    { path: "/login", name: "login", component: Login },

    // 写作相关（需要登录）
    { path: "/new", name: "new", component: EditPost, meta: { auth: true, mode: "create" } },
    { path: "/edit/:id", name: "edit", component: EditPost, meta: { auth: true, mode: "edit" } },

    // ✅ 作者主页（需要登录）
    { path: "/me", name: "me", component: AuthorHome, meta: { auth: true } },

    // 可选：兜底 404
    // { path: "/:pathMatch(.*)*", name: "notfound", component: NotFound },
  ],
});

router.beforeEach((to) => {
  if (to.meta.auth && !getToken()) {
    return { name: "login", query: { redirect: to.fullPath } };
  }
  return true; // 显式允许
});

export default router;
