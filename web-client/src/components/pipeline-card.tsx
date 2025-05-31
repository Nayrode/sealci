import type { Pipeline } from '@/types'
import { Clock, GitBranch } from 'lucide-react'
import { Link } from 'react-router-dom'
import { StatusBadge } from './status-badge'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from './ui/card'

interface PipelineCardProps {
  pipeline: Pipeline
}

export function PipelineCard({ pipeline }: PipelineCardProps) {
  // Déterminer le statut global de la pipeline
  const getOverallStatus = () => {
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

  // Extraire le nom du repo à partir de l'URL
  const getRepoName = (url: string) => {
    try {
      const urlObj = new URL(url)
      const pathParts = urlObj.pathname.split('/').filter(Boolean)
      return pathParts.length >= 2 ? `${pathParts[0]}/${pathParts[1]}` : url
    } catch {
      return url
    }
  }

  const repoName = getRepoName(pipeline.repository_url)
  const status = getOverallStatus()

  return (
    <Link to={`/${pipeline.id}`}>
      <Card className="h-full transition-all hover:shadow-md">
        <CardHeader className="pb-2">
          <div className="flex justify-between items-start gap-4">
            <CardTitle className="text-lg font-medium font-pixel">{pipeline.name}</CardTitle>
            <StatusBadge status={status} />
          </div>
        </CardHeader>
        <CardContent>
          <div className="flex items-center text-sm text-muted-foreground mb-2">
            <GitBranch className="h-4 w-4 mr-1" />
            <span className="truncate">{repoName}</span>
          </div>
          <div className="text-sm text-muted-foreground">
            <Clock className="h-4 w-4 inline mr-1" />
            <span>{pipeline.actions.length} actions</span>
          </div>
        </CardContent>
        <CardFooter className="pt-0">
          <div className="flex gap-1 flex-wrap">
            {pipeline.actions.slice(0, 3).map((action) => (
              <StatusBadge key={action.id} status={action.status} className="!py-0 !px-1.5 text-[10px]" />
            ))}
            {pipeline.actions.length > 3 && <span className="text-xs text-muted-foreground">+{pipeline.actions.length - 3} autres</span>}
          </div>
        </CardFooter>
      </Card>
    </Link>
  )
}
