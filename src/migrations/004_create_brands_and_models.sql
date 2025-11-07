-- Создание таблицы брендов
CREATE TABLE IF NOT EXISTS brands (
                                      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    country VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

-- Создание таблицы моделей автомобилей
CREATE TABLE IF NOT EXISTS car_models (
                                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    brand_id UUID NOT NULL REFERENCES brands(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(brand_id, name) -- Уникальная комбинация бренда и модели
    );

-- Обновляем таблицу автомобилей - добавляем связи с брендами и моделями
ALTER TABLE cars
    ADD COLUMN brand_id UUID REFERENCES brands(id),
ADD COLUMN model_id UUID REFERENCES car_models(id);

-- Обновляем таблицу запчастей - добавляем связи с брендами и моделями
ALTER TABLE parts
    ADD COLUMN brand_id UUID REFERENCES brands(id),
ADD COLUMN car_model_id UUID REFERENCES car_models(id);

-- Индексы
CREATE INDEX IF NOT EXISTS idx_brands_name ON brands(name);
CREATE INDEX IF NOT EXISTS idx_car_models_brand_id ON car_models(brand_id);
CREATE INDEX IF NOT EXISTS idx_car_models_name ON car_models(name);
CREATE INDEX IF NOT EXISTS idx_cars_brand_id ON cars(brand_id);
CREATE INDEX IF NOT EXISTS idx_cars_model_id ON cars(model_id);
CREATE INDEX IF NOT EXISTS idx_parts_brand_id ON parts(brand_id);
CREATE INDEX IF NOT EXISTS idx_parts_car_model_id ON parts(car_model_id);