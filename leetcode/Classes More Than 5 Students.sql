SELECT class FROM
  (SELECT class, count(DISTINCT student) num_students
   FROM courses
   GROUP BY class) AS csn
WHERE num_students >= 5