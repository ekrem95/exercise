WITH total AS
  (SELECT t.Status,
          t.Request_at AS DAY,
          CASE
              WHEN Status != 'completed' THEN 1
              ELSE 0
          END AS Canceled
   FROM Users c
   LEFT JOIN Trips t ON(c.Users_id = t.Client_Id)
   WHERE c.Banned = 'No'
     AND t.Request_at >= '2013-10-01'
     AND t.Request_at <= '2013-10-03' )
SELECT DAY,
       round(Sum(Canceled) / Count(*), 2) AS 'Cancellation Rate'
FROM total
GROUP BY DAY