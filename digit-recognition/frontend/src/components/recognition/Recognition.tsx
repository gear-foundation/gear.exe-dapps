import { DigitRecognizer, CatIdentifier } from "@/features";
import { Layout } from "@/components";
import ExportSvg from "@/assets/icons/export.svg?react";
import styles from "./Recognition.module.scss";
import {
  CAT_IDENTIFIER_CONTRACT_ADDRESS,
  DIGIT_RECOGNITION_CONTRACT_ADDRESS,
} from "@/consts";
import { useReadContract } from "wagmi";
import { catDogIdentifierAbi } from "@/features/cat-identifier/api/catDogIdentifierAbi";

export const Recognition = () => {
  const { data: catIdentifierMirrorId } = useReadContract({
    abi: catDogIdentifierAbi,
    address: CAT_IDENTIFIER_CONTRACT_ADDRESS,
    functionName: "mirror",
  });

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
          href={`https://holesky.etherscan.io/address/${DIGIT_RECOGNITION_CONTRACT_ADDRESS}`}
          target="_blank"
          rel="noopener noreferrer"
          className={styles.link}
        >
          View digit recognizer in Blockchain Explorer <ExportSvg />
        </a>

        <a
          href={`https://holesky.etherscan.io/address/${catIdentifierMirrorId}`}
          target="_blank"
          rel="noopener noreferrer"
          className={styles.link}
        >
          View Cat identifier in Blockchain Explorer <ExportSvg />
        </a>
      </div>
    </Layout>
  );
};
