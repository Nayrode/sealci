import { FormEvent } from "react";
import { Button } from "./ui/button";
import { CreateMonitor } from "@/types";

interface MonitorModalProps {
    closeModal: () => void;
    onSubmit: (data: CreateMonitor) => void;
}

export default function MonitorModal({ closeModal, onSubmit }: MonitorModalProps) {

    const handleSubmit = (e: FormEvent) => {
        e.preventDefault();
        const formData = new FormData(e.currentTarget as HTMLFormElement);
        const data: CreateMonitor = {
            repo_name: formData.get("repo_name") as string,
            repo_owner: formData.get("repo_owner") as string,
            event: formData.get("event") as string,
            file: formData.get("file") as File,
            github_token: formData.get("github_token") as string,
        };

        onSubmit(data);
        closeModal();
    }

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white p-6 rounded-md shadow-lg w-full max-w-md">
                <h2 className="text-xl font-semibold mb-4">Ajouter une configuration de surveillance</h2>
                {/* Formulaire d'ajout de configuration */}
                <form onSubmit={handleSubmit} className="flex flex-col gap-4">
                    <input type="text" name="repo_name" placeholder="Nom du dépôt" className="w-full p-2 border rounded" />
                    <input type="text" name="repo_owner" placeholder="Propriétaire du dépôt" className="w-full p-2 border rounded" />
                    <input type="text" name="github_token" placeholder="Clé d'accès au dépôt Github" className="w-full p-2 border rounded" />
                    <select name="event" className="w-full p-2 border rounded">
                        <option value="">Sélectionner un événement</option>
                        <option value="Commit">Commit</option>
                        <option value="PullRequest">Pull Request</option>
                        <option value="Tag">Tag</option>
                        <option value="All">Tous</option>
                    </select>
                    <input type="file" name="file" className="w-full p-2 border rounded" />

                    <div className="flex justify-between items-center gap-2">
                        <Button variant="secondary" onClick={() => closeModal()} className="mt-2">Annuler</Button>
                        <Button type="submit" className="mt-4">Enregistrer</Button>
                    </div>
                </form>
            </div>
        </div>
    )
}