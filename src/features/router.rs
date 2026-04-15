use actix_web::{middleware::from_fn, web};

use crate::{
    app_controller::root,
    features::{create_user::create_user, list_user::list_user},
    middlewares::auth::auth_middleware,
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .wrap(from_fn(auth_middleware))
        .service(root)
        .service(create_user)
        .service(list_user);
    // );

    conf.service(scope);
}
