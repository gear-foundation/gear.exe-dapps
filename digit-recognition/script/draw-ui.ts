import http from "node:http";
import type { Socket } from "node:net";
import { spawn } from "node:child_process";

export type DrawUiOptions = {
  /**
   * If not set (or 0) - OS will pick a free port automatically.
   */
  port?: number;
  openBrowser?: boolean;
};

type SubmitPayload = {
  width: number;
  height: number;
  pixels: number[];
};

function openUrl(url: string) {
  const platform = process.platform;

  if (platform === "darwin") {
    spawn("open", [url], { stdio: "ignore", detached: true }).unref();
    return;
  }
  if (platform === "win32") {
    spawn("cmd", ["/c", "start", "", url], {
      stdio: "ignore",
      detached: true,
    }).unref();
    return;
  }

  spawn("xdg-open", [url], { stdio: "ignore", detached: true }).unref();
}

/**
 * Opens a local draw UI (500x500 -> 28x28), waits for "Send",
 * returns pixels as array of 784 ints in range 0..255.
 */
export async function drawMnist28x28(
  opts: DrawUiOptions = {},
): Promise<number[]> {
  const port = opts.port ?? 0; 
  const shouldOpen = opts.openBrowser ?? true;

  return new Promise((resolve, reject) => {
    let resolved = false;
    const sockets = new Set<Socket>();

    const server = http.createServer(async (req, res) => {
      try {
        const url = new URL(req.url ?? "/", "http://localhost");

        // UI
        if (req.method === "GET" && url.pathname === "/") {
          res.writeHead(200, {
            "Content-Type": "text/html; charset=utf-8",
            Connection: "close",
          });
          res.end(getHtmlPage());
          return;
        }

        // Submit pixels
        if (req.method === "POST" && url.pathname === "/api/submit") {
          const chunks: Buffer[] = [];
          for await (const c of req) chunks.push(c as Buffer);

          const body = Buffer.concat(chunks).toString("utf8");
          const parsed = JSON.parse(body) as SubmitPayload;

          if (parsed.width !== 28 || parsed.height !== 28) {
            throw new Error(
              `Expected 28x28, got ${parsed.width}x${parsed.height}`,
            );
          }
          if (!Array.isArray(parsed.pixels) || parsed.pixels.length !== 784) {
            throw new Error("pixels must be an array of length 784");
          }

          for (let i = 0; i < parsed.pixels.length; i++) {
            const v = parsed.pixels[i];
            if (!Number.isInteger(v) || v < 0 || v > 255) {
              throw new Error(
                `Bad pixel[${i}]=${v} (expected integer 0..255)`,
              );
            }
          }

          res.writeHead(200, {
            "Content-Type": "application/json",
            Connection: "close",
          });
          res.end(JSON.stringify({ ok: true }));

          if (!resolved) {
            resolved = true;
            resolve(parsed.pixels);

            // Close server and all sockets after response is flushed
            setImmediate(() => {
              server.close();
              for (const s of sockets) s.destroy();
            });
          }
          return;
        }

        res.writeHead(404, { "Content-Type": "text/plain", Connection: "close" });
        res.end("Not found");
      } catch (e: any) {
        res.writeHead(500, {
          "Content-Type": "application/json",
          Connection: "close",
        });
        res.end(JSON.stringify({ ok: false, error: String(e?.message ?? e) }));
      }
    });

    // Track sockets for force closing keep-alive connections
    server.on("connection", (socket) => {
      sockets.add(socket);
      socket.on("close", () => sockets.delete(socket));
    });

    server.on("error", reject);

    server.listen(port, () => {
      const actualPort = (server.address() as any).port as number;
      const url = `http://localhost:${actualPort}`;

      console.log(`Draw UI is running: ${url}`);
      if (shouldOpen) openUrl(url);
    });
  });
}

function getHtmlPage(): string {
  return `<!doctype html>
<html>
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title>MNIST Draw</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 24px; }
    .row { display:flex; gap:24px; align-items:flex-start; }
    canvas { border:1px solid #ccc; touch-action:none; }
    .controls { display:flex; flex-direction:column; gap:12px; min-width: 320px; }
    label { display:flex; gap:8px; align-items:center; }
    input[type=range]{ width:220px; }
    button { padding: 8px 12px; }
    .status { padding: 8px; background:#f6f6f6; border-radius:8px; white-space:pre-wrap; }
  </style>
</head>
<body>
  <h2>Draw digit (500×500 → 28×28)</h2>
  <div class="row">
    <canvas id="canvas" width="500" height="500"></canvas>
    <div class="controls">
      <label>
        Brush size:
        <input id="brushSize" type="range" min="1" max="60" value="15" />
        <span id="brushSizeVal">15</span>
      </label>

      <label>
        Intensity:
        <input id="intensity" type="range" min="1" max="255" value="128" />
        <span id="intensityVal">128</span>
      </label>

      <button id="clear">Clear</button>
      <button id="send">Send to contract</button>

      <div class="status" id="status">Draw something and press "Send to contract".</div>
    </div>
  </div>

<script type="module">
  const CANVAS_W = 500;
  const CANVAS_H = 500;
  const LOW_W = 28;
  const LOW_H = 28;

  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d", { willReadFrequently: true });

  const brushSizeEl = document.getElementById("brushSize");
  const intensityEl = document.getElementById("intensity");
  const brushSizeVal = document.getElementById("brushSizeVal");
  const intensityVal = document.getElementById("intensityVal");
  const clearBtn = document.getElementById("clear");
  const sendBtn = document.getElementById("send");
  const statusEl = document.getElementById("status");

  let brushSize = Number(brushSizeEl.value);
  let intensity = Number(intensityEl.value);
  let drawing = false;
  let lastPos = null;

  function setStatus(t) { statusEl.textContent = t; }

  function clearCanvas() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, CANVAS_W, CANVAS_H);
  }

  function getCanvasPoint(e) {
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    return {
      x: (e.clientX - rect.left) * scaleX,
      y: (e.clientY - rect.top) * scaleY,
    };
  }

  function applyBrush(p) {
    const alpha = intensity / 255;
    ctx.save();
    ctx.fillStyle = \`rgba(255,255,255,\${alpha})\`;
    ctx.beginPath();
    ctx.arc(p.x, p.y, brushSize / 2, 0, Math.PI * 2);
    ctx.fill();
    ctx.restore();
  }

  function downscaleTo28x28() {
    const off = document.createElement("canvas");
    off.width = LOW_W;
    off.height = LOW_H;

    const octx = off.getContext("2d", { willReadFrequently: true });
    octx.imageSmoothingEnabled = true;
    octx.drawImage(canvas, 0, 0, LOW_W, LOW_H);

    const img = octx.getImageData(0, 0, LOW_W, LOW_H).data;
    const pixels = new Array(LOW_W * LOW_H);

    for (let i = 0; i < LOW_W * LOW_H; i++) {
      const r = img[i * 4 + 0];
      const g = img[i * 4 + 1];
      const b = img[i * 4 + 2];
      const gray = Math.round(0.299*r + 0.587*g + 0.114*b);
      pixels[i] = gray;
    }
    return pixels;
  }

  brushSizeEl.addEventListener("input", () => {
    brushSize = Number(brushSizeEl.value);
    brushSizeVal.textContent = String(brushSize);
  });

  intensityEl.addEventListener("input", () => {
    intensity = Number(intensityEl.value);
    intensityVal.textContent = String(intensity);
  });

  clearBtn.addEventListener("click", () => {
    clearCanvas();
    setStatus("Cleared.");
  });

  canvas.addEventListener("pointerdown", (e) => {
    drawing = true;
    canvas.setPointerCapture(e.pointerId);
    const p = getCanvasPoint(e);
    applyBrush(p);
    lastPos = p;
  });

  canvas.addEventListener("pointermove", (e) => {
    if (!drawing) return;
    const p = getCanvasPoint(e);

    if (lastPos) {
      const dx = p.x - lastPos.x;
      const dy = p.y - lastPos.y;
      const steps = Math.max(Math.abs(dx), Math.abs(dy));

      if (steps > 0) {
        for (let s = 0; s <= steps; s++) {
          const ix = lastPos.x + (dx * s) / steps;
          const iy = lastPos.y + (dy * s) / steps;
          applyBrush({ x: ix, y: iy });
        }
      }
    }

    lastPos = p;
  });

  canvas.addEventListener("pointerup", () => {
    drawing = false;
    lastPos = null;
  });
  canvas.addEventListener("pointerleave", () => {
    drawing = false;
    lastPos = null;
  });

  sendBtn.addEventListener("click", async () => {
    try {
      // ✅ prevent double click / multiple sends
      sendBtn.disabled = true;

      setStatus("Downscaling to 28x28...");
      const pixels = downscaleTo28x28();

      setStatus("Sending pixels to backend...");
      const resp = await fetch("/api/submit", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ width: 28, height: 28, pixels }),
      });

      const json = await resp.json();
      if (!json.ok) throw new Error(json.error || "submit failed");

      setStatus("Submitted. You can close this tab now.");
    } catch (e) {
      setStatus("Error: " + (e?.message ?? String(e)));
      sendBtn.disabled = false;
    }
  });

  clearCanvas();
</script>
</body>
</html>`;
}