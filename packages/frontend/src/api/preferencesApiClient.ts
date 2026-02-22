import { z } from "zod"
import type { HttpProxy } from "./HttpProxy"
import { useHttpApi } from "@/plugins/HttpPlugin"

const userPreferencesSchema = z.object({
  libraryYearFilter: z.string().nullable().optional(),
  libraryTypeFilter: z.array(z.string()).nullable().optional(),
})

export type UserPreferences = z.infer<typeof userPreferencesSchema>

export type YearFilterOption = "all" | number

export type EntryTypeFilter = "Book" | "Game"

class PreferencesApiClient {
  private proxy: HttpProxy

  constructor(proxy: HttpProxy) {
    this.proxy = proxy
  }

  async getPreferences(): Promise<UserPreferences> {
    return this.proxy.get("preferences", undefined, userPreferencesSchema)
  }

  async updatePreferences(preferences: UserPreferences): Promise<void> {
    return this.proxy.put("preferences", preferences)
  }
}

export function usePreferencesApi(): PreferencesApiClient {
  const proxy = useHttpApi()
  return new PreferencesApiClient(proxy)
}
