import { cn } from '@/lib/utils'
import { Link } from 'react-router-dom'
import { usePipelineContext } from '@/contexts/pipeline-context'
import { SlidersHorizontal } from 'lucide-react'

export function Header() {
  const { currentPipeline } = usePipelineContext()
  const pathname = window.location.pathname
  const isDetailPage = pathname !== '/'

  // Extraire l'organisation et le nom du repo à partir de l'URL
  const getRepoInfo = (url: string) => {
    try {
      const urlObj = new URL(url)
      const pathParts = urlObj.pathname.split('/').filter(Boolean)
      return pathParts.length >= 2 ? { org: pathParts[0], repo: pathParts[1] } : { org: '', repo: url }
    } catch {
      return { org: '', repo: url }
    }
  }

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur">
      <div className="container flex h-14 items-center justify-between">
        <div className="flex items-center gap-6 text-sm">
          <Link to="/" className={cn('flex gap-2 items-center transition-colors hover:text-foreground/80', isDetailPage ? 'text-foreground/60' : 'text-foreground font-medium')}>
            <img src="/logo.svg" alt="Logo" className="h-6 w-6" />
            <p>SealCI</p>
          </Link>

          {isDetailPage && currentPipeline && (
            <>
              <span className="text-foreground/60">/</span>
              {(() => {
                const { org, repo } = getRepoInfo(currentPipeline.repository_url)
                return (
                  <>
                    <span className="text-foreground/60">{org}</span>
                    <span className="text-foreground/60">/</span>
                    <span className="font-medium">{repo}</span>
                  </>
                )
              })()}
            </>
          )}
        </div>

        <div className='flex flex-row items-center gap-12'>
          <Link to="/configurations" className={cn('flex gap-2 items-center transition-colors hover:text-foreground/80', isDetailPage ? 'text-foreground/60' : 'text-foreground font-medium')}>
            <SlidersHorizontal className="h-4 w-4" />
            <p>Configurations</p>
          </Link>

          <a href="https://sealci.com/docs" className="underline">
            Docs
          </a>
        </div>
      </div>
    </header>
  )
}
