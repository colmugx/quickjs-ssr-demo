# QuickJS SSR

中文文章：[《我早就想拿 QuickJS 做 SSR 了》](https://colmugx.github.io/blog/2022/12/12/quickjs-ssr/)

## Usage
```shell
# build frontend
cd ssr
pnpm i
pnpm build && pnpm build:server

# build backend and run
cd ..
cargo run
```

then open `http://localhost:8080` to view it in the browser.

## Q&A

### unsupported keyword: export

delete the last line in `ssr/dist/server/entry-server.js`
```diff
- export { render }
```