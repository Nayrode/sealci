import MonitorCard from "@/components/monitor-card"
import MonitorModal from "@/components/monitor-modal"
import { Button } from "@/components/ui/button"
import { useMonitorContext } from "@/contexts/monitor-context"
import {  RefreshCw } from "lucide-react"

export default function MonitorsPage() {
    const { monitors, isLoading, addMonitor, realoadMonitors } = useMonitorContext()

    return (
        <>
            <main className="flex-1 container py-6">
                <div className="flex justify-between items-center mb-6">
                    <h1 className="text-2xl font-bold">Configurations</h1>
                    <div className="flex items-center gap-2">
                        <Button variant="outline" size="sm" onClick={() => realoadMonitors()} disabled={isLoading.get}>
                            <RefreshCw className={`h-4 w-4 mr-2 ${isLoading.get ? 'animate-spin' : ''}`} />
                            Actualiser
                        </Button>
                        <MonitorModal disabled={isLoading.get} onSubmit={addMonitor}></MonitorModal>
                    </div>
                </div>

                {isLoading.get ? (
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {[...Array(6)].map((_, i) => (
                            <div key={i} className="w-full border border-gray-200 rounded-xl animate-pulse p-4">
                                <div className="flex flex-col gap-6">
                                    <div className="h-5 bg-gray-300 rounded w-1/3"></div>
                                    <div className="flex flex-col gap-3">
                                        <div className="h-5 bg-gray-300 rounded w-1/2"></div>
                                        <div className="h-5 bg-gray-300 rounded w-full"></div>
                                        <div className="h-5 bg-gray-300 rounded w-1/3"></div>
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                ) : !monitors || monitors.length === 0 ? (
                    <div className="text-center py-12">
                        <h2 className="text-xl font-medium mb-2">Aucune configuration trouvée</h2>
                        <p className="text-muted-foreground">Aucune configuration de surveillance n'a été trouvée.</p>
                    </div>
                ) : (
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {monitors.map((monitor) => (
                            <MonitorCard monitor={monitor} key={`${monitor.repo_owner} - ${monitor.repo_url}`} />
                        ))}
                    </div>
                )}
            </main>
        </>
    )
}