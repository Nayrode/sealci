import { Button } from "@/components/ui/button";
import { House } from "lucide-react";
import { NavigateFunction, useNavigate } from "react-router-dom";

export default function NotFoundPage() {
    const navigation: NavigateFunction = useNavigate()

    return (
        <div className="flex items-center justify-center h-screen">
            <div className="flex flex-col items-center gap-4">
                <h1>Oups! Il semblerait que vous vous soyez perdu...</h1>
                <Button variant="outline" onClick={() => navigation('/')}>
                    <House className={`h-4 w-4 mr-2`} />
                    Revenir à l'accueil
                </Button>
            </div>
        </div>
    )
}