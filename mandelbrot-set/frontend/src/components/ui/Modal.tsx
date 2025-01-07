import { ReactNode, useEffect, useRef, MouseEvent } from "react";
import CrossIcon from "@/assets/icons/cross.svg?react";
import { Button } from "@/components";
import { cn } from "@/lib/utils";

type Props = {
  heading?: string;
  children: ReactNode;
  onClose: () => void;
  className?: string;
};

function Modal({ heading, children, onClose, className }: Props) {
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

    if (isBackdropClick) onClose();
  };

  return (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-noninteractive-element-interactions
    <dialog
      ref={ref}
      onClick={handleClick}
      className={cn(
        "flex justify-center items-center backdrop:backdrop-filter backdrop:backdrop-blur backdrop:bg-[#00000099]",
        className
      )}
    >
      <div className="p-4 w-[438px] bg-black border border-muted-foreground text-white">
        <header className="flex justify-between items-start mb-4 w-full">
          <h2>{heading}</h2>

          <Button variant="link" onClick={onClose}>
            <CrossIcon />
          </Button>
        </header>

        {children}
      </div>
    </dialog>
  );
}

export { Modal };
