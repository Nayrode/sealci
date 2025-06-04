import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import PipelinesPage from "./pages/pipelines.tsx";
import PipelinePage from "./pages/pipeline.tsx";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import MonitorConfigurations from "./pages/monitor-configurations.tsx";

const queryClient = new QueryClient();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<App />}>
            <Route path="/" element={<PipelinesPage />} />
            <Route path="/:id" element={<PipelinePage />} />
            <Route path="/configurations" element={<MonitorConfigurations />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  </StrictMode>,
);
