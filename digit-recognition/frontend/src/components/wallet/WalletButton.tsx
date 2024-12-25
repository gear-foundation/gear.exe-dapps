import { useAppKit } from "@reown/appkit/react";
import { useAccount } from "wagmi";
import { Button } from "@/components";
import { useIsTablet } from "@/lib/hooks/use-media";
import BurgerMenu from "@/assets/icons/burger-menu.svg?react";
import styles from "./WalletButton.module.scss";

export const WalletButton = () => {
  const ethAccount = useAccount();
  const { open } = useAppKit();
  const isConnected = Boolean(ethAccount.chainId);
  const isTablet = useIsTablet();

  return isConnected ? (
    <>
      <Button onClick={() => open()}>
        {isTablet ? ethAccount.address : <BurgerMenu />}
      </Button>
    </>
  ) : (
    <Button onClick={() => open()} className={styles.connect}>
      Connect wallet
    </Button>
  );
};
