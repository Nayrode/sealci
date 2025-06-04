import { createContext, useContext, ReactNode, useState } from 'react'
import type { Monitor } from '@/types'

type MonitorContextType = {
  monitors: Monitor[] | undefined;
  setMonitors: (monitors: Monitor[] | undefined) => void;
}

const MonitorContext = createContext<MonitorContextType | undefined>(undefined)

export function MonitorProvider({ children }: { children: ReactNode }) {
  const [monitors, setMonitors] = useState<Monitor[] | undefined>(undefined)

  return <MonitorContext.Provider value={{ monitors, setMonitors }}>{children}</MonitorContext.Provider>
}

export function useMonitorContext() {
  const context = useContext(MonitorContext)
  if (context === undefined) {
    throw new Error('useMonitorContext must be used within a MonitorProvider')
  }
  return context
}
