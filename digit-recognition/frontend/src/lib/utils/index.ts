import { FixedPoint } from "../types";
export { isMobileDevice, isMobile } from "./device-detection";
export { retryWhileDataChanged } from "./retry-while-data-changed";

export const copyToClipboard = async ({
  value,
  onSuccess,
  onError,
}: {
  value: string;
  onSuccess?: () => void;
  onError?: () => void;
}) => {
  function unsecuredCopyToClipboard(text: string) {
    const textArea = document.createElement("textarea");
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
      document.execCommand("copy");
      onSuccess?.();
    } catch (err) {
      console.error("Unable to copy to clipboard", err);
      onError?.();
    }
    document.body.removeChild(textArea);
  }

  if (window.isSecureContext && navigator.clipboard) {
    navigator.clipboard
      .writeText(value)
      .then(() => onSuccess?.())
      .catch(() => onError?.());
  } else {
    unsecuredCopyToClipboard(value);
  }
};

export const getFloatingPoint = ({ num, scale }: FixedPoint) => {
  return Number(Number(num) / Math.pow(10, Number(scale)));
};
