import { createContext, useContext, ReactNode, useState, useEffect } from 'react'
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
  getPipeline: (repoUrl: number) => Pipeline | undefined;
  updatePipeline: (pipeline: CreatePipeline) => void;
  removePipeline: (repoUrl: string) => void;
  reloadPipelines: () => void;
}

const PipelineContext = createContext<PipelineContextType | undefined>(undefined)

export function PipelineProvider({ children }: { children: ReactNode }) {
  const { id } = useParams<{ id: string }>()
  const { data: fetchedPipelines, isPending, refetch } = useGetPipelines(false)

  const [pipelines, setPipelines] = useState<Pipeline[] | undefined>(fetchedPipelines)
  const [currentPipeline, setCurrentPipeline] = useState<Pipeline | undefined>(undefined)

  const getPipeline = (id: number): Pipeline | undefined => {
    return pipelines?.find(p => p.id === id)
  }

  useEffect(() => {
    setPipelines(fetchedPipelines)
  }, [fetchedPipelines])

  useEffect(() => {
    if (!id) {
      setCurrentPipeline(undefined)
      return
    }

    const pipeline = getPipeline(+id)
    if (!pipeline) return

    setCurrentPipeline(pipeline)
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

//   const [pipeline, setPipeline] = useState<Pipeline | undefined>(undefined)

//   useEffect(() => {
//     if (!id) return

//     // Récupérer la pipeline depuis le contexte
//     const fetchedPipeline = getPipeline(+id)
//     if (fetchedPipeline) {
//       setPipeline(fetchedPipeline)
//     } else {
//       // Si la pipeline n'est pas trouvée, recharger les pipelines
//       reloadPipelines()
//     }

//     // Optionnel : mettre à jour le titre de la page
//     document.title = fetchedPipeline ? `Pipeline - ${fetchedPipeline.name}` : 'Chargement...'
//   }, [id, getPipeline, reloadPipelines])