import clsx from "clsx";
import { ReactNode, useEffect, useRef, MouseEvent } from "react";

import CrossIcon from "@/assets/icons/cross.svg?react";
import { Button } from "@/components";

import styles from "./modal.module.scss";

type Props = {
  heading?: string;
  children: ReactNode;
  onClose: () => void;
  className?: string;
  closeOnBackdropClick?: boolean;
};

function Modal({
  heading,
  children,
  onClose,
  className,
  closeOnBackdropClick = false,
}: Props) {
  const ref = useRef<HTMLDialogElement>(null);

  const disableScroll = () => document.body.classList.add("modal-open");
  const enableScroll = () => document.body.classList.remove("modal-open");

  const open = () => {
    ref.current?.showModal();
    disableScroll();
  };

  const close = () => {
    ref.current?.close();
    enableScroll();
  };

  useEffect(() => {
    open();

    return () => close();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleClick = ({ target }: MouseEvent) => {
    const isBackdropClick = target === ref.current;

    if (isBackdropClick && closeOnBackdropClick) onClose();
  };

  return (
    <dialog
      ref={ref}
      onClick={handleClick}
      className={clsx(styles.dialog, className)}
    >
      <header className={styles.header}>
        <h2>{heading}</h2>

        <Button variant="link" size="icon" onClick={onClose}>
          <CrossIcon />
        </Button>
      </header>

      {children}
    </dialog>
  );
}

export { Modal };
