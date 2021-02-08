SELECT d.Name AS Department,
       e.Name AS Employee,
       e.Salary
FROM Employee e
INNER JOIN Department d ON d.Id = e.DepartmentId
WHERE
    ( SELECT COUNT(DISTINCT salary)
     FROM Employee e2
     WHERE e2.DepartmentId = e.DepartmentId
       AND e2.Salary >= e.Salary ) <= 3