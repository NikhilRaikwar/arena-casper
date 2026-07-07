import express from "express";
import cors from "cors";
import { createArenaClient } from "../agents/shared/arena-client.js";
import { loadConfig } from "../agents/shared/config.js";

const config = loadConfig();
const client = createArenaClient(config);
const app = express();
const port = Number(process.env.SPECTATOR_PORT ?? 3001);

app.use(cors());
app.use(express.static("spectator/public"));

app.get("/api/match", async (_req, res) => {
  res.json(await client.getLatestMatch());
});

app.get("/api/events", async (_req, res) => {
  res.json(await client.listEvents(0));
});

app.get("/events", async (req, res) => {
  res.writeHead(200, {
    "Content-Type": "text/event-stream",
    "Cache-Control": "no-cache",
    Connection: "keep-alive"
  });

  let cursor = 0;
  const send = async () => {
    const events = await client.listEvents(cursor);
    for (const event of events) {
      res.write(`event: ${event.type}\n`);
      res.write(`data: ${JSON.stringify(event)}\n\n`);
    }
    cursor += events.length;
  };

  await send();
  const timer = setInterval(() => {
    send().catch((error) => {
      res.write(`event: error\n`);
      res.write(`data: ${JSON.stringify({ message: error.message })}\n\n`);
    });
  }, 1_000);

  req.on("close", () => clearInterval(timer));
});

app.listen(port, () => {
  console.log(`Arena spectator running at http://localhost:${port}`);
});

export default app;
