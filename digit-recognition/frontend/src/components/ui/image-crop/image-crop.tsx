import React, { useState, useCallback } from "react";
import Cropper, { Area } from "react-easy-crop";

import { getCroppedImg } from "@/lib/utils";

import styles from "./image-crop.module.scss";
import { Button } from "../button/Button";

type Props = {
  image: string;
  onClose: () => void;
  onCropComplete: (croppedImage: Blob) => void;
};

const ImageCrop: React.FC<Props> = ({ image, onClose, onCropComplete }) => {
  const [crop, setCrop] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(1);
  const [croppedAreaPixels, setCroppedAreaPixels] = useState<Area | null>(null);

  const onCropDone = async () => {
    if (!croppedAreaPixels) return;
    const croppedImage = await getCroppedImg(image, croppedAreaPixels);
    onCropComplete(croppedImage);
    onClose();
  };

  const onCropCompleteHandler = useCallback(
    (_croppedArea: Area, croppedPixels: Area) => {
      setCroppedAreaPixels(croppedPixels);
    },
    []
  );

  return (
    <div className={styles.cropModal}>
      <div className={styles.cropContainer}>
        <Cropper
          image={image}
          crop={crop}
          zoom={zoom}
          aspect={1}
          onCropChange={setCrop}
          onZoomChange={setZoom}
          onCropComplete={onCropCompleteHandler}
          
        />
      </div>
      <div className={styles.cropControls}>
        <Button variant="outline" onClick={onCropDone}>
          CROP
        </Button>
      </div>
    </div>
  );
};

export { ImageCrop };
