import { solutionSchema, type Solution } from "@/models/challenge"
import { useHttpApi } from "@/plugins/HttpPlugin"
import type { HttpProxy } from "./HttpProxy"
import { API_URL, buildSearchParams } from "./baseApiClient"
import z from "zod"

class SolutionsApiClient {
  constructor(private proxy: HttpProxy) {}

  async searchSolutions(query: { challengeId?: string }): Promise<Solution[]> {
    const params = buildSearchParams(query)

    const res = await this.proxy.get(
      `${API_URL}/solution`,
      params,
      z.object({ solutions: solutionSchema.array() }),
    )

    return res.solutions
  }

  async upsertSolutions(solutions: Solution[], challengeId: string): Promise<Solution[]> {
    const result = await this.proxy.post(
      `${API_URL}/solution/${challengeId}`,
      { solutions },
      z.object({ solutions: solutionSchema.array() }),
    )

    return result.solutions
  }
}

export function useSolutionsApi() {
  const httpProxy = useHttpApi()
  return new SolutionsApiClient(httpProxy)
}
