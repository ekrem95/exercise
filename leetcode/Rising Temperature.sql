SELECT w.id
FROM Weather w
INNER JOIN Weather w2 ON DATE_ADD(w2.recordDate, INTERVAL 1 DAY) = w.recordDate
WHERE w.temperature > w2.temperature