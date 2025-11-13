import { z } from "zod"

export const questionTypeSchema = z.enum(["Boolean", "TextInput"])
export type QuestionType = z.infer<typeof questionTypeSchema>

export const questionSchema = z.object({
  kind: questionTypeSchema,
  id: z.string(),
  question: z.string(),
  number: z.number(),
  questionClusterSize: z.number(),
})

export type Question = z.infer<typeof questionSchema>

export const answerSchema = z.object({
  kind: questionTypeSchema,
  id: z.string(),
  questionId: z.string(),
  answered: z.boolean(),
  answer: z.enum(["yes", "no"]).or(z.string()),
  itemId: z.string(),
})

export type Answer = z.infer<typeof answerSchema>

export const challengeSchema = z.object({
  id: z.string(),
  name: z.string(),
  status: z.enum(["active", "inactive"]),
  targetMedia: z.enum(["Book", "Game"]),
  questions: questionSchema.array(),
  kind: z.string(),
})
export type Challenge = z.infer<typeof challengeSchema>

export const solutionTypeSchema = z.enum(["SinglePartSolution", "MultiPartSolution"])

export const solutionSchema = z.object({
  kind: solutionTypeSchema,
  questionId: z.string(),
  singleAnswerItemId: z
    .string()
    .nullish()
    .transform((t) => t ?? undefined),
  multipleAnswerItemIds: z
    .string()
    .array()
    .nullish()
    .transform((t) => t ?? [])
    .default([]),
})

export type Solution = z.infer<typeof solutionSchema>
