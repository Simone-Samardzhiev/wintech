use backend::adapters::postgres::user::{
    TokenRepository as PostgresTokenRepository, UserRepository as PostgresUserRepository,
};
use backend::domain::user::{
    models::{Token, TokenKind, User, UserError},
    ports::{TokenRepository, UserRepository},
};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[sqlx::test]
#[ignore]
async fn test_user_repository_save(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);

    let email = "example@email.com";
    let user = User::parse(
        Uuid::new_v4(),
        "Username1".into(),
        email.into(),
        "Password_123".into(),
    )
    .unwrap();

    repository
        .save(
            &User::parse(
                Uuid::new_v4(),
                "Username2".into(),
                email.into(),
                "Password_123".into(),
            )
            .unwrap(),
        )
        .await
        .expect("Failed to save user");

    let result = repository.save(&user).await;
    assert!(
        matches!(&result, Err(UserError::EmailAlreadyExists(e)) if e == email),
        "Expected EmailAlreadyExists error, got {:?}",
        result
    );
}

#[sqlx::test]
#[ignore]
async fn test_user_repository_get_by_email(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let email = "unique_email@example.com".to_string();

    let saved_user = User::parse(
        Uuid::new_v4(),
        "Username1".into(),
        email.clone(),
        "Password_123".into(),
    )
    .unwrap();
    repository
        .save(&saved_user)
        .await
        .expect("Failed to save user");

    let fetched_user = repository
        .get_by_email(&email)
        .await
        .expect("Fetching with the same email should succeed");

    assert_eq!(
        saved_user, fetched_user,
        "Fetched user should be equals to the saved one"
    );
}

#[sqlx::test]
#[ignore]
async fn test_token_repository_save(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let token_repository = PostgresTokenRepository::new(pool);

    let user = User::parse(
        Uuid::new_v4(),
        "Username1".into(),
        "unique_email@example.com".into(),
        "Password_123".into(),
    )
    .unwrap();
    user_repository
        .save(&user)
        .await
        .expect("Failed to save user");

    token_repository
        .save(&Token::new(
            Uuid::new_v4(),
            TokenKind::Refresh,
            OffsetDateTime::now_utc(),
            user.id,
        ))
        .await
        .expect("Failed to save token");
}

#[sqlx::test]
#[ignore]
async fn test_token_repository_delete(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let token_repository = PostgresTokenRepository::new(pool);

    let user = User::parse(
        Uuid::new_v4(),
        "Username1".into(),
        "unique_email@example.com".into(),
        "Password_123".into(),
    )
    .unwrap();
    user_repository
        .save(&user)
        .await
        .expect("Failed to save user");

    let token = Token::new(
        Uuid::new_v4(),
        TokenKind::Refresh,
        OffsetDateTime::now_utc(),
        user.id,
    );

    token_repository
        .save(&token)
        .await
        .expect("Failed to save token");

    token_repository
        .delete(token.id)
        .await
        .expect("Failed to delete token");

    let result = token_repository.delete(token.id).await;
    assert!(
        matches!(&result, Err(UserError::TokenNotFoundById(id)) if *id == token.id),
        "Expected TokeNotFoundById error {:?}",
        result
    );
}
