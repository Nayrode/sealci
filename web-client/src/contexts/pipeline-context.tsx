import { createContext, useContext, ReactNode, useState, useEffect } from 'react'
import type { CreatePipeline, Pipeline } from '@/types'
import { useGetPipelines } from '@/hooks/use-pipelines';
import { useParams } from 'react-router-dom';
import { fetchPipeline } from '@/lib/api';

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
  getPipeline: (repoUrl: number) => Pipeline | undefined;
  updatePipeline: (pipeline: CreatePipeline) => void;
  removePipeline: (repoUrl: string) => void;
  reloadPipelines: () => void;
}

const PipelineContext = createContext<PipelineContextType | undefined>(undefined)

export function PipelineProvider({ children }: { children: ReactNode }) {
  const { id } = useParams<{ id: string }>()
  const { data: fetchedPipelines, isPending, refetch } = useGetPipelines(true)

  const [pipelines, setPipelines] = useState<Pipeline[] | undefined>(fetchedPipelines)
  const [currentPipeline, setCurrentPipeline] = useState<Pipeline | undefined>(undefined)

  const getPipeline = async (id: number): Promise<Pipeline | undefined> => {
    if (pipelines && !isPending) {
      return pipelines?.find(p => p.id === id)
    }

    const pipeline = await fetchPipeline({ id: +id, verbose: true })
    return pipeline
  }

  const handleGetPipelinePage = async (id: string | undefined): Promise<Pipeline | undefined> => {
    if (!id) {
      setCurrentPipeline(undefined)
      return
    }

    const pipeline = await getPipeline(+id)
    if (!pipeline) return

    setCurrentPipeline(pipeline)
  }

  useEffect(() => {
    setPipelines(fetchedPipelines)
  }, [fetchedPipelines])

  useEffect(() => {
    handleGetPipelinePage(id)
  }, [id])

  const values = {
    pipelines: pipelines,
    currentPipeline: currentPipeline,
    isLoading: {
      get: isPending,
      add: false,
      update: false,
      remove: false
    },
    addPipeline: (pipeline: CreatePipeline) => {},
    getPipeline: getPipeline,
    updatePipeline: (pipeline: CreatePipeline) => {},
    removePipeline: (repoUrl: string) => {},
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