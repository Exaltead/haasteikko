import { z } from "zod"
import { BaseApiClient } from "./baseApiClient"
import type { Book, Game, LibraryItem } from "@/models/LibraryItem"
import type { HttpProxy } from "./HttpProxy"
import { useHttpApi } from "@/plugins/HttpPlugin"

/*
const libraryBookSchema = z.object({
  title: z.string(),
  author: z.string(),
  translator: z.string().optional(),
})

const libraryGameSchema = z.object({
  title: z.string(),
  creator: z.string(),
})


const libraryApiItemSchema = z.object({
  kind: z.literal("Book").or(z.literal("Game")),
  id: z.string(),
  book: libraryBookSchema.optional(),
  game: libraryGameSchema.optional(),
  activatedChallengeIds: z.string().array(),
  favorite: z.boolean(),
})*/

const libraryApiItemSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("Book"),
    id: z.string(),
    title: z.string(),
    author: z.string(),
    translator: z.string().optional(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
  }),
  z.object({
    kind: z.literal("Game"),
    id: z.string(),
    title: z.string(),
    author: z.string(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
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
  }),
  z.object({
    kind: z.literal("Game"),
    title: z.string(),
    author: z.string(),
    activatedChallengeIds: z.string().array(),
    favorite: z.boolean(),
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
      }
    case "Game":
      return {
        kind: "Game",
        title: item.title,
        author: item.creator,
        activatedChallengeIds: item.activatedChallengeIds,
        favorite: item.favorite,
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

  async addLibraryItem(item: Omit<LibraryItem, "id">): Promise<string> {
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
