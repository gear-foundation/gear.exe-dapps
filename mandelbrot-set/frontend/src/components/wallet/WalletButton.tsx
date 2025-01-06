import { useAppKit } from "@reown/appkit/react";
import { useAccount } from "wagmi";
import { Button, EncryptButton } from "@/components";
import { useIsTablet } from "@/lib/hooks/use-media";
import BurgerMenu from "@/assets/icons/burger-menu.svg?react";

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
    <EncryptButton
      onClick={() => open()}
      targetText="Connect wallet"
      className="min-w-[168px]"
    />
  );
};
