import { useEffect, useState } from "react"
import { ActionItem } from "@/components/action-item"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { RefreshCw, ArrowLeft } from "lucide-react"
import { StatusBadge } from "@/components/status-badge"
import { Link } from "react-router-dom"
import { usePipelineContext } from "@/contexts/pipeline-context"
import { motion, AnimatePresence } from "framer-motion"
import LoadingWrapper from "@/components/loading-wrapper"

export default function PipelinePage() {
  const { currentPipeline, isLoading, reloadPipelines } = usePipelineContext()
  const [showSkeleton, setShowSkeleton] = useState(true)

  useEffect(() => {
    const timeout = setTimeout(() => {
      setShowSkeleton(false)
    }, 500)
    return () => clearTimeout(timeout)
  }, [])

  const isLoadingValue = isLoading.get ?? false
  const isStillLoading = isLoadingValue || showSkeleton

  const getOverallStatus = () => {
    if (!currentPipeline || !currentPipeline.actions.length) return "ACTION_STATUS_PENDING"

    if (currentPipeline.actions.some((a) => a.status === "ACTION_STATUS_ERROR")) {
      return "ACTION_STATUS_ERROR"
    }
    if (currentPipeline.actions.some((a) => a.status === "ACTION_STATUS_RUNNING")) {
      return "ACTION_STATUS_RUNNING"
    }
    if (
        currentPipeline.actions.some(
            (a) => a.status === "ACTION_STATUS_PENDING" || a.status === "ACTION_STATUS_SCHEDULED"
        )
    ) {
      return "ACTION_STATUS_PENDING"
    }
    if (currentPipeline.actions.every((a) => a.status === "ACTION_STATUS_COMPLETED")) {
      return "ACTION_STATUS_COMPLETED"
    }

    return "ACTION_STATUS_PENDING"
  }

  const skeletonContent = (
      <div className="space-y-4">
        {[...Array(4)].map((_, i) => (
            <div
                key={i}
                className="flex flex-row justify-between w-full h-[50px] border border-gray-200 rounded-md bg-muted p-4"
            >
              <div className="flex flex-row gap-2 w-full">
                <Skeleton className="h-5 w-[110px]" />
                <Skeleton className="h-5 w-1/4" />
              </div>
              <Skeleton className="h-5 w-[200px]" />
            </div>
        ))}
      </div>
  )

  const emptyContent = (
      <div className="text-center py-12">
        <h2 className="text-xl font-medium mb-2">Pipeline non trouvée</h2>
        <p className="text-muted-foreground">
          La pipeline demandée n'existe pas ou les données ne sont pas disponibles.
        </p>
        <Button asChild className="mt-4">
          <Link to="/">Retour à l'accueil</Link>
        </Button>
      </div>
  )

  return (
      <main className="flex-1 container py-6">
        <div className="flex items-center gap-2 mb-6">
          <Link to="/pipelines">
            <Button variant="ghost" size="icon" className="h-8 w-8">
              <ArrowLeft className="h-4 w-4" />
            </Button>
          </Link>
          <h1 className="text-2xl font-bold">{currentPipeline?.name || "Chargement..."}</h1>
          {currentPipeline && <StatusBadge status={getOverallStatus()} />}
          <div className="ml-auto">
            <Button
                variant="outline"
                size="sm"
                onClick={() => reloadPipelines()}
                disabled={isLoadingValue}
            >
              <RefreshCw className={`h-4 w-4 mr-2 ${isStillLoading ? "animate-spin" : ""}`} />
              Actualiser
            </Button>
          </div>
        </div>

        <LoadingWrapper
            isLoading={isStillLoading}
            skeleton={skeletonContent}
            empty={emptyContent}
            hasData={!!currentPipeline}
        >
          <AnimatePresence>
            <div className="space-y-2">
              {currentPipeline?.actions.map((action, index) => (
                  <motion.div
                      key={action.id}
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: 10 }}
                      transition={{ duration: 0.4, delay: index * 0.1 }}
                  >
                    <ActionItem action={action} />
                  </motion.div>
              ))}
            </div>
          </AnimatePresence>
        </LoadingWrapper>
      </main>
  )
}
