use sea_query::Iden;

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    Username,
    Email,
    PasswordHash,
    Role,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum PersonalAccessTokens {
    Table,
    Id,
    UserId,
    Token,
    Name,
    Abilities,
    LastUsedAt,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}
