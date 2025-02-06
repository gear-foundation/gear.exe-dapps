import { useState } from "react";
import { Button } from "@/components";
import Copy from "@/assets/icons/document-copy.svg?react";
import { copyToClipboard } from "@/lib/utils";
import styles from "./HashLink.module.scss";

type Props = React.AnchorHTMLAttributes<HTMLAnchorElement> & {
  hash: string;
  isExternalLink?: boolean;
};

export const HashLink = ({
  isExternalLink = true,
  hash,
  ...restProps
}: Props) => {
  const [showTip, setShowTip] = useState(false);
  const onCopy = () => {
    copyToClipboard({
      value: hash,
      onSuccess: () => {
        setShowTip(true);
        setTimeout(() => {
          setShowTip(false);
        }, 1000);
      },
    });
  };

  return (
    <div className={styles.wrapper}>
      <a
        target={isExternalLink ? "_blank" : undefined}
        rel={isExternalLink ? "noreferrer" : undefined}
        className={styles.link}
        {...restProps}
      >
        {hash}
      </a>
      <Button
        variant="link"
        onClick={onCopy}
        className={styles.button}
        size="icon"
      >
        <Copy />
        {showTip && <div className={styles.tip}>Copied</div>}
      </Button>
    </div>
  );
};
