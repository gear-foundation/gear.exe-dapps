import { clsx } from "clsx";
import styles from "./Card.module.scss";
import { HashLink } from "../hashLink/HashLink";

type Props = {
  title: string;
  address?: string;
  headerSlot?: React.ReactNode;
  canvasSlot?: React.ReactNode;
  description?: React.ReactNode;
  footer?: React.ReactNode;
};

export const Card = ({
  title,
  headerSlot,
  canvasSlot,
  address,
  description,
  footer,
}: Props) => {
  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <span className={styles.title}>{title}</span>
        {headerSlot}
      </div>

      <div className={styles.canvas}>
        <div className={clsx(styles.corner, styles.topLeft)} />
        <div className={clsx(styles.corner, styles.topRight)} />
        <div className={clsx(styles.corner, styles.bottomLeft)} />
        <div className={clsx(styles.corner, styles.bottomRight)} />
        {canvasSlot}
      </div>

      <div className={styles.address}>
        {address && (
          <HashLink
            hash={address}
            href={`https://holesky.etherscan.io/address/${address}`}
          />
        )}
      </div>
      <div className={styles.description}>{description}</div>
      <div className={styles.footer}>{footer}</div>
    </div>
  );
};
