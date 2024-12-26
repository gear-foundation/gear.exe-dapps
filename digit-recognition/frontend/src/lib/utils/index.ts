export { mergeRefs } from "./merge-refs";
export { isMobileDevice, isMobile } from "./device-detection";

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
