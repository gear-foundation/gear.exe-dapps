import logo from "@/assets/logo.png";
import { WalletButton } from "../wallet/WalletButton";
import styles from "./Header.module.scss";

export const Header = () => {
  return (
    <header className={styles.container}>
      <img src={logo} className={styles.logo} alt="Gear logo" />

      <WalletButton />
    </header>
  );
};
