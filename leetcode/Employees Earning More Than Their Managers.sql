select name as Employee
from Employee e
where ManagerId is not null
and e.Salary > (select Salary from Employee e2 where e.ManagerId = e2.Id)