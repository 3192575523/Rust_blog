// src/services/me.ts
import { http } from "../api";

export type UserProfile = {
  id: string;
  username: string;
  display_name?: string | null;
  avatar_url?: string | null;
  motto?: string | null;
  created_at?: string;
};

export type MyPostItem = {
  id: string;
  slug: string;
  title: string;
  status: "draft" | "published";
  visibility: "public" | "private";
  published_at?: string | null;
  excerpt?: string | null;
};

export async function getMe(): Promise<UserProfile> {
  const { data } = await http.get<UserProfile>("/api/me");
  return data;
}

export async function updateMe(
  patch: Partial<UserProfile>
): Promise<{ ok: true }> {
  const { data } = await http.put<{ ok: true }>("/api/me", patch);
  return data;
}

export async function getMyPosts(params?: {
  status?: "all" | "published" | "draft";
  visibility?: "all" | "public" | "private";
  page?: number;
  page_size?: number;
}): Promise<{ items: MyPostItem[] }> {
  const q = new URLSearchParams();
  if (params?.status) q.set("status", params.status);
  if (params?.visibility) q.set("visibility", params.visibility);
  if (params?.page) q.set("page", String(params.page));
  if (params?.page_size) q.set("page_size", String(params.page_size));
  const { data } = await http.get<{ items: MyPostItem[] }>(
    `/api/me/posts?${q.toString()}`
  );
  return data;
}

export async function uploadAvatar(file: File): Promise<string> {
  const fd = new FormData();
  fd.append("file", file, file.name);
  const { data } = await http.post<{ files: string[] }>(
    "/api/media",
    fd,
    { headers: { "Content-Type": "multipart/form-data" } }
  );
  return data?.files?.[0] ?? "";
}
