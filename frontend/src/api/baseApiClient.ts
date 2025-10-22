import { z } from "zod"
import type { HttpProxy } from "./HttpProxy"

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
    private readonly proxy: HttpProxy,
  ) {
    this.baseUrl = `${API_URL}/${urlSuffix}`
  }

  protected async updateEntity(id: string, entity: z.infer<T>): Promise<void> {
    const validated = this.schema.parse(entity)
    return await this.proxy.put(`${this.baseUrl}/${id}`, validated)
  }

  protected async addEntity(newEntity: Omit<z.infer<T>, "id">): Promise<string> {
    const validated = this.newSchema.parse(newEntity)
    const { id } = await this.proxy.post(this.baseUrl, validated, z.object({ id: z.string() }))
    return id
  }

  protected async deleteEntity(id: string): Promise<void> {
    return await this.proxy.delete(`${this.baseUrl}/${id}`)
  }

  protected async fetchEntities(queryParams: URLSearchParams): Promise<z.infer<T>[]> {
    return await this.proxy.get(this.baseUrl, queryParams, this.schema.array())
  }

  protected async fetchEntity(id: string): Promise<z.infer<T> | undefined> {
    return await this.proxy.get(`${this.baseUrl}/${id}`, undefined, this.schema.optional())
  }
}
