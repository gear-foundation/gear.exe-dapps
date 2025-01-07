import logo from "@/assets/logo.png";
import { WalletButton } from "../wallet/WalletButton";

export const Header = () => {
  return (
    <header className="w-screen max-w-screen-2xl mx-auto">
      <div className="flex justify-between items-center py-[16px] px-6 h-[78px]">
        <img src={logo} className="w-[166px]" alt="Gear logo" />

        <WalletButton />
      </div>
    </header>
  );
};
