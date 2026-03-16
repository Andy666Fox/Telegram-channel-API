use scraper::{Html, Selector};
use crate::schemas::ChannelInfo;

pub fn parse_html(html_body: &str, identifier: &str, is_redirected: bool) -> ChannelInfo {
    let document = Html::parse_document(html_body);
    
    let info_selector = Selector::parse(".tgme_channel_info").unwrap();
    let counter_item_selector = Selector::parse(".tgme_channel_info_counter").unwrap();
    let val_selector = Selector::parse(".counter_value").unwrap();
    let label_selector = Selector::parse(".counter_type").unwrap();

    let mut info = ChannelInfo {
        name: format!("@{}", identifier),
        ..Default::default()
    };

    if let Some(_) = document.select(&info_selector).next() {
        info.content_type = "channel".to_string();
        info.private = false;

        for counter_item in document.select(&counter_item_selector) {
            let val = counter_item.select(&val_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            
            let label = counter_item.select(&label_selector)
                .next()
                .map(|el| el.text().collect::<String>().to_lowercase())
                .unwrap_or_default();

            if label.contains("subscribers") || label.contains("members") {
                info.subscribers = Some(val);
            } else if label.contains("photo") {
                info.photos = Some(val);
            } else if label.contains("video") {
                info.videos = Some(val);
            } else if label.contains("link") {
                info.links = Some(val);
            }
        }
    } else {
        if is_redirected {
            info.content_type = "user".to_string();
            info.private = false;
        } else {
            info.content_type = "channel".to_string();
            info.private = true;
        }
    }

    info
}