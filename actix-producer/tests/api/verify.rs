use actix_web::App;

use actix_producer::api::pet;
use actix_producer::repository::pet::PetRepository;
use stubr::*;

use crate::utils::*;

#[actix_web::test]
async fn should_verify() {
    App::new()
        .app_data(fake_pet_repository())
        .service(pet::find_all)
        .service(pet::create)
        .wrap(ActixVerifyLifecycle::<PetRepository> {
            before_each: |repo| {
                repo.delete_all()
                    .and_then(|_| repo.insert_all(fake_pets()))
                    .unwrap();
            },
        })
        .verify()
        .await;
}