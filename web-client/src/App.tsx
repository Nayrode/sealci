import { Outlet } from 'react-router-dom'
import { Header } from './components/header'
import { PipelineProvider } from './contexts/pipeline-context'
import { MonitorProvider } from './contexts/monitor-context'

function App() {
  return (
    <MonitorProvider>
      <PipelineProvider>
        <Header />
        <Outlet />
      </PipelineProvider>
    </MonitorProvider>
  )
}

export default App
