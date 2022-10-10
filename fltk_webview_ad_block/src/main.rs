extern crate fltk;
extern crate tinyjson;

use fltk::{app, prelude::*, window};
use fltk_webview::Webview;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = match args.len() {
        1 => "".to_string(),
        _ => args[1].to_string(),
    };
    let uri = match arg1.trim() {
        "google" => "https://google.com/",
        "news" => "https://news.google.com/",
        _ => "https://news.google.com/",
    };


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
    wv.init(
        r#"
console.log('init');
function main_loop() {
    var idList = ['ad'];
    for(var i = 0; i < idList.length; i++) {
        const elm = document.getElementById(idList[i]);
        if (!!elm) elm.parentNode.removeChild(elm);
    }
    var classList = ['ad_overlay',
                     'ad_rectangle',
                     'ad_list_top',
                     'ad_topics_custom',
                     'ad_custom'];
    for(var i = 0; i < classList.length; i++) {
        const elmList = document.getElementsByClassName(classList[i]);
        for(var j = 0; j < elmList.length; j++) {
            elmList[j].parentNode.removeChild(elmList[j]);
        }
    }
    var selectorList = ['data-google-query-id', 'role="dialog"'];
    for(var i = 0; i < selectorList.length; i++) {
        const elmList = document.querySelectorAll('[' + selectorList[i] + ']');
        for(var j = 0; j < elmList.length; j++) {
            elmList[j].parentNode.removeChild(elmList[j]);
        }
    }
    document.getElementsByTagName('html')[0].style.overflow = "auto";
    setTimeout(main_loop, 1000);
}
main_loop();
        "#,
        );

    wv.navigate(uri);
    app.run().unwrap();
}
