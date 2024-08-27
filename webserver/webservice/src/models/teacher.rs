use actix_web::web;
use serde::{Deserialize, Serialize};
use actix_web::web::Json;
use crate::errors::MyError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32, //serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}


impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(update_teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        }
    }
}

/*
impl TryFrom<web::Json<CreateTeacher>> for CreateTeacher {
    type Error = MyError;

    fn try_from(new_teacher: Json<CreateTeacher>
    ) -> Result<Self, Self::Error> {
        Ok(CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        })
    }
}

impl TryFrom<web::Json<UpdateTeacher>> for UpdateTeacher {
    type Error = MyError;

    fn try_from(update_teacher: Json<UpdateTeacher>
    ) -> Result<Self, Self::Error> {
        Ok(UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        })
    }
}
*/