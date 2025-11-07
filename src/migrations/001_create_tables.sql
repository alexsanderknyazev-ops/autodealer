-- Создание таблицы автомобилей
CREATE TABLE IF NOT EXISTS cars (
                                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                    brand VARCHAR(100) NOT NULL,
                                    model VARCHAR(100) NOT NULL,
                                    year INTEGER NOT NULL CHECK (year >= 1990 AND year <= 2024),
                                    price FLOAT NOT NULL CHECK (price >= 0),
                                    mileage INTEGER NOT NULL CHECK (mileage >= 0),
                                    color VARCHAR(50) NOT NULL,
                                    fuel_type VARCHAR(20) NOT NULL CHECK (fuel_type IN ('Petrol', 'Diesel', 'Electric', 'Hybrid')),
                                    transmission VARCHAR(20) NOT NULL CHECK (transmission IN ('Manual', 'Automatic', 'CVT')),
                                    status VARCHAR(20) NOT NULL DEFAULT 'Available' CHECK (status IN ('Available', 'Reserved', 'Sold', 'Maintenance')),
                                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Таблица клиентов
CREATE TABLE IF NOT EXISTS customers (
                                         id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                         first_name VARCHAR(100) NOT NULL,
                                         last_name VARCHAR(100) NOT NULL,
                                         email VARCHAR(255) UNIQUE NOT NULL,
                                         phone VARCHAR(50) NOT NULL,
                                         created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Таблица заявок на покупку
CREATE TABLE IF NOT EXISTS purchase_requests (
                                                 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                                 car_id UUID NOT NULL REFERENCES cars(id) ON DELETE CASCADE,
                                                 customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
                                                 status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Approved', 'Rejected', 'Completed')),
                                                 offer_price FLOAT CHECK (offer_price >= 0),
                                                 notes TEXT,
                                                 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                                 updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Индексы для автомобилей
CREATE INDEX IF NOT EXISTS idx_cars_brand_model ON cars(brand, model);
CREATE INDEX IF NOT EXISTS idx_cars_status ON cars(status);
CREATE INDEX IF NOT EXISTS idx_cars_price ON cars(price);
CREATE INDEX IF NOT EXISTS idx_cars_created_at ON cars(created_at DESC);

-- Индексы для клиентов
CREATE INDEX IF NOT EXISTS idx_customers_email ON customers(email);
CREATE INDEX IF NOT EXISTS idx_customers_name ON customers(first_name, last_name);

-- Индексы для заявок
CREATE INDEX IF NOT EXISTS idx_purchase_requests_customer_id ON purchase_requests(customer_id);
CREATE INDEX IF NOT EXISTS idx_purchase_requests_car_id ON purchase_requests(car_id);
CREATE INDEX IF NOT EXISTS idx_purchase_requests_status ON purchase_requests(status);
CREATE INDEX IF NOT EXISTS idx_purchase_requests_created_at ON purchase_requests(created_at DESC);

-- Уникальный индекс чтобы один клиент не мог создать multiple pending заявки на одну машину
CREATE UNIQUE INDEX IF NOT EXISTS idx_purchase_requests_car_customer_pending
    ON purchase_requests(car_id, customer_id)
    WHERE status = 'Pending';