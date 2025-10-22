import type { HttpProxy } from "@/api/HttpProxy"
import { useAuth0 } from "@auth0/auth0-vue"
import { inject, type Plugin } from "vue"
import type { ZodTypeAny, TypeOf, z } from "zod"

export type ApiConfig = {
  apiUrl: string
}

const API_CONFIG_INJECTION_KEY = "apiConfig"

export function createApi(config: ApiConfig): Plugin {
  return {
    install(app) {
      app.provide(API_CONFIG_INJECTION_KEY, config)
    },
  }
}

export function useHttpApi(): HttpProxy {
  const { getAccessTokenSilently } = useAuth0()
  const apiConfig: ApiConfig | undefined = inject(API_CONFIG_INJECTION_KEY)
  if (!apiConfig) {
    throw new Error("ApiConfig not provided")
  }

  async function getHeaders(): Promise<HeadersInit> {
    const accessToken = await getAccessTokenSilently()
    return {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    }
  }

  const proxy: HttpProxy = {
    get: async function <T extends ZodTypeAny>(
      route: string,
      queryParams: URLSearchParams | undefined,
      schema: T,
    ): Promise<z.infer<T>> {
      const url = new URL(route, apiConfig.apiUrl)
      if (queryParams) {
        url.search = queryParams.toString()
      }
      const resp = await fetch(url, {
        method: "GET",
        headers: await getHeaders(),
      })
      if (!resp.ok) {
        throw new Error("Failed to get: " + url)
      }
      const data = await resp.json()
      return schema.parse(data)
    },
    put: async function <T>(route: string, object: T): Promise<void> {
      const url = new URL(route, apiConfig.apiUrl)
      const resp = await fetch(url.toString(), {
        method: "PUT",
        headers: await getHeaders(),
        body: JSON.stringify(object),
      })
      if (!resp.ok) {
        throw new Error("Failed to update entity: " + url)
      }
    },
    post: async function <TIn, TOut extends ZodTypeAny>(
      route: string,
      object: TIn,
      schema: TOut,
    ): Promise<TypeOf<TOut>> {
      const url = new URL(route, apiConfig.apiUrl)
      const resp = await fetch(url.toString(), {
        method: "POST",
        headers: await getHeaders(),
        body: JSON.stringify(object),
      })
      if (!resp.ok) {
        throw new Error("Failed to add entity: " + url)
      }
      const data = await resp.json()
      return schema.parse(data)
    },
    delete: async function (route: string): Promise<void> {
      const url = new URL(route, apiConfig.apiUrl)
      const resp = await fetch(url.toString(), {
        method: "DELETE",
        headers: await getHeaders(),
      })
      if (!resp.ok) {
        throw new Error("Failed to delete entity: " + url)
      }
    },
  }
  return proxy
}
