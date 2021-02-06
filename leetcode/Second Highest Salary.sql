with my_employee as (
    select distinct salary from employee 
)

select 
    case when(select count(*) from my_employee) = 1
    then null 
    else
    (select salary from my_employee
    order by salary desc
     limit 1 offset 1
    )
    end as SecondHighestSalary