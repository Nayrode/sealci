import MonitorCard from "@/components/monitor-card"
import MonitorModal from "@/components/monitor-modal"
import { Button } from "@/components/ui/button"
import { useMonitorContext } from "@/contexts/MonitorContext"
import { CirclePlus, RefreshCw } from "lucide-react"
import { useState } from "react"

export default function Monitors() {
    const { monitors, isLoading, addMonitor, realoadMonitors } = useMonitorContext()

    const [openMonitorModal, setOpenMonitorModal] = useState(false)

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
                        <Button variant="outline" size="sm" onClick={() => setOpenMonitorModal(true)} disabled={isLoading.get}>
                            <CirclePlus className={`h-4 w-4 mr-2`} />
                            Ajouter
                        </Button>
                    </div>
                </div>

                {isLoading.get ? (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        {[...Array(6)].map((_, i) => (
                            <div key={i} className="h-[180px] rounded-md bg-muted animate-pulse" />
                        ))}
                    </div>
                ) : !monitors || monitors.length === 0 ? (
                    <div className="text-center py-12">
                        <h2 className="text-xl font-medium mb-2">Aucune configuration trouvée</h2>
                        <p className="text-muted-foreground">Aucune configuration de surveillance n'a été trouvée.</p>
                    </div>
                ) : (
                    <div className="flex flex-col gap-4">
                        {monitors.map((monitor) => (
                            <MonitorCard monitor={monitor} key={`${monitor.repo_owner} - ${monitor.repo_url}`} />
                        ))}
                    </div>
                )}
            </main>
            {
                openMonitorModal && (
                    <MonitorModal closeModal={() => setOpenMonitorModal(false)} onSubmit={addMonitor} />
                )
            }
        </>
    )
}