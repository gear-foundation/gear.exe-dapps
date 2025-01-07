import "./globals.css";
import { useAccount } from "wagmi";

import { WalletButton, Header, Layout, ComputationForm } from "@/components";

function App() {
  const ethAccount = useAccount();
  const isConnected = Boolean(ethAccount.chain);

  return (
    <>
      <Header />
      {!isConnected && (
        <Layout className="flex flex-col items-center justify-center gap-12 h-screen-header pt-0">
          <h1>Distributed Computation</h1>

          <WalletButton />
        </Layout>
      )}
      {isConnected && <ComputationForm />}
    </>
  );
}

export default App;
