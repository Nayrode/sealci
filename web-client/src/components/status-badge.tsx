import { cn } from '@/lib/utils'
import type { PipelineStatus } from '@/types'
import { CheckCircle, Clock, AlertCircle, Play, Loader2 } from 'lucide-react'

interface StatusBadgeProps {
  status: string
  className?: string
}

export function StatusBadge({ status, className }: StatusBadgeProps) {
  const getStatusConfig = () => {
    switch (status as PipelineStatus) {
      case 'ACTION_STATUS_PENDING':
        return {
          color: 'bg-yellow-100 text-yellow-800 border-yellow-200',
          icon: <Clock className="h-3 w-3 mr-1" />,
          label: 'En attente',
        }
      case 'ACTION_STATUS_SCHEDULED':
        return {
          color: 'bg-blue-100 text-blue-800 border-blue-200',
          icon: <Clock className="h-3 w-3 mr-1" />,
          label: 'Programmé',
        }
      case 'ACTION_STATUS_RUNNING':
        return {
          color: 'bg-purple-100 text-purple-800 border-purple-200',
          icon: <Loader2 className="h-3 w-3 mr-1 animate-spin" />,
          label: 'En cours',
        }
      case 'ACTION_STATUS_COMPLETED':
        return {
          color: 'bg-green-100 text-green-800 border-green-200',
          icon: <CheckCircle className="h-3 w-3 mr-1" />,
          label: 'Terminé',
        }
      case 'ACTION_STATUS_ERROR':
        return {
          color: 'bg-red-100 text-red-800 border-red-200',
          icon: <AlertCircle className="h-3 w-3 mr-1" />,
          label: 'Erreur',
        }
      default:
        return {
          color: 'bg-gray-100 text-gray-800 border-gray-200',
          icon: <Play className="h-3 w-3 mr-1" />,
          label: status,
        }
    }
  }

  const { color, icon, label } = getStatusConfig()

  return (
    <div className={cn('inline-flex items-center px-2 py-1 whitespace-nowrap rounded-full text-xs font-medium border', color, className)}>
      {icon}
      {label}
    </div>
  )
}
