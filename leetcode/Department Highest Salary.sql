    with bulk as (
    select
        d.Id as DepartmentId,
        d.name as DepartmentName,
        e.name as EmployeeName,
        e.salary as Salary
    from employee e
    left join department d on(e.departmentid = d.id)
),
max_per_department as (
    select DepartmentId, max(salary) as max_salary
    from bulk
    group by DepartmentId
)

select 
    DepartmentName as Department,
    EmployeeName as Employee,
    Salary
from bulk b
inner join max_per_department m on(b.DepartmentId = m.DepartmentId and b.salary = m.max_salary)