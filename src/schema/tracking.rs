use sea_query::Iden;

#[derive(Iden)]
pub enum UserShowsTracking {
    Table,
    Id,
    UserId,
    ShowId,
    Status, // WATCHING, COMPLETED, DROPPED, PLANNED
    Progress, // Número do episódio atual
    UpdatedAt,
}

#[derive(Iden)]
pub enum WatchHistory {
    Table,
    Id,
    UserId,
    ShowId,
    EpisodeId,
    WatchedAt,
}
