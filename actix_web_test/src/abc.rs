pub mod test {
    use actix_web::{get, Responder, HttpRequest};
    use serde_derive::Deserialize;
    use actix_web::web::Path;
    use serde::export::Option::Some;
    use std::borrow::Borrow;
    use dashmap::DashMap;

    lazy_static! {
        static ref WhiteHASHMAP: DashMap<String,bool> = {
            let mut m = dashmap::DashMap::new();
            m
        };
        static ref BlackHASHMAP: DashMap<String,bool> = {
            let mut m1 = dashmap::DashMap::new();
            m1
        };
    }

    #[get("/index.html")]
    pub async fn index() -> impl Responder {
        format!("Hello")
    }

    #[derive(Debug, Deserialize)]
    pub struct Req {
        #[serde(rename = "req_type")]
        req_num: i32,
        #[serde(rename = "req_content")]
        content: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Content {
        phone: String,
        act_type: i32,
        list_type: i32,
        platform_id: i32,
        content: String,
    }

    pub async fn yyy(source: HttpRequest, path: Path<String>, data: String) -> impl Responder {
        let mut rip: String = String::from("");
        if let Some(ip) = source.peer_addr() {
            rip = ip.ip().to_string()
        }
        let req: serde_json::Result<Req> = serde_json::from_str(&*data);
        return match req {
            Ok(x) => {
                process(x.content.borrow()).await
            }
            Err(e) => e.to_string()
        };
    }

    const TYURL: &str = "http://47.96.136.198:8001/sms/api/sendMessage";

    async fn process(str: &String) -> String {
        let o: serde_json::Result<Content> = serde_json::from_str(str);
        return match o {
            Ok(req) => {
                if req.act_type < 3 {  //添加或删除白名单或黑名单
                    return process_list(&req.phone, req.list_type, req.act_type).await
                } else {  //发送短信操作
                    match req.list_type {
                        1 => {
                            if WhiteHASHMAP.contains_key(&req.phone) {
                                return "手机号在白名单中！".to_string();
                            }
                        }
                        2 => {
                            if BlackHASHMAP.contains_key(&req.phone) {
                                return "手机号在黑名单中！".to_string();
                            }
                        }
                        _ => return send_msg(&req).await
                    }
                }
                "".to_string()
            }
            Err(e) => e.to_string()
        };
    }

    //发送短信
    async fn send_msg(req:&Content)->String{
        let payload = format!(r#"{{"userName":"{}","password":"{}","content": "{}","phoneList": ["{}"],"callData":""}}"#,
                                   "cetyzm","123456",req.content,req.phone);
        println!("{}",payload);
        let clinet = awc::Client::new();
        let mut response = clinet.post(TYURL)
            .header("content-type","application/json")
            .header("cache-control","no-cache")
            .send_body(payload)
            .await;
        return match response {
            Ok(mut o)=>{
                let body = o.body().await;
                return match body {
                    Ok(b)=>{
                        let tmp11 = String::from_utf8(b.to_vec()).unwrap();
                        gl_log::log::info!("{}",tmp11);
                        tmp11
                    },
                    Err(e)=>{
                        gl_log::log::error!("{:#?}",e);
                        "222".to_string()
                    }
                }
            },
            Err(e)=>{
                gl_log::log::error!("{:#?}",e);
                "111".to_string()
            }
        }
    }

    //操作黑白名单
    async fn process_list(phone: &String, list_type: i32, act_type: i32) -> String {
        return match act_type {
            1 => {
                return match list_type {
                    1 => {
                        if !WhiteHASHMAP.contains_key(phone) {
                            WhiteHASHMAP.insert(phone.to_string(), true);
                        }
                        "添加白名单成功".to_string()
                    }
                    2 => {
                        if !BlackHASHMAP.contains_key(phone) {
                            BlackHASHMAP.insert(phone.to_string(), true);
                        }
                        "添加黑名单成功".to_string()
                    }
                    _ => "请求错误".to_string()
                };
            }
            2 => {
                return match list_type {
                    1 => {
                        if WhiteHASHMAP.contains_key(phone) {
                            WhiteHASHMAP.remove(phone);
                        }
                        "删除白名单成功".to_string()
                    }
                    2 => {
                        if BlackHASHMAP.contains_key(phone) {
                            BlackHASHMAP.remove(phone);
                        }
                        "删除黑名单成功".to_string()
                    }
                    _ => "请求错误".to_string()
                };
            }
            _ => "请求错误".to_string()
        };
    }
}