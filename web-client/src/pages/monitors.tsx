import MonitorCard from "@/components/monitor-card"
import { Skeleton } from "@/components/ui/skeleton"
import { useMonitorContext } from "@/contexts/monitor-context"
import { motion, AnimatePresence } from "framer-motion"
import MonitorModal from "@/components/monitor-modal"
import ReloadButton from "@/components/reload-button"
import LoadingWrapper from "@/components/loading-wrapper"

export default function MonitorsPage() {
    const { monitors, isLoading, addMonitor, realoadMonitors } = useMonitorContext()

    const monitorsList = monitors ?? []
    const isLoadingValue = isLoading.get ?? false

    const skeletonContent = (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {[...Array(4)].map((_, i) => (
                <div key={i} className="w-full border border-gray-200 rounded-xl animate-pulse p-4">
                    <div className="flex flex-col gap-6">
                        <Skeleton className="h-5 w-1/3" />
                        <div className="flex flex-col gap-3">
                            <Skeleton className="h-5 w-1/2" />
                            <Skeleton className="h-5 w-full" />
                            <Skeleton className="h-5 w-1/3" />
                        </div>
                    </div>
                </div>
            ))}
        </div>
    )

    const emptyContent = (
        <div className="text-center py-12">
            <h2 className="text-xl font-medium mb-2">Aucune configuration trouvée</h2>
            <p className="text-muted-foreground">Aucune configuration de surveillance n'a été trouvée.</p>
        </div>
    )

    return (
        <main className="flex-1 container py-6">
            <div className="flex justify-between items-center mb-6">
                <h1 className="text-2xl font-bold">Configurations</h1>
                <div className="flex items-center gap-2">
                    <ReloadButton isLoading={isLoadingValue} onClick={realoadMonitors} />
                    <MonitorModal disabled={isLoadingValue} onSubmit={addMonitor} />
                </div>
            </div>

            <LoadingWrapper
                isLoading={isLoadingValue}
                skeleton={skeletonContent}
                empty={emptyContent}
                hasData={monitorsList.length > 0}
            >
                <AnimatePresence>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {monitorsList.map((monitor, index) => (
                            <motion.div
                                key={index + "_" + monitor.repo_name}
                                initial={{ opacity: 0, y: 10 }}
                                animate={{ opacity: 1, y: 0 }}
                                exit={{ opacity: 0, y: 10 }}
                                transition={{ duration: 0.4, delay: index * 0.1 }}
                            >
                                <MonitorCard monitor={monitor} />
                            </motion.div>
                        ))}
                    </div>
                </AnimatePresence>
            </LoadingWrapper>
        </main>
    )
}
