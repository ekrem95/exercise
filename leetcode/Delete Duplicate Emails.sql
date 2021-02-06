with to_keep as (
    select min(id) as id, email
    from person
    group by email
)

delete from person where id not in (select id from to_keep)