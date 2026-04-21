import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { join, extname } from "node:path";
import { execFile } from "node:child_process";
import { writeFile, unlink } from "node:fs/promises";
import { randomUUID } from "node:crypto";
import { tmpdir } from "node:os";

const PUBLIC_DIRECTORY = join(import.meta.dirname, "public");
const PORT = 3000;

const CONTENT_TYPES = {
  ".html": "text/html",
  ".css": "text/css",
  ".js": "application/javascript",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".json": "application/json",
};

function serveStaticFile(response, filePath) {
  const extension = extname(filePath);
  const contentType = CONTENT_TYPES[extension] || "application/octet-stream";

  return readFile(filePath)
    .then((content) => {
      response.writeHead(200, { "Content-Type": contentType });
      response.end(content);
    })
    .catch(() => {
      response.writeHead(404);
      response.end("Not found");
    });
}

function handleRender(request, response) {
  let body = "";

  request.on("data", (chunk) => {
    body += chunk;
  });

  request.on("end", async () => {
    let sourceCode;
    try {
      const parsed = JSON.parse(body);
      sourceCode = parsed.source;
    } catch {
      response.writeHead(400, { "Content-Type": "application/json" });
      response.end(JSON.stringify({ error: "Invalid JSON" }));
      return;
    }

    if (!sourceCode) {
      response.writeHead(400, { "Content-Type": "application/json" });
      response.end(JSON.stringify({ error: "Missing source" }));
      return;
    }

    const identifier = randomUUID();
    const workingDirectory = tmpdir();
    const sourceFile = join(workingDirectory, `${identifier}.pix`);

    try {
      await writeFile(sourceFile, sourceCode);

      const outputFilename = await new Promise((resolve, reject) => {
        execFile(
          "pix",
          [sourceFile],
          { cwd: workingDirectory, timeout: 5000 },
          (error, stdout, stderr) => {
            if (error) {
              reject(new Error(stderr || error.message));
              return;
            }
            resolve(identifier);
          }
        );
      });

      const exportMatch = sourceCode.match(/export\s+"([^"]+)"\s+in\s+(\w+)/);
      if (!exportMatch) {
        response.writeHead(400, { "Content-Type": "application/json" });
        response.end(JSON.stringify({ error: "No export statement found" }));
        return;
      }

      const exportedName = exportMatch[1];
      const format = exportMatch[2];
      const formatExtensions = { png: "png", svg: "svg", webp: "webp", gif: "gif" };
      const formatContentTypes = {
        png: "image/png",
        svg: "image/svg+xml",
        webp: "image/webp",
        gif: "image/gif",
      };
      const extension = formatExtensions[format] || "png";
      const contentType = formatContentTypes[format] || "image/png";

      const imageFile = join(workingDirectory, `${exportedName}.${extension}`);
      const imageData = await readFile(imageFile);

      response.writeHead(200, { "Content-Type": contentType });
      response.end(imageData);

      await unlink(sourceFile).catch(() => {});
      await unlink(imageFile).catch(() => {});
    } catch (error) {
      response.writeHead(422, { "Content-Type": "application/json" });
      response.end(JSON.stringify({ error: error.message }));

      await unlink(sourceFile).catch(() => {});
    }
  });
}

function handleComplete(request, response) {
  let body = "";

  request.on("data", (chunk) => {
    body += chunk;
  });

  request.on("end", () => {
    const child = execFile(
      "pix-language-server",
      ["complete"],
      { timeout: 5000 },
      (error, stdout, stderr) => {
        if (error) {
          response.writeHead(500, { "Content-Type": "application/json" });
          response.end(JSON.stringify({ error: stderr || error.message }));
          return;
        }
        response.writeHead(200, { "Content-Type": "application/json" });
        response.end(stdout);
      }
    );
    child.stdin.write(body);
    child.stdin.end();
  });
}

const server = createServer((request, response) => {
  if (request.method === "POST" && request.url === "/render") {
    handleRender(request, response);
    return;
  }

  if (request.method === "POST" && request.url === "/complete") {
    handleComplete(request, response);
    return;
  }

  let filePath = join(PUBLIC_DIRECTORY, request.url === "/" ? "index.html" : request.url);
  serveStaticFile(response, filePath);
});

server.listen(PORT, () => {
  console.log(`Pix Playground running on http://localhost:${PORT}`);
});
