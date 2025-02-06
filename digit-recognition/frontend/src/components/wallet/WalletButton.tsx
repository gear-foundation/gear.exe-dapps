import { useAppKit } from "@reown/appkit/react";
import { useAccount } from "wagmi";
import { Button } from "@/components";
import BurgerMenu from "@/assets/icons/burger-menu.svg?react";
import styles from "./WalletButton.module.scss";

export const WalletButton = () => {
  const ethAccount = useAccount();
  const { open } = useAppKit();
  const isConnected = Boolean(ethAccount.chainId);

  return isConnected ? (
    <>
      <Button onClick={() => open()} className={styles.button}>
        <span>{ethAccount.address}</span>
        <BurgerMenu />
      </Button>
    </>
  ) : (
    <Button onClick={() => open()} className={styles.connect}>
      Connect wallet
    </Button>
  );
};
