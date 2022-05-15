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
enum Direction {
    East(u8),
    West(u8),
    North(u8),
    South(u8),
}
//
/**
 * Rust 的对象定义和方法定义是分离的
 * 该例子定义了一个 Rectangle 结构体，并且在其上定义了一个 area 方法，用于计算该矩形的面积。
 * impl Rectangle {} 表示为 Rectangle 实现方法(impl 是实现 implementation 的缩写)，这样的写法表明 impl 语句块中的一切都是跟 Rectangle 相关联的。
 */
pub struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> bool {
        return self.width > 0;
    }
}
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height; //表达式属于返回值
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}
/**
 * T,U 是定义在结构体 Point 上的泛型参数，V,W 是单独定义在方法 mixup 上的泛型参数，它们并不冲突，说白了，你可以理解为，一个是结构体泛型，一个是函数泛型。
 */
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
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
    console_log!(
        "字符'中👨🏻🐱🐴为什么'占用了{}字节的内存大小",
        size_of_val(&x)
    );
    console_log!("add_with_extra{}", add_with_extra(12, 34));
    for c in "中国人".chars() {
        console_log!("{}", c);
    }
    let list: [i32; 5] = [1, 2, 3, 4, 5];
    for s in &list {
        console_log!("{}", &s);
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    console_log!("ifcondition {}", &number);
    // 枚举判断
    let dire = [
        Direction::South(5),
        Direction::North(5),
        Direction::East(2),
        Direction::West(5),
    ];
    match dire[3] {
        // Direction::West(5) 不存在
        Direction::East(2) => console_log!("East"),
        Direction::North(3) | Direction::South(5) => {
            console_log!("South or North");
        }
        _ => console_log!("West"),
    };
    let rect = Rectangle::new(30, 50);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    console_log!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    if rect.width() {
        console_log!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    console_log!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    console_log!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
/**
 * 因为特征只定义行为看起来是什么样的，因此我们需要为类型实现具体的特征，定义行为具体是怎么样的。特征很类似接口
 */
pub trait Summary {
    // fn summarize(&self) -> String;
    // 你可以在特征中定义具有默认实现的方法，这样其它类型无需再实现该方法，或者也可以选择重载该方法：
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}
impl Summary for Post {
    // 只实现 summarize_author 
    fn summarize_author(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}
pub struct Weibo {
    pub username: String,
    pub content: String
}
impl Summary for Weibo {
    // 从写类似接口方法
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
#[wasm_bindgen]
pub fn call_weibo(){
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    console_log!("{}",post.summarize());
    console_log!("{}",weibo.summarize());
}
#[wasm_bindgen]
pub async fn async_run_fetch(repo: String) -> Result<JsValue, JsValue> {
    let window = window().unwrap();
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
