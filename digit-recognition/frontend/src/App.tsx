import "./index.css";
import { useAccount } from "wagmi";

import { WalletButton, Header, Layout } from "@/components";
import { Recognition } from "./components/recognition/Recognition";
import { ETH_CHAIN_ID } from "./consts";
import styles from "./App.module.scss";

function App() {
  const ethAccount = useAccount();
  const isConnected = Boolean(
    ethAccount.chain && ethAccount.chain.id === ETH_CHAIN_ID
  );

  return (
    <>
      <Header />
      {!isConnected && (
        <Layout className={styles.connectionWrapper}>
          <h1>AI Image Recognition</h1>

          <WalletButton />
        </Layout>
      )}
      {isConnected && <Recognition />}
    </>
  );
}

export default App;
