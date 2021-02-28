with ids as(
    select s.id from Stadium s
    inner join Stadium s2 on(s.id+1 = s2.id)
    inner join Stadium s3 on(s2.id+1 = s3.id)
    where s.people >= 100 and s2.people >= 100 and s3.people >= 100
),
all_ids as(
    select id from ids
    union all
    select id+1 from ids
    union all
    select id+2 from ids
)
select * from Stadium
where id in(select id from all_ids)
order by visit_date