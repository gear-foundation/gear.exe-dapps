import { DigitRecognizer, CatIdentifier } from "@/features";
import { Layout } from "@/components";
import ExportSvg from "@/assets/icons/export.svg?react";
import styles from "./Recognition.module.scss";
import { useAccount } from "wagmi";

export const Recognition = () => {
  const ethAccount = useAccount();

  return (
    <Layout>
      <h1 className={styles.title}>AI Image Recognition</h1>

      <p className={styles.description}>
        //_Draw a digit or upload an image to experience the power of AI.
        Gear.EXE recognition model identifies handwritten numbers (0–9) or
        determines if an image contains cats. Submit, see results instantly, and
        start again.
      </p>

      <div className={styles.list}>
        <DigitRecognizer />

        <CatIdentifier />
      </div>

      <div className={styles.links}>
        <a
          href={`https://holesky.etherscan.io/address/${ethAccount.address}`}
          target="_blank"
          rel="noopener noreferrer"
          className={styles.link}
        >
          <ExportSvg /> View in Blockchain Explorer
        </a>
      </div>
    </Layout>
  );
};
