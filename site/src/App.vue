<template>
    <canvas ref="canvasElement"></canvas>
</template>
<script lang="ts">
import { ref, defineComponent } from 'vue'
import init, { draw_julia, async_run_fetch, greet, add, call_weibo } from "../pkg/rustWasmTest";
export default defineComponent(
    {
        setup() {
            const canvasElement = ref<HTMLCanvasElement | undefined>();
            return {
                canvasElement
            }
        },
        async mounted() {
            const ratio = window.devicePixelRatio;
            const darwText = (canvas: HTMLCanvasElement) => {
                const ctx: CanvasRenderingContext2D | null = canvas.getContext("2d");
                if (ctx) {
                    canvas.width = canvas.offsetWidth * ratio;
                    canvas.height = canvas.offsetHeight * ratio;
                    ctx.scale(ratio, ratio);
                    ctx.font = "18px serif";
                    ctx.fillText(`Rust操作cavas对象，调用add计算 ${add(21, 34)}`, 10, 50);
                    setTimeout(() => {
                        draw_julia(ctx, canvas.width, canvas.height, -0.15, 0.65);
                    }, 1000);
                }
            }

            const run_rust_async_fetch = () => {
                async_run_fetch("ningxiao/rustwasm").then((data) => {
                    console.log(data);
                    console.log("The latest commit to the demo %s branch is:", data.name);
                    console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
                });
            }
            await init();
            const result = add(12, 25);
            console.log(`12 + 25 = ${result}`);
            if (result !== 37) {
                throw new Error("wasm addition doesn't work!");
            }
            greet('World RustWasm');
            if (this.canvasElement) {
                darwText(this.canvasElement);
            }
            run_rust_async_fetch();
            call_weibo();
        }
    }
)
</script>
