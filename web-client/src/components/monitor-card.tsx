import { Monitor } from '@/types'
import { Card, CardContent, CardHeader, CardTitle } from './ui/card'
import { CalendarClock, Link, User } from 'lucide-react'

interface MonitorCardProps {
  monitor: Monitor
}

export default function MonitorCard({ monitor }: MonitorCardProps) {
  return (
    <Card className="h-full transition-all">
      <CardHeader>
        <CardTitle className="text-lg font-medium font-pixel">{monitor.repo_name}</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="flex items-center gap-1 text-sm text-muted-foreground mb-2">
          <User className="h-4 w-4" />
          <span>{monitor.repo_owner}</span>
        </div>
        <div className="flex items-center gap-1 text-sm text-muted-foreground mb-2">
          <Link className="h-4 w-4" />
          <a href={monitor.repo_url} target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">
            {monitor.repo_url}
          </a>
        </div>
        <div className="flex items-center gap-1 text-sm text-muted-foreground">
          <CalendarClock className="h-4 w-4" />
          <span>{monitor.events.join(', ')}</span>
        </div>
      </CardContent>
    </Card>
  )
}
