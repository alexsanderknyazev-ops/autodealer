ALTER TABLE cars ADD COLUMN completed_service_campaigns UUID[] DEFAULT '{}' NOT NULL;

-- Создаем индекс для оптимизации поиска
CREATE INDEX idx_cars_completed_campaigns ON cars USING GIN (completed_service_campaigns);

-- Комментарий к колонке
COMMENT ON COLUMN cars.completed_service_campaigns IS 'Массив ID выполненных сервисных кампаний для этого автомобиля';