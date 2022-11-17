import esbuild from "esbuild";
import { dtsPlugin } from "esbuild-plugin-d.ts";
import serve, { error, log } from "@btwiuse/serve";
import { NodeGlobalsPolyfillPlugin } from "@esbuild-plugins/node-globals-polyfill";
import { NodeModulesPolyfillPlugin } from "@esbuild-plugins/node-modules-polyfill";
import plugin from "node-stdlib-browser/helpers/esbuild/plugin";
import stdLibBrowser from "node-stdlib-browser";

const isDevServer = process.argv.includes("--dev");

esbuild
  .build({
    entryPoints: ["gurls.ts"],
    bundle: true,
    // platform: "node",
    // format: "esm",
    // keepNames: true,
    // minify: false,
    outfile: "dist/gurls.js",
    loader: {
      ".svg": "dataurl",
      ".wasm": "binary",
    },
    minify: !isDevServer,
    sourcemap: false,
    incremental: isDevServer,
    target: ["esnext"],
    define: {
      "process.env.NODE_ENV": isDevServer ? '"development"' : '"production"',
    },
    watch: isDevServer && {
      onRebuild(err) {
        serve.update();
        err ? error("❌ Failed") : log("✅ Updated");
      },
    },
    plugins: [
      // dtsPlugin(),
      plugin(stdLibBrowser),
      NodeGlobalsPolyfillPlugin({
        fs: true,
        process: true,
        buffer: true,
      }),
      NodeModulesPolyfillPlugin(),
    ],
  })
  .catch(() => process.exit(1));
