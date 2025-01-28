import { DigitRecognizer, CatIdentifier } from "@/features";
import { Layout } from "@/components";
import styles from "./Recognition.module.scss";

export const Recognition = () => {
  return (
    <Layout>
      <h1 className={styles.title}>AI Image Recognition</h1>

      <p className={styles.description}>
        //_Draw a digit or upload an image to experience the power of AI.
        Gear.EXE recognition model identifies handwritten numbers (0–9) or
        determines if an image contains cats. Submit, see results
        instantly, and start again.
      </p>

      <div className={styles.list}>
        <DigitRecognizer />

        <CatIdentifier />
      </div>
    </Layout>
  );
};
