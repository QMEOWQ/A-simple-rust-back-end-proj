
use super::handlers::{course::*, general::*, teacher::*};
use actix_web::web;
//use actix_web::web::{self, route};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("/courses")
            .route("/", web::post().to(post_new_course)) // 创建新课程
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher)) // 获取特定教师的课程列表
            .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail)) // 获取课程详情
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course)) // 删除课程
            .route("/{teacher_id}/{course_id}", web::put().to(update_course_details)) // 更新课程详情
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("/teachers")
            .route("/", web::post().to(post_new_teacher)) // 创建新课程
            .route("/", web::get().to(get_all_teachers)) // 获取特定教师的课程列表
            .route("/{teacher_id}", web::get().to(get_teacher_details)) // 获取课程详情
            .route("/{teacher_id}", web::put().to(update_teacher_details)) // 删除课程
            .route("/{teacher_id}", web::delete().to(delete_teacher)) // 更新课程详情
    );
}







