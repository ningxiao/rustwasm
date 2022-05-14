mod utils;
use serde::{Deserialize, Serialize};
use std::mem::size_of_val;
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console, window, CanvasRenderingContext2d, ImageData, Request, RequestInit, RequestMode,
    Response,
};
/**
 * When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
 * https://rustwasm.github.io/docs/wasm-pack/commands/build.html
 *  https://rustwasm.github.io/docs/wasm-bindgen/
 */
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}
#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}
impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }
    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen(module = "/site/src/utils/defined-in-js.js")]
extern "C" {
    fn name() -> String;

    type MyClass;

    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;

    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;

    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
}
fn run_wasm_bindgen_js() {
    let x = MyClass::new();
    assert_eq!(x.number(), 42);
    x.set_number(10);
    log(&x.render());
}
fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = ((amt as u32) % 1_000) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}
fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();
    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.0025;
    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }
    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    let y2 = {
        // 表达式
        let x = 3;
        x + 1
    };
    x + y + y2 // 表达式
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
    console::log_1(&"Hello using web-sys".into());
    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
    console_log!("1 + 3 = {}", 1 + 3);
    run_wasm_bindgen_js();
    let (a, mut b): (u64, u64) = (12, 13);
    // a = true,不可变; b = false，可变
    console_log!("a = {}, b = {}", a, b);
    b = 42_u64;
    console_log!("a = {}, b = {}", a, b);
    for i in 1..=5 {
        console_log!("{}", i);
    }
    let x = "中👨🏻🐱🐴为什么";
    console_log!("字符'中👨🏻🐱🐴为什么'占用了{}字节的内存大小", size_of_val(&x));
    console_log!("add_with_extra{}", add_with_extra(12, 34));
    for c in "中国人".chars() {
        console_log!("{}", c);
    }
    for s in "中国人".bytes() {
        console_log!("{}", s);
    }
}
#[wasm_bindgen]
pub async fn async_run_fetch(repo: String) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = format!("https://api.github.com/repos/{}/branches/master", repo);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` 是一个 `Response` 对象。
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // 将这另一个 `Promise` 转换成 rust `Future`。
    let json = JsFuture::from(resp.json()?).await?;
    // 使用 serde 将 JSON 解析为结构。
    let branch_info: Branch = json.into_serde().unwrap();
    // 将 `Branch` 结构体作为 `Object` 发送回 JS。
    Ok(JsValue::from_serde(&branch_info).unwrap())
}
#[wasm_bindgen]
pub fn draw_julia(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    console::time_with_label("Running draw_julia");
    let c = Complex { real, imaginary };
    let mut data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    console_log!("The value of y is: x{},y{},z{}", x, y, z);
    console::time_end_with_label("Running draw_julia");
    ctx.put_image_data(&data, 0.0, 0.0)
}
// 加载完成立即执行
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let performance = window
        .performance()
        .expect("performance should be available");
    console_log!("the current time (in ms) is {}", performance.now());

    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let el = document.create_element("canvas")?;
    el.set_id("canvas");
    body.append_child(&el)?;
    let start = perf_to_system(performance.timing().request_start());
    let end = perf_to_system(performance.timing().response_end());

    console_log!("request started at {}", humantime::format_rfc3339(start));
    console_log!("request ended at {}", humantime::format_rfc3339(end));
    Ok(())
}
