import { getAccessToken } from "@/modules/auth-store"
import { z } from "zod"

const API_URL = import.meta.env.VITE_API_URL

export abstract class BaseApiClient<
  T extends z.ZodType<{ id: string }>,
  TNew extends z.ZodTypeAny,
> {
  private readonly baseUrl: string
  constructor(
    private readonly schema: T,
    private readonly newSchema: TNew,
    urlSuffix: string,
  ) {
    this.baseUrl = `${API_URL}/${urlSuffix}`
  }

  private getHeaders(): HeadersInit {
    return {
      "Content-Type": "application/json",
      Authorization: `Bearer ${getAccessToken()}`,
    }
  }

  protected async updateEntity(id: string, entity: z.infer<T>): Promise<void> {
    const validated = this.schema.parse(entity)
    const resp = await fetch(`${this.baseUrl}/${id}`, {
      method: "PUT",
      headers: this.getHeaders(),
      body: JSON.stringify(validated),
    })

    if (!resp.ok) {
      throw new Error("Failed to update entity")
    }
  }

  protected async addEntity(newEntity: Omit<z.infer<T>, "id">): Promise<string> {
    const validated = this.newSchema.parse(newEntity)
    const resp = await fetch(this.baseUrl, {
      method: "POST",
      headers: this.getHeaders(),
      body: JSON.stringify(validated),
    })

    if (!resp.ok) {
      throw new Error("Failed to add entity")
    }

    const data = z.object({ id: z.string() }).parse(await resp.json())
    return data.id
  }

  protected async deleteEntity(id: string): Promise<void> {
    const resp = await fetch(`${this.baseUrl}/${id}`, {
      method: "DELETE",
      headers: this.getHeaders(),
    })

    if (!resp.ok) {
      throw new Error("Failed to delete entity")
    }
  }

  protected async fetchEntities(queryParams: URLSearchParams): Promise<z.infer<T>[]> {
    const resp = await fetch(this.baseUrl + "?" + queryParams, {
      method: "GET",
      headers: this.getHeaders(),
    })

    if (!resp.ok) {
      throw new Error("Failed to fetch entities")
    }
    const data = await resp.json()
    return this.schema.array().parse(data)
  }

  protected async fetchEntity(id: string): Promise<z.infer<T> | undefined>{
    const resp = await fetch(`${this.baseUrl}/${id}`, {
      method: "GET",
      headers: this.getHeaders()
    })

    if(!resp.ok){
      return undefined
    }

    const data = await resp.json()

    return this.schema.optional().parse(data)
  }
}
