import { Card } from "@/components";

export const CatIdentifier = () => {
  return (
    <Card
      title="Cat identifier"
      description="Upload any image to see if the AI detects a cat. The model will analyze the picture and let you know if a cat is present—true for cats, false for no cats."
      canvasSlot={
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            height: "100%",
          }}
        >
          Comming soon...
        </div>
      }
    />
  );
};
