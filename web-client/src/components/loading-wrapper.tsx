import React, { useState, useEffect } from 'react'

interface LoadingWrapperProps {
  isLoading: boolean
  minSkeletonTime?: number
  skeleton: React.ReactNode
  empty: React.ReactNode
  hasData: boolean
  children: React.ReactNode
}

export default function LoadingWrapper({ isLoading, minSkeletonTime = 500, skeleton, empty, hasData, children }: LoadingWrapperProps) {
  const [showSkeleton, setShowSkeleton] = useState(true)

  useEffect(() => {
    if (!isLoading) {
      const timeout = setTimeout(() => setShowSkeleton(false), minSkeletonTime)
      return () => clearTimeout(timeout)
    } else {
      setShowSkeleton(true)
    }
  }, [isLoading, minSkeletonTime])

  if (isLoading || showSkeleton) {
    return <>{skeleton}</>
  }

  if (!hasData) {
    return <>{empty}</>
  }

  return <>{children}</>
}
