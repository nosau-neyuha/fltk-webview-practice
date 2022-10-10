extern crate fltk;

mod data_scraper;
mod file_handler;

use fltk::{app, prelude::*, window};
use fltk_webview::Webview;
use rust_embed::RustEmbed;
use serde_json::Value;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use urlencoding::encode;

#[derive(RustEmbed)]
#[folder = "src/static"]
struct Asset;

fn main() {
    let raw_html = Asset::get("index.html").expect("Error: asset index.html");
    let str_html: &str = std::str::from_utf8(raw_html.data.as_ref()).expect("Error: str index.html");
    let encoded_html = encode(str_html);

    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.make_resizable(true);
    win.show();

    let wv = Webview::create(true, &mut wv_win);
    wv.bind("searchItemList", |_seq, content| {
        let v: Value = serde_json::from_str(content).unwrap();
        let val: String = v[0].as_str().unwrap().to_string();

        let (tx, rx) = oneshot::channel();
        let rt = rt();
        rt.spawn(async move {
            let data = data_scraper::search(val.trim()).await;
            tx.send(data)
        });
        let out = rt.block_on(async { rx.await }).unwrap();
        wv.eval(&format!("OnSearchData('{}');", out.trim()));
    });
    wv.bind("setItemList", |_seq, content| {
        let v: Value = serde_json::from_str(content).unwrap();
        let val: String = v[0].as_str().unwrap().to_string();

        let rt = rt();
        rt.block_on(async move {
            let _ = file_handler::data_save(val.trim()).await;
        });
    });
    wv.bind("getItemList", |_seq, _content| {
        let (tx, rx) = oneshot::channel();
        let rt = rt();
        rt.spawn(async move {
            let data = file_handler::data_load().await;
            tx.send(data)
        });
        let out = rt.block_on(async { rx.await }).unwrap();
        wv.eval(&format!("OnLoadData('{}');", out.trim()));
    });

    wv.navigate(&format!("data:text/html,{}", encoded_html));
    app.run().unwrap();
}

fn rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}
