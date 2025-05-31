import { createContext, useContext, ReactNode, useState } from 'react'
import type { Pipeline } from '@/types'

type PipelineContextType = {
  currentPipeline: Pipeline | undefined
  setCurrentPipeline: (pipeline: Pipeline | undefined) => void
}

const PipelineContext = createContext<PipelineContextType | undefined>(undefined)

export function PipelineProvider({ children }: { children: ReactNode }) {
  const [currentPipeline, setCurrentPipeline] = useState<Pipeline | undefined>(undefined)

  return <PipelineContext.Provider value={{ currentPipeline, setCurrentPipeline }}>{children}</PipelineContext.Provider>
}

export function usePipelineContext() {
  const context = useContext(PipelineContext)
  if (context === undefined) {
    throw new Error('usePipelineContext must be used within a PipelineProvider')
  }
  return context
}
