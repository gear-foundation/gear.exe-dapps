import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import { EthProvider, QueryProvider } from "./providers.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryProvider>
      <EthProvider>
        <App />
      </EthProvider>
    </QueryProvider>
  </StrictMode>
);
