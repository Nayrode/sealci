import { ActionItem } from '@/components/action-item'
import { usePipeline } from '@/hooks/use-pipelines'
import { Button } from '@/components/ui/button'
import { RefreshCw, ArrowLeft } from 'lucide-react'
import { StatusBadge } from '@/components/status-badge'
import { Link } from 'react-router-dom'
import { useParams } from 'react-router-dom'
import { usePipelineContext } from '@/contexts/PipelineContext'
import { useEffect } from 'react'

export default function PipelineDetail() {
  const { id } = useParams<{ id: string }>()
  const { data: pipeline, isPending, refetch } = usePipeline(true, id!)
  const { setCurrentPipeline } = usePipelineContext()

  useEffect(() => {
    if (pipeline) {
      setCurrentPipeline(pipeline)
    }
    return () => {
      // Nettoyer le context quand on quitte la page
      setCurrentPipeline(undefined)
    }
  }, [pipeline, setCurrentPipeline])

  // Déterminer le statut global de la pipeline
  const getOverallStatus = () => {
    if (!pipeline || !pipeline.actions.length) return 'ACTION_STATUS_PENDING'

    if (pipeline.actions.some((a) => a.status === 'ACTION_STATUS_ERROR')) {
      return 'ACTION_STATUS_ERROR'
    }
    if (pipeline.actions.some((a) => a.status === 'ACTION_STATUS_RUNNING')) {
      return 'ACTION_STATUS_RUNNING'
    }
    if (pipeline.actions.some((a) => a.status === 'ACTION_STATUS_PENDING' || a.status === 'ACTION_STATUS_SCHEDULED')) {
      return 'ACTION_STATUS_PENDING'
    }
    if (pipeline.actions.every((a) => a.status === 'ACTION_STATUS_COMPLETED')) {
      return 'ACTION_STATUS_COMPLETED'
    }
    return 'ACTION_STATUS_PENDING'
  }

  return (
    <main className="flex-1 container py-6">
      <div className="flex items-center gap-2 mb-6">
        <Link to="/">
          <Button variant="ghost" size="icon" className="h-8 w-8">
            <ArrowLeft className="h-4 w-4" />
          </Button>
        </Link>
        <h1 className="text-2xl font-bold">{pipeline?.name || 'Chargement...'}</h1>
        {pipeline && <StatusBadge status={getOverallStatus()} />}
        <div className="ml-auto">
          <Button variant="outline" size="sm" onClick={() => refetch()} disabled={isPending}>
            <RefreshCw className={`h-4 w-4 mr-2 ${isPending ? 'animate-spin' : ''}`} />
            Actualiser
          </Button>
        </div>
      </div>

      {isPending ? (
        <div className="space-y-4">
          {[...Array(4)].map((_, i) => (
            <div key={i} className="h-[100px] rounded-md bg-muted animate-pulse" />
          ))}
        </div>
      ) : !pipeline ? (
        <div className="text-center py-12">
          <h2 className="text-xl font-medium mb-2">Pipeline non trouvée</h2>
          <p className="text-muted-foreground">La pipeline demandée n'existe pas ou les données ne sont pas disponibles.</p>
          <Button asChild className="mt-4">
            <Link to="/">Retour à l'accueil</Link>
          </Button>
        </div>
      ) : (
        <div className="space-y-2">
          {pipeline.actions.map((action) => (
            <ActionItem key={action.id} action={action} />
          ))}
        </div>
      )}
    </main>
  )
}
