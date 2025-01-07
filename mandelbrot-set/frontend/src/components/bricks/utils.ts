import { LINES_COUNT, LINE_LENGHT, LINE_WIDTH, ROW_HEIGHT } from "./consts";
import { Direction, Rows, Section, StartPoint } from "./types";

function generateRandomPath(
  currentRow: number[],
  rowNumber: number,
  width: number
): {
  startPoint: { x: number; y: number };
  sections: Section[];
} {
  let startX: number;
  startX = Math.round(Math.random()) * width;

  const additionalOptionalRow = Math.round(Math.random());

  const startPoint = {
    x: startX,
    y: (rowNumber + additionalOptionalRow) * ROW_HEIGHT,
  };

  const sections: Section[] = [];
  let currentX = startPoint.x;
  let currentDirection: Direction;

  for (let i = 0; i < 3; i++) {
    const isHorizontal = i % 2 === 0;

    let size: number;

    if (isHorizontal) {
      if (i === 0) {
        currentDirection = startX === 0 ? "right" : "left";
        const rowStopper = Math.floor(Math.random() * (currentRow.length + 1));

        if (currentDirection === "right") {
          size = [...currentRow, width][rowStopper];
          currentX += size;
        } else {
          size = width - [0, ...currentRow][rowStopper];
          currentX -= size;
        }
      } else {
        if (
          currentX === 0 ||
          (Math.floor(Math.random()) && currentX !== width)
        ) {
          size = width - currentX;
          currentX += size;
          currentDirection = "right";
        } else {
          size = currentX;
          currentX -= size;
          currentDirection = "left";
        }
      }
    } else {
      currentDirection = additionalOptionalRow ? "up" : "down";
      size = ROW_HEIGHT;
    }

    sections.push({ size, direction: currentDirection });
  }

  return { startPoint, sections };
}

const drawLine = (
  ctx: CanvasRenderingContext2D,
  from: [number, number],
  to: [number, number]
) => {
  ctx.beginPath();
  ctx.moveTo(from[0], from[1]);
  ctx.lineTo(to[0], to[1]);
  ctx.stroke();
};

const drawSection = (
  ctx: CanvasRenderingContext2D,
  x_start: number,
  y_start: number,
  x_end: number,
  y_end: number,
  startOpacity: number,
  endOpacity: number
) => {
  const gradient = ctx.createLinearGradient(x_start, y_start, x_end, y_end);

  gradient.addColorStop(0, `rgba(168, 245, 147, ${startOpacity})`);
  gradient.addColorStop(0.5, "rgba(168, 245, 147, 1)");
  gradient.addColorStop(1, `rgba(168, 245, 147, ${endOpacity})`);

  ctx.strokeStyle = gradient;

  drawLine(ctx, [x_start, y_start], [x_end, y_end]);
};

const drawRows = (
  ctx: CanvasRenderingContext2D,
  rows: Rows,
  width: number,
  height: number
) => {
  const lineColor = "#FFFFFF0A";
  ctx.strokeStyle = lineColor;
  ctx.lineWidth = LINE_WIDTH;

  const drawVerticalBorders = () => {
    drawLine(ctx, [0, 0], [0, height]);
    drawLine(ctx, [width - 1, 0], [width - 1, height]);
  };

  const drawInnerVerticalLines = (
    row: number[],
    yStart: number,
    yEnd: number
  ) => {
    let xPosition = 0;
    row.forEach((offset) => {
      xPosition += offset;
      drawLine(ctx, [xPosition, yStart], [xPosition, yEnd]);
    });
  };

  const drawInnerHorizontalLines = (yStart: number, yEnd: number) => {
    drawLine(ctx, [0, yStart], [width, yStart]);
    drawLine(ctx, [0, yEnd], [width, yEnd]);
  };

  drawVerticalBorders();

  rows.forEach((row, index) => {
    const yStart = index * ROW_HEIGHT;
    const yEnd = yStart + ROW_HEIGHT;

    drawInnerHorizontalLines(yStart, yEnd);
    drawInnerVerticalLines(row, yStart, yEnd);
  });
};

const drawAnimatedLine = (
  ctx: CanvasRenderingContext2D,
  startPoint: StartPoint,
  sections: Section[],
  progress: number
) => {
  ctx.lineWidth = LINE_WIDTH;
  let currentX = startPoint.x;
  let currentY = startPoint.y;
  let traveledDistance = 0;

  sections.forEach((section) => {
    const sectionLength = section.size;

    if (
      progress >= traveledDistance &&
      progress < traveledDistance + sectionLength + LINE_LENGHT
    ) {
      const sectionProgress = progress - traveledDistance;

      const [dx, dy] = {
        left: [-1, 0],
        right: [1, 0],
        up: [0, -1],
        down: [0, 1],
      }[section.direction];

      const x_start =
        currentX + dx * Math.max(0, sectionProgress - LINE_LENGHT);
      const y_start =
        currentY + dy * Math.max(0, sectionProgress - LINE_LENGHT);
      const x_end = currentX + dx * Math.min(sectionLength, sectionProgress);
      const y_end = currentY + dy * Math.min(sectionLength, sectionProgress);

      const startOpacity = Math.max(
        0.2,
        1 - Math.abs(progress - traveledDistance) / LINE_LENGHT
      );
      const endOpacity = Math.max(
        0.2,
        1 -
          Math.abs(progress - (traveledDistance + sectionLength)) / LINE_LENGHT
      );

      drawSection(
        ctx,
        x_start,
        y_start,
        x_end,
        y_end,
        startOpacity,
        endOpacity
      );
    }

    currentX +=
      sectionLength * { left: -1, right: 1, up: 0, down: 0 }[section.direction];
    currentY +=
      sectionLength * { left: 0, right: 0, up: -1, down: 1 }[section.direction];
    traveledDistance += sectionLength;
  });
};

function generateNonAdjacentRows(totalRows: number, maxRows: number) {
  const count = Math.min(Math.floor((totalRows - 1) / 3) + 1, maxRows);
  const rows: number[] = [];

  while (rows.length < count) {
    const randomRow = Math.floor(Math.random() * totalRows);

    const isAdjacent = rows.some((row) => Math.abs(row - randomRow) === 1);

    if (!isAdjacent && !rows.includes(randomRow)) {
      rows.push(randomRow);
    }
  }

  return rows;
}

const generatePaths = (rows: Rows, width: number) => {
  const rowNumbers = generateNonAdjacentRows(rows.length, LINES_COUNT);
  return rowNumbers.map((rowNumber) =>
    generateRandomPath(rows[rowNumber], rowNumber, width)
  );
};

export {
  generateRandomPath,
  generateNonAdjacentRows,
  generatePaths,
  drawSection,
  drawRows,
  drawAnimatedLine,
};
