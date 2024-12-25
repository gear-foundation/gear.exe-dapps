import "./globals.css";
import { useAccount } from "wagmi";

import { WalletButton, Header, Layout } from "@/components";
import { Recognition } from "./components/recognition/Recognition";

function App() {
  const ethAccount = useAccount();
  const isConnected = Boolean(ethAccount.chain);

  return (
    <>
      <Header />
      {!isConnected && (
        <Layout className="flex flex-col items-center justify-center gap-12 h-screen-header pt-0">
          <h1>AI Image Recognition</h1>

          <WalletButton />
        </Layout>
      )}
      {isConnected && <Recognition />}
    </>
  );
}

export default App;
