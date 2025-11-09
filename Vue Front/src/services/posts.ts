// src/services/posts.ts
import api from "../api";

export type Visibility = "public" | "private";
export type Status = "draft" | "published";

export interface PostInput {
  title: string;
  slug?: string;
  excerpt?: string;
  body_md: string;
  tags?: string[];
  status?: Status;
  visibility?: Visibility;
}

export interface Post {
  id: string;
  slug: string;
  title: string;
  excerpt?: string | null;
  body_html: string;
  published_at?: string | null;
  author_id: string;
  visibility: Visibility;
  status: Status;
}

export interface AdminPost extends Post {
  body_md: string;
  tags: string[];
}

export interface PublicListItem {
  id: string;
  slug: string;
  title: string;
  excerpt?: string | null;
  published_at?: string | null;
}

export interface PublicListResp {
  page: number;
  page_size: number;
  items: PublicListItem[];
}

export async function login(username: string, password: string): Promise<{ access_token: string }> {
  const res = await api.post<{ access_token: string }>("/api/auth/login", { username, password });
  return res.data;
}

export async function listPublic(
  page = 1,
  page_size = 10,
  tag?: string,
  q?: string
): Promise<PublicListResp> {
  const res = await api.get<PublicListResp>("/api/posts", { params: { page, page_size, tag, q } });
  return res.data;
}

export async function getPublicBySlug(slug: string): Promise<Post> {
  const res = await api.get<Post>(`/api/posts/slug/${encodeURIComponent(slug)}`);
  return res.data;
}

export async function getAdminById(id: string): Promise<AdminPost> {
  const res = await api.get<AdminPost>(`/api/posts/${id}`);
  return res.data;
}

export async function createPost(inp: PostInput): Promise<{ id: string; slug: string }> {
  const res = await api.post<{ id: string; slug: string }>("/api/posts", inp);
  return res.data;
}

export async function updatePost(id: string, inp: PostInput): Promise<{ ok: true }> {
  const res = await api.put<{ ok: true }>(`/api/posts/${id}`, inp);
  return res.data;
}

export async function publishPost(id: string): Promise<{ ok: true }> {
  const res = await api.post<{ ok: true }>(`/api/posts/${id}/publish`);
  return res.data;
}

export async function deletePost(id: string): Promise<{ ok: true }> {
  const res = await api.delete<{ ok: true }>(`/api/posts/${id}`);
  return res.data;
}
