import { Outlet } from 'react-router-dom'
import { Header } from './components/header'
import { PipelineProvider } from './contexts/PipelineContext'
import { MonitorProvider } from './contexts/MonitorContext'

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
