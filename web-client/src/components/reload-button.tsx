import { RefreshCw } from 'lucide-react'
import { Button } from './ui/button'
import React from 'react'

interface ReloadButtonProps {
  isLoading: boolean
  onClick: () => void
}

export default function ReloadButton({ isLoading, onClick }: ReloadButtonProps) {
  const [spinning, setSpinning] = React.useState(false)

  React.useEffect(() => {
    let timer: string | number | NodeJS.Timeout | undefined
    if (isLoading) {
      setSpinning(true)
    } else {
      timer = setTimeout(() => setSpinning(false), 500)
    }
    return () => clearTimeout(timer)
  }, [isLoading])

  return (
    <Button variant="outline" size="sm" onClick={onClick} disabled={isLoading}>
      <RefreshCw className={`h-4 w-4 mr-2 ${spinning ? 'animate-spin' : ''}`} />
      Actualiser
    </Button>
  )
}
