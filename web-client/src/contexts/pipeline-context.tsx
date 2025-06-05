import { createContext, useContext, ReactNode, useState, useEffect, useCallback } from 'react'
import type { CreatePipeline, Pipeline } from '@/types'
import { useGetPipelines } from '@/hooks/use-pipelines';
import { useParams } from 'react-router-dom';

type PipelineContextType = {
  pipelines: Pipeline[] | undefined;
  currentPipeline: Pipeline | undefined;
  isLoading: {
    get: boolean;
    add: boolean;
    update: boolean;
    remove: boolean;
  };
  addPipeline: (pipeline: CreatePipeline) => void;
  getPipeline: (id: number) => Pipeline | undefined;
  updatePipeline: (pipeline: CreatePipeline) => void;
  removePipeline: (id: number) => void;
  reloadPipelines: () => void;
}

const PipelineContext = createContext<PipelineContextType | undefined>(undefined)

export function PipelineProvider({ children }: { children: ReactNode }) {
  const { id } = useParams<{ id: string }>()
  const { data: fetchedPipelines, isFetching, refetch } = useGetPipelines(true)

  const [pipelines, setPipelines] = useState<Pipeline[] | undefined>(fetchedPipelines)
  const [currentPipeline, setCurrentPipeline] = useState<Pipeline | undefined>(undefined)

  const getPipeline = useCallback((id: number): Pipeline | undefined => {
    return pipelines?.find(p => p.id === id)
  }, [pipelines])

  useEffect(() => {
    setPipelines(fetchedPipelines)
  }, [fetchedPipelines])

  useEffect(() => {
    if (id !== undefined) {
      setCurrentPipeline(getPipeline(+id))
    } else {
      setCurrentPipeline(undefined)
    }
  }, [pipelines, id, getPipeline])

  const values: PipelineContextType = {
    pipelines: pipelines,
    currentPipeline: currentPipeline,
    isLoading: {
      get: isFetching,
      add: false,
      update: false,
      remove: false,
    },
    addPipeline: (pipeline: CreatePipeline) => {},
    getPipeline: getPipeline,
    updatePipeline: (pipeline: CreatePipeline) => {},
    removePipeline: (id: number) => {},
    reloadPipelines: () => refetch(),
  }

  return <PipelineContext.Provider value={values}>{children}</PipelineContext.Provider>
}

export function usePipelineContext() {
  const context = useContext(PipelineContext)
  if (context === undefined) {
    throw new Error('usePipelineContext must be used within a PipelineProvider')
  }
  return context
}