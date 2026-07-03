const dir = import.meta.dir;
const port = process.env.PORT ? Number(process.env.PORT) : 8070;

const MIME = {
  '.html': 'text/html',
  '.js':   'application/javascript',
  '.wasm': 'application/wasm',
  '.md':   'text/plain',
  '.svg':  'image/svg+xml',
};

console.log(`newweb → http://localhost:${port}`);

Bun.serve({
  port,
  hostname: '0.0.0.0',
  async fetch(req) {
    const url = new URL(req.url);
    const pathname = url.pathname === '/' ? '/index.html' : url.pathname;
    const file = Bun.file(`${dir}${pathname}`);

    if (await file.exists()) {
      const ext = pathname.slice(pathname.lastIndexOf('.'));
      const contentType = MIME[ext];
      return new Response(file, contentType ? { headers: { 'Content-Type': contentType } } : undefined);
    }

    return new Response('Not Found', { status: 404 });
  },
});
