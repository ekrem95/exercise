with ranks as (
    select distinct score, 
        rank () over (
            order by score desc
        ) as rank_number
    
    from (select distinct score from Scores) as ds
) 

select score, rank_number as `Rank`
from Scores
left join ranks using (score)
order by rank_number 