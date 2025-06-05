import { fetchMonitors } from '@/lib/api'
import { useQuery } from '@tanstack/react-query'

export const useGetMonitors = () => {
  const { data, error, isFetching, refetch } = useQuery({
    queryKey: ['monitors'],
    queryFn: () => fetchMonitors(),
  })
  return { data, error, isFetching, refetch }
}

// export const usePipeline = (verbose: boolean, id: string) => {
//   const { data, error, isPending, refetch } = useQuery({
//     queryKey: ['pipeline', id],
//     queryFn: () => fetchPipeline({ verbose, id }),
//   })
//   return { data, error, isPending, refetch }
// }
