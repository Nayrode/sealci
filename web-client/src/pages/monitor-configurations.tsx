import { Button } from "@/components/ui/button"
import { useMonitors } from "@/hooks/use-monitors"
import { CirclePlus, RefreshCw } from "lucide-react"
import { FormEvent, useState } from "react"

export default function MonitorConfigurations() {
    const { data: monitors, isPending, refetch } = useMonitors()

    const [addMonitor, setAddMonitor] = useState(false)

    return (
        <>
            <main className="flex-1 container py-6">
                <div className="flex justify-between items-center mb-6">
                    <h1 className="text-2xl font-bold">Configurations</h1>
                    <div className="flex items-center gap-2">
                        <Button variant="outline" size="sm" onClick={() => refetch()} disabled={isPending}>
                            <RefreshCw className={`h-4 w-4 mr-2 ${isPending ? 'animate-spin' : ''}`} />
                            Actualiser
                        </Button>
                        <Button variant="outline" size="sm" onClick={() => setAddMonitor(true)} disabled={isPending}>
                            <CirclePlus className={`h-4 w-4 mr-2`} />
                            Ajouter
                        </Button>
                    </div>
                </div>

                {isPending ? (
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
                    <div className="border">
                        {monitors.map((monitor) => (
                            <div key={monitor.repo_url} className="p-4 rounded-md bg-white shadow-sm">
                                <h3 className="font-semibold">{monitor.repo_name}</h3>
                                <p>{monitor.repo_owner}</p>
                                <p>{monitor.repo_url}</p>
                                <p>Événements: {monitor.events.join(", ")}</p>
                            </div>
                        ))}
                    </div>
                )}
            </main>
            {
                addMonitor && (
                    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                        <div className="bg-white p-6 rounded-md shadow-lg w-full max-w-md">
                            <h2 className="text-xl font-semibold mb-4">Ajouter une configuration de surveillance</h2>
                            {/* Formulaire d'ajout de configuration */}
                            <form onSubmit={(e: FormEvent) => {console.log("Form submitted"); console.log(e.currentTarget); e.preventDefault(); }}>
                                <input type="text" placeholder="Nom du dépôt" className="w-full p-2 border rounded mb-4" />
                                <input type="text" placeholder="Propriétaire du dépôt" className="w-full p-2 border rounded mb-4" />
                                <input type="text" placeholder="URL du dépôt" className="w-full p-2 border rounded mb-4" />
                                <select className="w-full p-2 border rounded mb-4">
                                    <option value="">Sélectionner un événement</option>
                                    <option value="push">Push</option>
                                    <option value="pull_request">Pull Request</option>
                                    <option value="issue">Issue</option>
                                    {/* Ajouter d'autres événements si nécessaire */}
                                </select>
                                <input type="file" className="w-full p-2 border rounded mb-4" />

                                <div className="flex justify-between items-center gap-2">
                                    <Button variant="secondary" onClick={() => setAddMonitor(false)} className="mt-2">Annuler</Button>
                                    <Button type="submit" className="mt-4">Enregistrer</Button>
                                </div>
                            </form>
                        </div>
                    </div>
                )
            }
        </>
    )
}