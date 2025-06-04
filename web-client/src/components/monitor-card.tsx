import { Monitor } from "@/types";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";

interface MonitorCardProps {
  monitor: Monitor
}

export default function MonitorCard({ monitor }: MonitorCardProps) {

    return (
        <Card className="h-full transition-all ">
            <CardHeader>
                <CardTitle className="text-lg font-medium font-pixel">
                    {monitor.repo_name}
                </CardTitle>
            </CardHeader>
            <CardContent>
                <div className="text-sm text-muted-foreground mb-2">
                    <span className="font-semibold">Propriétaire:</span> {monitor.repo_owner}
                </div>
                <div className="text-sm text-muted-foreground mb-2">
                    <span className="font-semibold">URL du dépôt:</span> <a href={monitor.repo_url} target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">{monitor.repo_url}</a>
                </div>
                <div className="text-sm text-muted-foreground">
                    <span className="font-semibold">Événements:</span> {monitor.events.join(", ")}
                </div>
            </CardContent>
        </Card>
    )
}