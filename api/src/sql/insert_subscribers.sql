INSERT INTO subscribers (endpoint, p256dh, auth, date)
VALUES (?, ?, ?, strftime('%s', 'now'));