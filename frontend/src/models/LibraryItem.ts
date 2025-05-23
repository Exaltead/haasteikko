type BaseLibraryItem = {
  id: string
  activatedChallengeIds: string[]
  favorite: boolean
}

export type Book = BaseLibraryItem & {
  kind: "Book"
  title: string
  author: string
  translator?: string
}

export type Game = BaseLibraryItem & {
  kind: "Game"
  title: string
  creator: string
}

export type LibraryItem = Book | Game
