//Autogenerated
use hyper::Client;
pub fn get(base: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 13);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cat/aliases");
    url_fmtd
}
pub fn get_name(base: String, name: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 14 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cat/aliases/");
    url_fmtd.push_str(&name);
    url_fmtd
}