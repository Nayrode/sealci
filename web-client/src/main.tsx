import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import PipelinesPage from "./pages/pipelines.tsx";
import PipelinePage from "./pages/pipeline.tsx";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import MonitorsPage from "./pages/monitors.tsx";
import NotFoundPage from "./pages/404.tsx";

const queryClient = new QueryClient();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<App />}>
            <Route index element={<Navigate to="/pipelines" replace />} />
            <Route path="/pipelines" element={<PipelinesPage />} />
            <Route path="/pipelines/:id" element={<PipelinePage />} />
            <Route path="/configurations" element={<MonitorsPage />} />
            <Route path="*" element={<NotFoundPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  </StrictMode>,
);
