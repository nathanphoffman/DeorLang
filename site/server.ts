import { statSync } from 'fs';
import { extname, join, normalize } from 'path';

const dir = join(import.meta.dir, 'public');
const PORT = process.env.PORT ? Number(process.env.PORT) : 8080;

const MIME = {
  '.html': 'text/html',
  '.js':   'application/javascript',
  '.wasm': 'application/wasm',
  '.md':   'text/plain',
  '.css':  'text/css',
  '.svg':  'image/svg+xml',
  '.png':  'image/png',
  '.jpg':  'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.gif':  'image/gif',
  '.webp': 'image/webp',
};

function fileType(path) {
  try {
    const stat = statSync(path);
    return stat.isFile() ? 'file' : stat.isDirectory() ? 'dir' : null;
  } catch {
    return null;
  }
}

function serveFile(path) {
  return new Response(Bun.file(path), {
    headers: {
      'Content-Type': MIME[extname(path)] || 'application/octet-stream',
      'Cache-Control': 'no-store',
    },
  });
}

const server = Bun.serve({
  port: PORT,
  hostname: '0.0.0.0',
  fetch(req) {
    const url = new URL(req.url);
    let pathname = decodeURIComponent(url.pathname);
    if (pathname.length > 1 && pathname.endsWith('/')) pathname = pathname.slice(0, -1);

    // resolve within `dir` only — reject any path that escapes it (e.g. "..")
    const filePath = normalize(join(dir, pathname));
    if (filePath !== dir && !filePath.startsWith(dir + '/')) {
      return new Response('not found', { status: 404 });
    }

    if (pathname === '/') return serveFile(join(dir, 'index.html'));

    // real file on disk — serve as-is
    if (fileType(filePath) === 'file') return serveFile(filePath);

    // folder with its own index.html — e.g. /pico -> public/pico/index.html
    if (fileType(join(filePath, 'index.html')) === 'file') {
      return serveFile(join(filePath, 'index.html'));
    }

    return new Response('not found', { status: 404 });
  },
});

console.log(`newweb → ${server.url}`);