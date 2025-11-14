import { z } from "zod"

export interface HttpProxy {
  get<T extends z.ZodTypeAny>(route: string, queryParams: URLSearchParams | undefined, schema: T): Promise<z.infer<T>>
  put<T>(route: string, object: T): Promise<void>
  post<TIn, TOut extends z.ZodTypeAny>(route: string, object: TIn, schema: TOut): Promise<z.infer<TOut>>
  delete(route: string): Promise<void>
}

