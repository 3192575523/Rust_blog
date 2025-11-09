<template>
  <div class="max-w-3xl mx-auto p-6 space-y-4" v-if="ready">
    <h1 class="text-2xl font-bold">{{ isCreate ? "新建文章" : "编辑文章" }}</h1>

    <input v-model="title" class="border p-2 w-full" placeholder="标题" />
    <input v-model="slug" class="border p-2 w-full" placeholder="自定义 slug（可选）" />
    <textarea v-model="excerpt" class="border p-2 w-full" rows="3" placeholder="摘要（可选）" />

    <div class="flex items-center gap-4">
      <label>可见性：</label>
      <select v-model="visibility" class="border p-1">
        <option value="public">public</option>
        <option value="private">private</option>
      </select>
      <label>状态：</label>
      <select v-model="status" class="border p-1">
        <option value="draft">draft</option>
        <option value="published">published</option>
      </select>
    </div>

    <MarkdownEditor v-model="body_md" />

    <!-- ✅ 改成 if/else 组 -->
    <div class="space-x-2">
      <template v-if="isCreate">
        <button @click="create('draft')" class="px-3 py-1 bg-gray-800 text-white rounded">
          保存草稿
        </button>
        <button @click="create('published')" class="px-3 py-1 bg-green-700 text-white rounded">
          发布
        </button>
      </template>
      <template v-else>
        <button @click="saveDraft" class="px-3 py-1 bg-gray-800 text-white rounded">
          保存
        </button>
        <button @click="savePublish" class="px-3 py-1 bg-green-700 text-white rounded">
          保存并发布
        </button>
        <button @click="doDelete" class="px-3 py-1 bg-red-700 text-white rounded">
          删除
        </button>
      </template>
    </div>

    <p v-if="err" class="text-red-600">{{ err }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import MarkdownEditor from "../components/MarkdownEditor.vue";
import { getAdminById, updatePost, publishPost, deletePost, createPost } from "../services/posts";

const route = useRoute();
const router = useRouter();

const mode = route.meta.mode as "create" | "edit" | undefined;
const isCreate = computed(() => mode === "create");

const ready = ref(false);
const err = ref("");

const id = ref<string>("");
const title = ref("");
const slug = ref("");
const excerpt = ref("");
const body_md = ref("");
const visibility = ref<"public" | "private">("public");
const status = ref<"draft" | "published">("draft");

onMounted(async () => {
  try {
    if (isCreate.value) {
      ready.value = true;
      return;
    }
    id.value = route.params.id as string;
    const p = await getAdminById(id.value);
    title.value = p.title;
    slug.value = p.slug;
    excerpt.value = p.excerpt || "";
    body_md.value = (p as any).body_md || "";
    visibility.value = p.visibility as any;
    status.value = p.status as any;
  } catch (e: any) {
    err.value = e.message || "加载失败";
  } finally {
    ready.value = true;
  }
});

async function saveDraft() {
  try {
    await updatePost(id.value, {
      title: title.value,
      slug: slug.value || undefined,
      excerpt: excerpt.value || undefined,
      body_md: body_md.value,
      visibility: visibility.value,
      status: status.value,
    });
    alert("已保存");
  } catch (e: any) {
    err.value = e.message || "保存失败";
  }
}

async function savePublish() {
  try {
    await saveDraft();
    await publishPost(id.value);
    alert("已发布");
  } catch (e: any) {
    err.value = e.message || "发布失败";
  }
}

async function doDelete() {
  if (!confirm("确定删除？")) return;
  try {
    await deletePost(id.value);
    router.push("/");
  } catch (e: any) {
    err.value = e.message || "删除失败";
  }
}

async function create(finalStatus: "draft" | "published") {
  try {
    const r = await createPost({
      title: title.value,
      slug: slug.value || undefined,
      excerpt: excerpt.value || undefined,
      body_md: body_md.value,
      visibility: visibility.value,
      status: finalStatus,
    });
    router.push(`/p/${r.slug}`);
  } catch (e: any) {
    err.value = e.message || "创建失败";
  }
}
</script>

<style scoped>
.prose :where(img) {
  max-width: 100%;
  height: auto;
}
</style>
