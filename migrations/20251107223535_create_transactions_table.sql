-- Add migration script here
CREATE TABLE "transactions" (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    category_id BIGINT NOT NULL,
    type VARCHAR(50) NOT NULL,
    amount BIGINT NOT NULL,
    memo VARCHAR(255),
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "users" (id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES "categories" (id) ON DELETE CASCADE
);

CREATE TRIGGER update_transactions_updated_at
BEFORE UPDATE ON "transactions"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
