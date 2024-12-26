import "./index.css";
import { useAccount } from "wagmi";

import { WalletButton, Header, Layout } from "@/components";
import { Recognition } from "./components/recognition/Recognition";
import styles from "./App.module.scss";

function App() {
  const ethAccount = useAccount();
  const isConnected = Boolean(ethAccount.chain);

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
