// src/stores/auth.ts
const KEY = "token";

export function setToken(t: string) {
  localStorage.setItem(KEY, t);
}
export function getToken(): string | null {
  return localStorage.getItem(KEY);
}
export function clearToken() {
  localStorage.removeItem(KEY);
}

type JwtPayload = {
  sub?: string;
  [k: string]: unknown;
};

// Base64URL -> Base64，再用 atob 解码为字符串
function base64UrlDecode(input: string): string {
  // JWT 使用 - 和 _，需要替换回 + 和 /
  let s = input.replace(/-/g, "+").replace(/_/g, "/");
  // 补齐 padding 到 4 的倍数
  const pad = s.length % 4;
  if (pad) s += "=".repeat(4 - pad);
  return atob(s);
}

// 从 JWT payload 解析当前用户 id（后端把 user_id 放在 sub）
export function getCurrentUserId(): string | null {
  const t = getToken();
  if (!t) return null;

  const parts = t.split(".");
  const payloadPart = parts[1]; // 可能是 undefined，需要判断
  if (!payloadPart) return null;

  try {
    const json = base64UrlDecode(payloadPart);
    const payload = JSON.parse(json) as JwtPayload;
    return payload.sub ?? null;
  } catch {
    return null;
  }
}
