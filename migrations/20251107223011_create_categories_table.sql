-- Add migration script here
CREATE TABLE "categories" (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "users" (id) ON DELETE CASCADE
);

CREATE TRIGGER update_categories_updated_at
BEFORE UPDATE ON "categories"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column(); 