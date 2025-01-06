export const ROW_HEIGHT = 80;
export const LINE_LENGHT = 200;
export const LINE_WIDTH = 1;
export const LINES_COUNT = 3;
export const ANIMATION_SPEED = 2;

const rows1 = [
  [160],
  [],
  [107],
  [],
  [76, 113],
  [154],
  [107],
  [],
  [160],
  [107],
  [],
  [76, 113],
];

const rows2 = [
  [75],
  [128],
  [],
  [113, 47],
  [],
  [128],
  [154],
  [],
  [160],
  [76, 47],
  [],
  [128],
];

export const leftRows = [...rows1, ...rows2];
export const rightRows = [...rows2, ...rows1];
