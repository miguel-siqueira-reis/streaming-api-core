-- PostgreSQL Schema
-- ==========================================
-- 1. SISTEMA DE AUTENTICAÇÃO E PERMISSÕES (Laravel Sanctum Style)
-- ==========================================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'USER',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE personal_access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    token VARCHAR(64) NOT NULL UNIQUE,
    abilities JSONB DEFAULT '["*"]',
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE password_reset_tokens (
    email VARCHAR(255) PRIMARY KEY,
    token VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- 2. DOMÍNIO DO PRODUTO (Catálogo de Streaming)
-- ==========================================

CREATE TABLE shows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,                           -- Título Principal (Romaji ou PT-BR)
    alternative_titles JSONB DEFAULT '{}',                 -- Dicionário de línguas: {"en": "Attack on Titan", "ja": "進撃の巨人", "synonyms": ["SnK", "AOT"]}
    slug VARCHAR(255) NOT NULL UNIQUE,
    synopsis TEXT,
    cover_image_url TEXT,
    status VARCHAR(50) DEFAULT 'ONGOING',
    metadata JSONB DEFAULT '{}',                           -- flexibilidade (estúdio, data de estreia, dub/leg)
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE episodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    show_id UUID NOT NULL REFERENCES shows(id) ON DELETE CASCADE,
    number INTEGER NOT NULL,
    title VARCHAR(255),
    video_path TEXT,
    transcode_status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    metadata JSONB DEFAULT '{}', -- flexibilidade (qualidades HLS geradas, se é filler, etc)
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_shows_episode_number ON episodes(show_id, number);
CREATE INDEX idx_tokens_lookup ON personal_access_tokens(token);

-- ==========================================
-- 3. A PROFUNDIDADE DO PRODUTO (Gêneros, Minha Lista e Histórico)
-- ==========================================

-- Tabela Central de Gêneros/Tags (Shounen, Romance, Isekai, etc)
CREATE TABLE genres (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE
);

-- Tabela Pivô (N:N) - Qual show tem qual categoria?
CREATE TABLE show_genres (
    show_id UUID NOT NULL REFERENCES shows(id) ON DELETE CASCADE,
    genre_id UUID NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
    PRIMARY KEY (show_id, genre_id)
);

-- ==========================================
-- ESTRUTURAS DE ALTA PERFORMANCE E BUSCA (Search Engines)
-- ==========================================

-- Ativamos Trigramas no Postgres (A Magia do Fuzzy Search para erros de digitação e Buscas TEXT LIKE)
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- 1. Índice GIN com Trigramas para o Título (Permite buscar '%Hero%' na velocidade da luz invés do Sequential Scan)
CREATE INDEX idx_shows_title_trgm ON shows USING GIN (title gin_trgm_ops);

-- 2. Índice GIN para os Títulos Alternativos (Inglês, Japonês, Abreviações)
-- Quando o cara digitar "Shingeki", o Postgres varre esse JSONB instantaneamente sem ler a tabela shows.
CREATE INDEX idx_shows_alt_titles_gin ON shows USING GIN (alternative_titles);

-- 3. Índice Fixo na tabela de shows. Isso permite queries ultrarrápidas de "Dê-me Shows ordenados por novidades"
CREATE INDEX idx_shows_created_at ON shows(created_at DESC);

-- 2. Índice Reverso na Tabela Pivô! Magia de Performance
-- A Primary Key "show_id, genre_id" faz o banco saber rápido "Quais generos o Show X tem?"
-- Mas esse índice reverso (Baixo) diz ao banco como achar instantaneamente "Quais Shows têm a categoria Y?" -> Extremamente rápido num SQL Intersect / IN / NOT IN.
CREATE INDEX idx_reverso_genre_shows ON show_genres(genre_id, show_id);

-- ==========================================
-- 4. O SISTEMA DE TRACKING DO USUÁRIO
-- ==========================================
-- Status permitidos que o App vai usar: 'WATCHING', 'PLAN_TO_WATCH', 'COMPLETED', 'DROPPED'
CREATE TABLE user_show_tracking (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    show_id UUID NOT NULL REFERENCES shows(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'PLAN_TO_WATCH',
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, show_id)
);

-- O grande Salvador de Vidas: "Onde parei no último episódio?" (Continue Watching)
CREATE TABLE watch_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    episode_id UUID NOT NULL REFERENCES episodes(id) ON DELETE CASCADE,
    progress_seconds INTEGER NOT NULL DEFAULT 0,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    -- Essa chave abaixo garante que não existem 2 saves do mesmo usuário no mesmo episódio. Se ele ver de novo, atualiza o antigo.
    UNIQUE (user_id, episode_id)
);

-- Filtro Rápido pro "Continue Assistindo" (Ordena pela última vez que ele deu Play em algo do banco)
CREATE INDEX idx_watch_history_user_updated ON watch_history(user_id, updated_at DESC);

