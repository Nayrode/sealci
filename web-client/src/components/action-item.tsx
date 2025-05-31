import type { Action } from '@/types'
import { StatusBadge } from './status-badge'
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@/components/ui/accordion'
import { Code, Terminal } from 'lucide-react'

interface ActionItemProps {
  action: Action
}

export function ActionItem({ action }: ActionItemProps) {
  return (
    <Accordion type="single" collapsible className="w-full">
      <AccordionItem value={`action-${action.id}`} className="border rounded-md mb-4">
        <AccordionTrigger className="px-4 py-3 hover:no-underline">
          <div className="flex items-center justify-between w-full">
            <div className="flex items-center gap-3">
              <StatusBadge status={action.status} />
              <span className="font-medium">{action.name}</span>
            </div>
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <Terminal className="h-4 w-4" />
              <span>{action.commands.length} commandes</span>
            </div>
          </div>
        </AccordionTrigger>
        <AccordionContent className="px-4 pb-4">
          <div className="space-y-4">
            <div>
              <h4 className="text-sm font-medium mb-2 flex items-center gap-1">
                <Code className="h-4 w-4" />
                Commandes
              </h4>
              <div className="bg-muted p-3 rounded-md text-sm font-mono">
                {action.commands.map((cmd, index) => (
                  <div key={index} className="pb-1">
                    $ {cmd}
                  </div>
                ))}
              </div>
            </div>

            {action.logs && action.logs.length > 0 && (
              <div>
                <h4 className="text-sm font-medium mb-2 flex items-center gap-1">
                  <Terminal className="h-4 w-4" />
                  Logs
                </h4>
                <div className="bg-black text-green-400 p-3 rounded-md text-sm font-mono h-[200px] overflow-y-auto">
                  {action.logs.map((log, index) => (
                    <div key={index}>{log}</div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  )
}
