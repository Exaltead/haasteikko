import { z } from "zod"
import { BaseApiClient } from "./baseApiClient"
import type { Book, Game, LibraryItem } from "@/models/LibraryItem"
import type { HttpProxy } from "./HttpProxy"
import { useHttpApi } from "@/plugins/HttpPlugin"

const libraryApiItemSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("Book"),
    id: z.string(),
    title: z.string(),
    author: z.string(),
    translator: z.string().optional(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
    completedAt: z.string(),
    addedAt: z.string().datetime({offset: true}),
  }),
  z.object({
    kind: z.literal("Game"),
    id: z.string(),
    title: z.string(),
    author: z.string(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
    completedAt: z.string(),
    addedAt: z.string().datetime({offset: true}),
  }),
])

const newlibraryApiItemSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("Book"),
    title: z.string(),
    author: z.string(),
    translator: z.string().optional(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
    completedAt: z.string(),
  }),
  z.object({
    kind: z.literal("Game"),
    title: z.string(),
    author: z.string(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
    completedAt: z.string(),
  }),
])

type ApiLibraryItem = z.infer<typeof libraryApiItemSchema>

type NewApiLibraryItem = z.infer<typeof newlibraryApiItemSchema>

function mapFromApi(item: ApiLibraryItem): LibraryItem {
  switch (item.kind) {
    case "Book":
      const book: Book = {
        kind: "Book",
        favorite: item.favorite,
        id: item.id,
        activatedChallengeIds: item.activatedChallengeIds,
        translator: item.translator,
        title: item.title,
        author: item.author,
        completedAt: item.completedAt,
        addedAt: item.addedAt,
      }

      return book
    case "Game":
      const game: Game = {
        kind: "Game",
        favorite: item.favorite,
        id: item.id,
        activatedChallengeIds: item.activatedChallengeIds,
        title: item.title,
        creator: item.author,
        completedAt: item.completedAt,
        addedAt: item.addedAt,
      }

      return game
  }
}

function mapToApi(item: LibraryItem): NewApiLibraryItem {
  switch (item.kind) {
    case "Book":
      return {
        kind: "Book",
        title: item.title,
        author: item.author,
        translator: item.translator,
        activatedChallengeIds: item.activatedChallengeIds,
        favorite: item.favorite,
        completedAt: item.completedAt,
      }
    case "Game":
      return {
        kind: "Game",
        title: item.title,
        author: item.creator,
        activatedChallengeIds: item.activatedChallengeIds,
        favorite: item.favorite,
        completedAt: item.completedAt,
      }
  }
}

class LibraryApiClient extends BaseApiClient<
  typeof libraryApiItemSchema,
  typeof newlibraryApiItemSchema
> {
  constructor(proxy: HttpProxy) {
    super(libraryApiItemSchema, newlibraryApiItemSchema, "library", proxy)
  }

  async fetchLibraryItems(): Promise<LibraryItem[]> {
    const items = await this.fetchEntities(new URLSearchParams())
    return items.map(mapFromApi)
  }

  async addLibraryItem(item: Omit<LibraryItem, "id" | "addedAt">): Promise<string> {
    // Discriminated unions and omits do not work together
    const apiItem = mapToApi({ ...item } as unknown as LibraryItem)
    return this.addEntity(apiItem)
  }

  async updateLibraryItem(item: LibraryItem): Promise<void> {
    const apiItem = mapToApi(item)
    return await this.updateEntity(item.id, { ...apiItem, id: item.id })
  }

  async deleteItem(id: string): Promise<void> {
    return await this.deleteEntity(id)
  }

  async getLibraryItem(id: string): Promise<LibraryItem | undefined> {
    const item = await this.fetchEntity(id)
    if (item === undefined) {
      return undefined
    }

    return mapFromApi(item)
  }
}

export function useLibraryApi(): LibraryApiClient {
  const proxy = useHttpApi()
  return new LibraryApiClient(proxy)
}
