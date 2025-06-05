import { fetchMonitors } from '@/lib/api'
import { useQuery } from '@tanstack/react-query'

export const useGetMonitors = () => {
  const { data, error, isFetching, refetch } = useQuery({
    queryKey: ['monitors'],
    queryFn: () => fetchMonitors(),
  })
  return { data, error, isFetching, refetch }
}
