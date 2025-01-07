import { useState } from "react";
import { Button } from "@/components";
import Copy from "@/assets/icons/document-copy.svg?react";
import { copyToClipboard } from "@/lib/utils";

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
    <div className="flex gap-2">
      <a
        target={isExternalLink ? "_blank" : undefined}
        rel={isExternalLink ? "noreferrer" : undefined}
        {...restProps}
      >
        {hash}
      </a>
      <Button
        variant="link"
        onClick={onCopy}
        className="relative text-white hover:text-[#a8f593]"
      >
        <Copy />
        {showTip && (
          <div className="absolute bottom-6 -left-6 px-3 py-2 text-xs font-normal text-white bg-black cursor-auto border border-[#FFFFFF80] capitalize">
            Copied
          </div>
        )}
      </Button>
    </div>
  );
};
