import React, { FormEvent, useRef, useState } from 'react'
import { Button } from './ui/button'
import { CreateMonitor } from '@/types'
import { CirclePlus } from 'lucide-react'
import { Dialog, DialogContent, DialogTrigger } from './ui/dialog'
import { DialogHeader, DialogTitle } from './ui/dialog'
import { Input } from './ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select'
import { Label } from './ui/label'

interface MonitorModalProps {
  onSubmit: (data: CreateMonitor) => void
  disabled: boolean
}

export default function MonitorModal({ onSubmit, disabled }: MonitorModalProps) {
  const [eventType, setEventType] = useState('')
  const inputRef = useRef<HTMLInputElement>(null)
  const [fileName, setFileName] = useState('Aucun fichier')
  const [openMonitorModal, setOpenMonitorModal] = useState(false)

  const closeModal = () => {
    setOpenMonitorModal(false)
    setFileName('Aucun fichier')
    setEventType('')
  }

  const handleClick = () => {
    inputRef.current?.click()
  }

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    setFileName(file?.name || 'Aucun fichier')
  }
  const handleSubmit = (e: FormEvent) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget as HTMLFormElement)
    const data: CreateMonitor = {
      repo_name: formData.get('repo_name') as string,
      repo_owner: formData.get('repo_owner') as string,
      github_token: formData.get('github_token') as string,
      file: formData.get('file') as File,
      event: eventType,
    }

    onSubmit(data)
    closeModal()
  }

  return (
    <Dialog open={openMonitorModal} onOpenChange={setOpenMonitorModal}>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm" disabled={disabled}>
          <CirclePlus className={`h-4 w-4 mr-2`} />
          Ajouter
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Ajouter une configuration</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4 flex flex-col gap-2 ">
          <div className="flex flex-col gap-2">
            <Label>Nom du dépôt</Label>
            <Input type="text" name="repo_name" id="repo_name" required />
          </div>
          <div className="flex flex-col gap-2">
            <Label>Propriétaire du dépôt</Label>
            <Input type="text" name="repo_owner" id="repo_owner" required />
          </div>

          <div className="flex flex-col gap-2">
            <Label>Clé d'accès Github</Label>
            <Input type="text" name="github_token" id="github_token" required />
          </div>

          <div className="flex flex-col gap-2">
            <Label>Événement</Label>
            <Select onValueChange={setEventType} value={eventType}>
              <SelectTrigger>
                <SelectValue placeholder="Sélectionner un événement" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="Commit">Commit</SelectItem>
                <SelectItem value="PullRequest">Pull Request</SelectItem>
                <SelectItem value="Tag">Tag</SelectItem>
                <SelectItem value="All">Tous</SelectItem>
              </SelectContent>
            </Select>
          </div>

          <div className="flex flex-col gap-2">
            <Label>Fichier</Label>
            <input ref={inputRef} type="file" name="file" id="file" onChange={handleChange} className="hidden" />
            <div className="flex items-center gap-2">
              <Button type="button" variant="outline" onClick={handleClick}>
                Choisir un fichier
              </Button>
              <span className="text-sm text-muted-foreground truncate max-w-[200px]">{fileName}</span>
            </div>
          </div>

          <div className="flex justify-between gap-2">
            <Button type="button" variant="secondary" onClick={() => closeModal()}>
              Annuler
            </Button>
            <Button type="submit">Enregistrer</Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}
