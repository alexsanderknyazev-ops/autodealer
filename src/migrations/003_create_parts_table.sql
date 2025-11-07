-- Создание таблицы запчастей
CREATE TABLE IF NOT EXISTS parts (
                                     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(200) NOT NULL,
    model VARCHAR(100) NOT NULL,
    purchase_price FLOAT NOT NULL CHECK (purchase_price >= 0),
    sale_price FLOAT NOT NULL CHECK (sale_price >= 0),
    compatible_vins TEXT[] NOT NULL DEFAULT '{}', -- Массив VIN кодов
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

-- Индексы для быстрого поиска
CREATE INDEX IF NOT EXISTS idx_parts_article ON parts(article);
CREATE INDEX IF NOT EXISTS idx_parts_model ON parts(model);
CREATE INDEX IF NOT EXISTS idx_parts_created_at ON parts(created_at DESC);

-- Индекс для поиска по массиву VIN кодов
CREATE INDEX IF NOT EXISTS idx_parts_compatible_vins ON parts USING GIN (compatible_vins);