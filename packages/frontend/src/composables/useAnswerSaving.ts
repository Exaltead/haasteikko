import type { Answer } from "@/models/challenge";
import { ref } from "vue";
import type { Ref } from "vue";

type AnswerSaveState = {
  [answerId: string]: 'idle' | 'saving' | 'success' | 'error'
}

export function useAnswerSaving(
  answersMap: Ref<Map<string, Answer>>,
  challengeId: string,
  itemId: string,
  emits: (event: 'submit', answers: Answer[]) => void,
  answerApiClient: { upsertAnswers: (answers: Answer[], challengeId: string, itemId: string) => Promise<Answer[]> }
): {
  answerSaveStates: Ref<AnswerSaveState>,
  errorMessage: Ref<string | null>,
  isAnswerDisabled: (answerId: string) => boolean,
  saveAnswers: () => Promise<void>,
  updateAnswerStates: (ids: string[], newState: AnswerSaveState[keyof AnswerSaveState]) => void
} {
  const answerSaveStates = ref<AnswerSaveState>({})
  const errorMessage = ref<string | null>(null)
  
  function updateAnswerStates(ids: string[], newState: AnswerSaveState[keyof AnswerSaveState]): void {
    answerSaveStates.value = ids.reduce((acc, id) => {
      acc[id] = newState
      return acc
    }, { ...answerSaveStates.value })
  }
  
  function isAnswerDisabled(answerId: string): boolean {
    return answerSaveStates.value[answerId] === 'saving'
  }
  
  async function saveAnswers(): Promise<void> {
    const savingAnswers = Object.entries(answerSaveStates.value)
      .filter(([, state]) => state === 'saving')
      .map(([answerId]) => answerId)
    
    if (savingAnswers.length === 0) return
    
    try {
      const answers: Answer[] = [...answersMap.value.values()]
      await answerApiClient.upsertAnswers(answers, challengeId, itemId)
      
      // Set success state
      updateAnswerStates(savingAnswers, 'success')
      emits('submit', answers)
      
      // Reset success state after 1 second
      setTimeout(() => {
        const currentStates = { ...answerSaveStates.value }
        savingAnswers.forEach(answerId => {
          // Only reset if still in success state (not changed by new edit)
          if (currentStates[answerId] === 'success') {
            currentStates[answerId] = 'idle'
          }
        })
        answerSaveStates.value = currentStates
      }, 1000)
      
    } catch (error) {
      console.error("Failed to save answer:", error)
      errorMessage.value = "Vastauksen tallentaminen epäonnistui. Yritä uudelleen."
      updateAnswerStates(savingAnswers, 'error')
    }
  }
  
  return {
    answerSaveStates,
    errorMessage,
    isAnswerDisabled,
    saveAnswers,
    updateAnswerStates
  }
}