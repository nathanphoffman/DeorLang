const port = process.env.PORT ? Number(process.env.PORT) : 3000;
console.log(`Server running at http://localhost:${port}`);

Bun.serve({
    port,
    hostname: "0.0.0.0",
    async fetch(req) {
        const url = new URL(req.url);
        const filePath = url.pathname === "/" ? "/index.html" : url.pathname;

        const file = Bun.file(`${import.meta.dir}/public${filePath}`);

        if (await file.exists()) {
            return new Response(file);
        }

        return new Response("Not Found", { status: 404 });
    },
});