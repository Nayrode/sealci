import { createContext, useContext, ReactNode, useState, useEffect } from 'react'
import type { CreateMonitor, Monitor } from '@/types'
import { useGetMonitors } from '@/hooks/use-monitors';
import { createMonitor } from '@/lib/api';

type MonitorContextType = {
  monitors: Monitor[] | undefined;
  isLoading: {
    get: boolean;
    add: boolean;
    update: boolean;
    remove: boolean;
  };
  addMonitor: (monitor: CreateMonitor) => void;
  updateMonitor: (monitor: CreateMonitor) => void;
  removeMonitor: (repoOwner: string, repoName: string) => void;
  realoadMonitors: () => void;
}

const MonitorContext = createContext<MonitorContextType | undefined>(undefined)

export function MonitorProvider({ children }: { children: ReactNode }) {
  const { data: fetchedMonitors, isFetching, refetch } = useGetMonitors()

  const [monitors, setMonitors] = useState<Monitor[] | undefined>(fetchedMonitors)

  const addMonitor = async (monitor: CreateMonitor) => {
    const result = await createMonitor(monitor)

    if (result) {
      setMonitors((prev) => [...(prev || []), result])
    }
  }

  useEffect(() => {
    setMonitors(fetchedMonitors)
  }, [fetchedMonitors])

  const values = {
    monitors,
    isLoading: {
      get: isFetching,
      add: false,
      update: false,
      remove: false,
    },
    addMonitor: addMonitor,
    updateMonitor: (monitor: CreateMonitor) => {
      setMonitors((prev) =>
        prev?.map((m) => (m.repo_owner === monitor.repo_owner && m.repo_name === monitor.repo_name ? { ...m, ...monitor } : m))
      )
    },
    removeMonitor: (repoOwner: string, repoName: string) => {
      setMonitors((prev) => prev?.filter((m) => !(m.repo_owner === repoOwner && m.repo_name === repoName)))
    },
    realoadMonitors: () => refetch(),
  }

  return <MonitorContext.Provider value={values}>{children}</MonitorContext.Provider>
}

export function useMonitorContext() {
  const context = useContext(MonitorContext)
  if (context === undefined) {
    throw new Error('useMonitorContext must be used within a MonitorProvider')
  }
  return context
}
