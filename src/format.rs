use reqwest::blocking::Response;
use select::{document::Document, node::Node, predicate::Name};
#[derive(Debug)]
pub struct ListItem {
    pub name: String,
    pub magnet: String,
    pub size: String,
}
pub fn get_list(response: Option<Response>) -> Vec<ListItem> {
    let mut vec_result = Vec::new();
    // check if there is a response
    if let Some(res) = response {
        let docum = Document::from_read(res);
        if let Ok(doc) = docum {
            if let Some(tbody) = doc.find(Name("tbody")).nth(0) {
                let trs = tbody.find(Name("tr"));

                for elements in trs {
                    vec_result.push(ListItem {
                        name: get_p(elements, 1, "title"),
                        magnet: get_p(elements, 2, "href"),
                        size: get_p(elements, 3, ""),
                    })
                }
            }
        }
    }
    vec_result
}
fn get_p(n: Node, indexes: usize, att: &str) -> String {
    let mut result: String = String::new();
    // get the first element (which is a td's children)
    if let Some(a_element) = n.find(Name("td")).nth(indexes) {
        if att == "" {
            result = a_element.text();
        } else {
            if let Some(b_element) = a_element.find(Name("a")).last() {
                if let Some(text) = b_element.attr(att) {
                    result = text.clone().to_string();
                }
            }
        }
    }
    result
}
