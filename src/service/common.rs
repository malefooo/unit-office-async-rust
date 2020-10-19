use actix_web::{HttpResponse, post, HttpRequest, web, body};
use actix_http::{http, Request, Response};
use crate::{entity, MONGODB_INSTANCE, CONF_INSTANCE, constant};
use actix_web::http::StatusCode;
use log::info;
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};
use mongodb::options::FindOneOptions;
use uuid::Uuid;
use crate::entity::company::Company;
use actix_web::dev::ResponseHead;

pub async fn register(vo:web::Json<entity::customer::Customer>) -> Response{
    info!("{:?}",vo.0);
    let uid = Uuid::new_v4();
    let mut response = HttpResponse::new(StatusCode::OK);
    let mongodb_client = MONGODB_INSTANCE.get().clone().unwrap();
    let config = CONF_INSTANCE.get().clone().unwrap();
    let str_data_base = config.clone().mongo.database;

    let data_base = mongodb_client.database(str_data_base.as_str());
    let company_coll = data_base.collection(constant::COMPANY_COLLECTION);
    let company_value = vo.0.company.clone().unwrap();
    let company: entity::company::Company = serde_json::from_value(company_value).unwrap();
    let company_name = company.name.unwrap();
    let filter = doc!{"name":company_name};
    let mut op_find_company = company_coll.find_one(filter,FindOneOptions::default()).await.unwrap();

    if op_find_company.is_some() {
        info!("{:?},注册:公司已存在:{:?}",uid,op_find_company.clone());
        let r : entity::r::R = entity::r::R{
            code: Option::Some(10102005),
            message: Option::Some("公司已存在".to_string()),
            data: None
        };
        let v = serde_json::to_value(r).unwrap();
        let body = body::Body::from()

        return Response::Ok()
            .set_header(http::header::CONTENT_TYPE, "application/json;charset=UTF-8")
            .body(body);
    }

    Response::new(StatusCode::OK)
}