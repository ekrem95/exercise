with email_count as(
    select Email, count(Email) as c
    from Person
    group by Email
)

select Email from email_count
where c > 1
