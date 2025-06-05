import { fetchPipeline, fetchPipelines } from '@/lib/api'
import { useQuery } from '@tanstack/react-query'

export const useGetPipelines = (verbose: boolean) => {
  const { data, error, isPending, refetch } = useQuery({
    queryKey: ['pipelines'],
    queryFn: () => fetchPipelines({ verbose }),
  })
  return { data, error, isPending, refetch }
}

export const useGetPipeline = (verbose: boolean, id: string) => {
  const { data, error, isPending, refetch } = useQuery({
    queryKey: ['pipeline', id],
    queryFn: () => fetchPipeline({ verbose, id }),
  })
  return { data, error, isPending, refetch }
}
