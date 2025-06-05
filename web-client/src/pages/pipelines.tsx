import { Button } from '@/components/ui/button'
import { RefreshCw } from 'lucide-react'
import { PipelineCard } from '@/components/pipeline-card'
import { usePipelineContext } from '@/contexts/pipeline-context'

export default function Home() {
  const { pipelines, isLoading, reloadPipelines } = usePipelineContext()

  return (
    <main className="flex-1 container py-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Pipelines</h1>
        <Button variant="outline" size="sm" onClick={() => reloadPipelines()} disabled={isLoading.get}>
          <RefreshCw className={`h-4 w-4 mr-2 ${isLoading.get ? 'animate-spin' : ''}`} />
          Actualiser
        </Button>
      </div>

      {isLoading.get ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {[...Array(6)].map((_, i) => (
            <div key={i} className="h-[180px] rounded-md bg-muted animate-pulse" />
          ))}
        </div>
      ) : !pipelines || pipelines.length === 0 ? (
        <div className="text-center py-12">
          <h2 className="text-xl font-medium mb-2">Aucune pipeline trouvée</h2>
          <p className="text-muted-foreground">Aucune pipeline n'a été lancée ou les données ne sont pas disponibles.</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {pipelines.map((pipeline) => (
            <PipelineCard key={pipeline.id} pipeline={pipeline} />
          ))}
        </div>
      )}
    </main>
  )
}
