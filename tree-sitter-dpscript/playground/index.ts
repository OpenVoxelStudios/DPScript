import { Application, Router } from "@oak/oak";
import * as colors from "@std/fmt/colors";
import * as path from "@std/path";

const root = import.meta.dirname!;
const app = new Application();
const router = new Router();

app.use(async (ctx, next) => {
    await next();

    const c = ctx.response.status >= 500 ? colors.red : ctx.response.status >= 400 ? colors.yellow : colors.green;

    console.log(
        `${c(ctx.request.method)} ${c(`(${ctx.response.status})`)} - ${
            colors.cyan(
                `${ctx.request.url.pathname}${ctx.request.url.search}`,
            )
        }`,
    );
});

router.get("/", (cx) => cx.send({ root, index: "index.html" }));
router.get("/playground.js", (cx) => cx.send({ root }));
router.get("/tree-sitter-dpscript.wasm", (cx) => cx.send({ root: path.dirname(root) }));

app.use(router.routes());
app.use(router.allowedMethods());

console.log(colors.brightBlue("Server running on http://localhost:8000"));

app.listen({ port: 8000 });
