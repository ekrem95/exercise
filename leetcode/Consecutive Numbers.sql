select distinct l1.num as ConsecutiveNums from logs l1
inner join logs l2 on (l1.id+1 =l2.id and l1.num = l2.num)
inner join logs l3 on (l1.id+2 =l3.id and l1.num = l3.num)
