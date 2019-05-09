select
  q1.f_group_id,
  coalesce(q2.f_article_count, 0)::integer as f_article_count,
  coalesce(q2.f_low_water_mark, 0)::integer as f_low_water_mark,
  coalesce(q2.f_high_water_mark, 0)::integer as f_high_water_mark
from
  dialogue.t_group q1
  left outer join
  (
    select
      f_group_id,
      count(f_number) as f_article_count,
      min(f_number) as f_low_water_mark,
      max(f_number) as f_high_water_mark
    from
      dialogue.t_group_article
    group by f_group_id
  ) q2
  on q1.f_group_id = q2.f_group_id
where
  q1.f_group_id = $1
;
