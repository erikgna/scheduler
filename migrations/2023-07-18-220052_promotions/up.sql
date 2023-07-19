CREATE TABLE promotions (
    id_promotion SERIAL PRIMARY KEY,
    promotion_name VARCHAR(100) NOT NULL,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    discount_code VARCHAR(20)
);