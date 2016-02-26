//Autogenerated
use hyper::Client;
pub fn get(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 17);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template");
    url_fmtd
}
pub fn post(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 17);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template");
    url_fmtd
}
pub fn get_id(base: String, id: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(&id);
    url_fmtd
}
pub fn post_id(base: String, id: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(&id);
    url_fmtd
}