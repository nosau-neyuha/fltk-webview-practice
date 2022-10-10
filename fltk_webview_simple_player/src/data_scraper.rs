use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSet {
    pub id: String,
    pub title: String,
    pub length: String,
}

struct DataAccessStrSet<'a> {
    base_access_str: &'a str,
    id_access_str: &'a str,
    title_access_str: &'a str,
    length_access_str: &'a str,
}

const CHANNEL_STR_SET: DataAccessStrSet<'static> = DataAccessStrSet {
    base_access_str: "contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.1.itemSectionRenderer.contents.0.shelfRenderer.content.horizontalListRenderer.items",
    id_access_str: "gridVideoRenderer.videoId",
    title_access_str: "gridVideoRenderer.title.simpleText",
    length_access_str: "gridVideoRenderer.thumbnailOverlays.0.thumbnailOverlayTimeStatusRenderer.text.simpleText",
};

const TREND_STR_SET: DataAccessStrSet<'static> = DataAccessStrSet {
    base_access_str: "contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.0.itemSectionRenderer.contents.0.shelfRenderer.content.expandedShelfContentsRenderer.items",
    id_access_str: "videoRenderer.videoId",
    title_access_str: "videoRenderer.title.runs.0.text",
    length_access_str: "videoRenderer.lengthText.simpleText",
};

const PLAYLIST_STR_SET: DataAccessStrSet<'static> = DataAccessStrSet {
    base_access_str: "contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.sectionListRenderer.contents.0.itemSectionRenderer.contents.0.playlistVideoListRenderer.contents",
    id_access_str: "playlistVideoRenderer.videoId",
    title_access_str: "playlistVideoRenderer.title.runs.0.text",
    length_access_str: "playlistVideoRenderer.lengthText.simpleText",
};

const SEARCH_STR_SET: DataAccessStrSet<'static> = DataAccessStrSet {
    base_access_str: "contents.twoColumnSearchResultsRenderer.primaryContents.sectionListRenderer.contents.0.itemSectionRenderer.contents",
    id_access_str: "videoRenderer.videoId",
    title_access_str: "videoRenderer.title.runs.0.text",
    length_access_str: "videoRenderer.lengthText.simpleText",
};

const TOP_STR_SET: DataAccessStrSet<'static> = DataAccessStrSet {
    base_access_str: "contents.twoColumnBrowseResultsRenderer.tabs.0.tabRenderer.content.richGridRenderer.contents",
    id_access_str: "richItemRenderer.content.videoRenderer.videoId",
    title_access_str: "richItemRenderer.content.videoRenderer.title.runs.0.text",
    length_access_str: "richItemRenderer.content.videoRenderer.lengthText.simpleText",
};

pub async fn search(param: &str) -> String {
    let raw_uri = param.to_string();
    if raw_uri.as_str().is_empty() {
        eprint!("Error: empty");
        return "".to_string();
    }
    let video_id = if raw_uri.starts_with("https://www.youtube.com/watch?v=") {
        let v: Vec<&str> = raw_uri[32..].split('&').collect();
        v[0].to_string()
    } else if raw_uri.starts_with("https://youtu.be/") {
        let v: Vec<&str> = raw_uri[17..].split('/').collect();
        v[0].to_string()
    } else if raw_uri.starts_with("https://www.youtube.com/shorts/") {
        let v: Vec<&str> = raw_uri[31..].split('/').collect();
        v[0].to_string()
    } else {
        "".to_string()
    };
    let uri = if !video_id.is_empty() {
        String::from(format!("{}{}{}", "https://www.youtube.com/results?search_query='", video_id, "'"))
    } else if raw_uri.to_string().starts_with("https://www.youtube.com/") {
        raw_uri
    } else {
        String::from(format!("{}{}", "https://www.youtube.com/results?search_query='", raw_uri))
    };

    let res = reqwest::get(uri.to_string()).await;
    if res.is_err() {
        eprint!("Error: reqwest");
        return "".to_string();
    }
    let raw_body = res.unwrap().text().await;
    if raw_body.is_err() {
        eprint!("Error: body");
        return "".to_string();
    }

    let body = raw_body.unwrap();
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("script").unwrap();
    let mut inner_html = String::from("");
    for element in fragment.select(&selector) {
        if element.inner_html().contains("ytInitialData") {
            inner_html = element.inner_html();
            break;
        }
    }
    if inner_html.is_empty() {
        eprint!("Error: inner_html");
        return "".to_string();
    }
    let raw_data = &inner_html.replace("\n", "").trim().to_string();
    let raw_data_ = if raw_data.starts_with("// scraper_data_begin") {
        &raw_data[21..]
    } else {
        &raw_data
    };
    let raw_data__ = if raw_data_.starts_with("window[\"ytInitialData\"]") {
        &raw_data_[26..]
    } else if raw_data_.starts_with("var ytInitialData") {
        &raw_data_[20..]
    } else {
        &raw_data_
    };
    if raw_data__.is_empty() {
        eprint!("Error: raw_data");
        return "".to_string();
    }
    let data_vec: Vec<&str> = raw_data__.split("};").collect();
    let data = format!("{}{}", data_vec.get(0).unwrap(), "}");

    let data_access_str_set = if uri.to_string().starts_with("https://www.youtube.com/c/") ||
                                 uri.to_string().starts_with("https://www.youtube.com/user/") ||
                                 uri.to_string().starts_with("https://www.youtube.com/channel/") {
        CHANNEL_STR_SET
    } else if uri.to_string().starts_with("https://www.youtube.com/feed/trending") {
        TREND_STR_SET
    } else if uri.to_string().starts_with("https://www.youtube.com/results?search_query") {
        SEARCH_STR_SET
    } else if uri.to_string().starts_with("https://www.youtube.com/playlist") {
        PLAYLIST_STR_SET
    } else {
        TOP_STR_SET
    };

    let base_json_data = gjson::get(&data, data_access_str_set.base_access_str);
    let mut result: Vec<DataSet> = Vec::new();
    base_json_data.each(|_, value| {
        if value.get(data_access_str_set.id_access_str).exists() &&
            (video_id.len() < 1 || video_id.eq(&(value.get(data_access_str_set.id_access_str).str().to_string()))) {
            let data_set = DataSet{
                id:     value.get(data_access_str_set.id_access_str).str().to_string(),
                title:  value.get(data_access_str_set.title_access_str).str().to_string(),
                length: value.get(data_access_str_set.length_access_str).str().to_string(),
            };
            result.push(data_set);
        }
        true
    });
    if result.len() < 1 && !video_id.is_empty() {
        let data_set = DataSet{
            id:     video_id,
            title:  "".to_string(),
            length: "".to_string(),
        };
        result.push(data_set);
    }
    serde_json::to_string(&result).unwrap()
}
