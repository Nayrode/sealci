import { fetchMonitors } from '@/lib/api'
import { useQuery } from '@tanstack/react-query'

export const useMonitors = () => {
  const { data, error, isPending, refetch } = useQuery({
    queryKey: ['monitors'],
    queryFn: () => fetchMonitors(),
  })
  return { data, error, isPending, refetch }
}

// export const usePipeline = (verbose: boolean, id: string) => {
//   const { data, error, isPending, refetch } = useQuery({
//     queryKey: ['pipeline', id],
//     queryFn: () => fetchPipeline({ verbose, id }),
//   })
//   return { data, error, isPending, refetch }
// }
