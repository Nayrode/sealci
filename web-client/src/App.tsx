import { Outlet } from 'react-router-dom'
import { Header } from './components/header'
import { PipelineProvider } from './contexts/PipelineContext'

function App() {
  return (
    <PipelineProvider>
      <Header />
      <Outlet />
    </PipelineProvider>
  )
}

export default App
