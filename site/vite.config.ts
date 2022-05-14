import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import importToCDN from 'vite-plugin-cdn-import';
// https://vitejs.dev/config/
export default defineConfig(
    {
        plugins: [vue(),
        importToCDN({
            modules: [
                {
                    var: 'Vue',
                    name: 'vue',
                    path: `https://unpkg.com/vue@3.2.19/dist/vue.runtime.global.prod.js`,
                },
            ]
        })],
        server: {
            open: true,
            port: 8080
        },
        build: {
            target: "esnext",
            brotliSize: false, /* 压缩大型输出文件可能会很慢，因此禁用该功能可能会提高大型项目的构建性能 */
            outDir: 'dist', /* 指定输出路径 */
            cssCodeSplit: false, /* 整个项目中的所有 CSS 将被提取到一个 CSS 文件中 */
            chunkSizeWarningLimit: 1500, /* chunk 大小警告的限制（以 kbs 为单位） */
            sourcemap: false, /* 构建后是否生成 source map 文件 */
            assetsDir: 'static/images/', /* 指定生成静态资源的存放路径 */
            emptyOutDir: true, /* 默认情况下，若 outDir 在 root 目录下，则 Vite 会在构建时清空该目录 */
            rollupOptions: {
                output: {
                    chunkFileNames: 'js/[name].[hash].js',
                    entryFileNames: 'js/[name].[hash].js',
                    assetFileNames: '[ext]/[name].[hash].[ext]',
                },
            }
        }
    }
)
