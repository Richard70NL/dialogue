select
  f_number
from
  dialogue.t_group_article
where
  f_group_id = $1
  and f_number >= $2
  and f_number <= $3
;
