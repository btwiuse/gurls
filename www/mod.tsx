/** @jsx h */
import { h, jsx, serve } from "https://deno.land/x/sift/mod.ts";
import { nanoid } from "https://cdn.esm.sh/v14/nanoid/esnext/nanoid.js";
import { GearApi } from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";

import { metaWasm } from "https://unpkg.com/gurls@0.1.2/dist/mod.ts";
import deploy from "https://unpkg.com/gurls@0.1.2/dist/deploy.json" assert { type: "json" };

let RPC_NODE = deploy.RPC_NODE;

let PROGRAM_ID = deploy.programId as `0x${string}`;

let PORT = Deno.env.get("PORT") || "8000";

serve(
  {
    "/": homePage,
    "/:code": handleCodeRequests,
  },
  { port: PORT }
);

let api = await GearApi.create({ providerAddress: RPC_NODE });

// Styles for the home page.
const style = css``;

// Script for the clipboard button.
const script = `

`;

/** The main function that responds to `/` route. */
async function homePage(request: Request) {
  let shortCode;

  // The input form makes a GET request with 'url' query
  // populated when someone submits the form. We use this
  // information to either create a new short link or get
  // an existing short link for the url.
  const { protocol, host, searchParams } = new URL(request.url);
  const url = searchParams.get("url");
  if (url) {
    let code = await findCode(url);
    if (code) {
      shortCode = code;
    } else {
      code = searchParams.get("shortCode") ?? nanoid(6);
      shortCode = (await addUrl(url, code)).code;
    }
  }

  return jsx(
    <html lang="en">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width" />
        <title>GURLS | Gear URL Shortener</title>
      </head>
      <script
        type="text/javascript"
        dangerouslySetInnerHTML={{ __html: script }}
      />
      <style dangerouslySetInnerHTML={{ __html: style }} />
      <script
        charSet="utf-8"
        src="https://unpkg.com/gurls@0.1.2/dist/gurls.js"
        type="module"
      />
    </html>
  );
}

/** Handle short link (`/<code>`) requests. */
async function handleCodeRequests(req: Request) {
  let url = new URL(req.url);
  const code = decodeURI(url.pathname.replace(/^\//, "")) ?? "";
  if (code.length > 0) {
    const url = await findUrl(code);
    if (url) {
      return Response.redirect(url, 302);
    }
  }

  return jsx(<html>url not found</html>, { status: 404 });
}

/** Cache the code as key and url as value. */
const codeCache = new Map<string, string>();
/** Cache the url as key and code as value. */
const urlCache = new Map<string, string>();

/** Find url for the provided url. */
async function findUrl(code: string): Promise<string | null> {
  if (codeCache.has(code)) {
    return codeCache.get(code);
  }
  // TODO: read contract state
  const query = { Code: code };
  let result = await api.programState.read(PROGRAM_ID, metaWasm, query);
  let maybeUrl = (result.toJSON() as any).maybeUrl;
  console.log({ code, maybeUrl });
  return maybeUrl;
}

/** Find short code for the provided url. */
async function findCode(url: string): Promise<string | undefined> {
  if (urlCache.has(url)) {
    return urlCache.get(url);
  }

  return undefined;
}

/** Create a new link with the provided url and code.
 * Also populate the cache. */
async function addUrl(
  url: string,
  code: string
): Promise<{ code: string; url: string }> {
  let link = {
    url,
    code,
  };
  console.log(link);

  // TODO: write contract state
  codeCache.set(code, url);
  urlCache.set(url, code);
  return link;
}

/** Wrapper function to get syntax highlight for CSS in editors. */
function css(style: TemplateStringsArray) {
  return style.join("");
}
