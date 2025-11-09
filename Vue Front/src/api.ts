// src/api.ts
import axios, { type AxiosError } from "axios";
import { getToken } from "./stores/auth";

export const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE ?? "",
  timeout: 15000,
  withCredentials: false,
});

// 请求拦截：自动附带 Bearer token
api.interceptors.request.use((config) => {
  const t = getToken();
  if (t) {
    config.headers = config.headers ?? {};
    (config.headers as any).Authorization = `Bearer ${t}`;
  }
  return config;
});

// ⚠️ 保持返回 AxiosResponse（不要在这里 return res.data）
api.interceptors.response.use(
  (res) => res,
  (err: AxiosError<any>) => {
    const msg = (err.response?.data as any)?.error || err.message || "request error";
    return Promise.reject(new Error(msg));
  }
);

export default api;
export const http = api;
