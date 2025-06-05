import { fetchPipelines } from '@/lib/api'
import { useQuery } from '@tanstack/react-query'

export const useGetPipelines = (verbose: boolean) => {
  const { data, error, isFetching, refetch } = useQuery({
    queryKey: ['pipelines'],
    queryFn: () => fetchPipelines({ verbose }),
  })
  return { data, error, isFetching, refetch }
}