CREATE TABLE IF NOT EXISTS subscribers (
    endpoint TEXT NOT NULL,
    p256dh TEXT NOT NULL,
    auth TEXT NOT NULL,
    date INTEGER NOT NULL,
    PRIMARY KEY (endpoint)
)