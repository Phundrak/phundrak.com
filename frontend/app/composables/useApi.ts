import type { FetchOptions } from 'ofetch';

export const useApi = () => {
  const config = useRuntimeConfig();
  const apiFetch = $fetch.create({
    baseURL: config.public.apiBase,
  });

  const get = <T>(url: string, options?: FetchOptions) => apiFetch<T>(url, { method: 'GET', ...options });

  const post = <ResultT, PayloadT = Record<string, string | number | boolean>>(
    url: string,
    body?: PayloadT,
    options?: FetchOptions,
  ) => apiFetch<ResultT>(url, { method: 'POST', body, ...options });

  const put = <ResultT, PayloadT = Record<string, string | number | boolean>>(
    url: string,
    body?: PayloadT,
    options?: FetchOptions,
  ) => apiFetch<ResultT>(url, { method: 'PUT', body, ...options });

  const patch = <ResultT, PayloadT = Record<string, string | number | boolean>>(
    url: string,
    body?: PayloadT,
    options?: FetchOptions,
  ) => apiFetch<ResultT>(url, { method: 'PATCH', body, ...options });

  const del = <T>(url: string, options?: FetchOptions) => apiFetch<T>(url, { method: 'DELETE', ...options });

  return { get, post, put, patch, del };
};
