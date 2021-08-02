create table readings(
	time timestamp,
	id integer,
	type integer,
	consumption integer
);

-- ADMINISTRATION: table sizes, bytes per row

SELECT l.metric, l.nr AS bytes
     , CASE WHEN is_size THEN pg_size_pretty(nr) END AS bytes_pretty
     , CASE WHEN is_size THEN nr / NULLIF(x.ct, 0) END AS bytes_per_row
FROM  (
   SELECT min(tableoid)        AS tbl      -- = 'public.tbl'::regclass::oid
        , count(*)             AS ct
        , sum(length(t::text)) AS txt_len  -- length in characters
   FROM   public.readings t                     -- provide table name *once*
   ) x
CROSS  JOIN LATERAL (
   VALUES
     (true , 'core_relation_size'               , pg_relation_size(tbl))
   , (true , 'visibility_map'                   , pg_relation_size(tbl, 'vm'))
   , (true , 'free_space_map'                   , pg_relation_size(tbl, 'fsm'))
   , (true , 'table_size_incl_toast'            , pg_table_size(tbl))
   , (true , 'indexes_size'                     , pg_indexes_size(tbl))
   , (true , 'total_size_incl_toast_and_indexes', pg_total_relation_size(tbl))
   , (true , 'live_rows_in_text_representation' , txt_len)
   , (false, '------------------------------'   , NULL)
   , (false, 'row_count'                        , ct)
   , (false, 'live_tuples'                      , pg_stat_get_live_tuples(tbl))
   , (false, 'dead_tuples'                      , pg_stat_get_dead_tuples(tbl))
   ) l(is_size, metric, nr);
   
-- ANALYSIS


-- last 10 records
select * from readings order by time desc limit 10;

-- 100.73.94.71

-- recent watt hour readings
select time, consumption * 10 as watt_hours from readings order by time desc limit 10;

-- rate of readings
select count(*), date_trunc('minute', time) as minute from readings where time > now() - interval '1 day' group by minute order by minute desc;
select count(*), date_trunc('hour', time) as minute from readings where time > now() - interval '1 day' group by minute order by minute desc;
select count(*), date_trunc('day', time) as minute from readings where time > now() - interval '1 day' group by minute order by minute desc;

-- retransmit durations
select time, lag(time) over (order by time desc) - time as time_since_last from readings order by time desc;

-- average interval sizes
select avg(count), max(count), min(count), avg(interval), max(interval), min(interval) from (select count(*) count, max(time) - min(time) as interval from readings group by consumption) as retransmit_groups;

-- discrete derivative
select 
	count(*) as retransmit_count,
	min(time) as timestamp_first_transmit,
	max(time) - min(time) as interval_duration,
	extract(epoch from max(time) - min(time)) as interval_duration_seconds,
	consumption * 10 watt_hours,
	consumption * 10 - lag(consumption * 10) over (order by min(time) asc) diff
--	 (consumption * 10 - lag(consumption * 10) over (order by min(time) asc)) / (extract(epoch from max(time) - min(time)) / 3600) diff
from readings 
group by consumption
order by timestamp_first_transmit desc;

-- watt hours per interval, using complex nested query to filter single-item intervals (that would thus have an interval length of zero)
-- ********************** THIS IS WRONG **********************
with intervals as (
	select 
		count(*) as retransmit_count,
		min(time) as start_time,
		max(time) as end_time,
		consumption * 10 as watt_hours,
		extract(epoch from max(time) - min(time)) as interval_duration_seconds
	from readings 
	group by consumption
	having count(*) > 1
	order by start_time desc
)
select 
	*,
	watt_hours - lag(watt_hours) over (order by start_time asc) as new_watt_hours,
	watt_hours - lag(watt_hours) over (order by start_time asc) / (interval_duration_seconds / 3600) as watt_hours_per_second
	--	 (consumption * 10 - lag(consumption * 10) over (order by min(time) asc)) / (extract(epoch from max(time) - min(time)) / 3600) diff
from intervals
order by start_time desc;

-- probably correct discrete derivative per retransmit interval:
explain analyze with intervals as (
	select 
		count(*) as retransmit_count,
		min(time) as start_time,
		max(time) as end_time,
		consumption * 10 as watt_hours
	from readings 
	group by consumption
)
select 
	*,
	watt_hours - lag(watt_hours) over (order by start_time asc) as new_watt_hours,
	EXTRACT(epoch FROM (start_time - lag(start_time) over (order by start_time asc))) / 3600 as hours_since_last_update,
	(watt_hours - lag(watt_hours) over (order by start_time asc))
	/ 
	(EXTRACT(epoch FROM (start_time - lag(start_time) over (order by start_time asc))) / 3600) as avg_watts_in_interval
from intervals
order by start_time desc;

-- stripped down aggregates
with intervals as (
	select 
		(consumption * 10 - lag(consumption * 10) over (order by min(time) asc))
	/ 
	(EXTRACT(epoch FROM (min(time) - lag(min(time)) over (order by min(time) asc))) / 3600) as watts
	from readings 
	group by consumption
)
select 
	avg(watts),
	min(watts),
	max(watts)
from intervals;

-- as "up to date" as we can get:
with intervals as (
	select 
		count(*) as retransmit_count,
		min(time) as start_time,
		max(time) as end_time,
		consumption * 10 as watt_hours
	from readings 
	group by consumption
	order by start_time desc
	limit 2
)
select 
	start_time as last_updated,
	now() - start_time as data_staleness,
	(watt_hours - lag(watt_hours) over (order by start_time asc))
	/ 
	(EXTRACT(epoch FROM (start_time - lag(start_time) over (order by start_time asc))) / 3600) as avg_watts_in_last_complete_interval
from intervals
order by start_time desc
limit 1;

-- as json
select json_agg(t) from (
	with intervals as (
		select 
			min(time)::timestamptz at time zone 'America/Denver' as start_time,
			EXTRACT(epoch from min(time)) as start_time_unix,
			consumption * 10 as watt_hours
		from readings 
		group by consumption
	)
	select 
		start_time,
		start_time_unix,
		(watt_hours - lag(watt_hours) over (order by start_time asc))
		/ 
		((start_time_unix - lag(start_time_unix) over (order by start_time asc)) / 3600) as avg_watts_in_interval
	from intervals
	order by start_time desc
) as t;

-- kwh per day
select 
	date_trunc('day', time::timestamptz at time zone 'America/Denver') as day,
	min(consumption) * 10 as reading,
	(min(consumption) - lag(min(consumption)) over (order by date_trunc('day', time::timestamptz at time zone 'America/Denver') asc)) * 10 as diff
from readings
group by day
order by day desc;

select 
	date_trunc('hour', time::timestamptz at time zone 'America/Denver') as day,
	min(consumption) * 10 as reading,
	(min(consumption) - lag(min(consumption)) over (order by date_trunc('hour', time::timestamptz at time zone 'America/Denver') asc)) * 10 as diff
from readings
group by day
order by day desc;


-- debugging some weird 69,000 Watts issue :( 
select '2020-07-31T02:00:00'::timestamp, '2020-07-31T03:00:00'::timestamp;
select * from readings where time::timestamptz > '2020-07-31T02:00:00Z'::timestamptz and time::timestamptz < '2020-07-31T03:00:00Z'::timestamptz;

select 
	min(time) as first_seen, 
	EXTRACT(epoch from min(time)) as first_seen_unix, max(time) as last_seen, 
	consumption, 
	consumption - lag(consumption, 2) over (order by min(time) asc) as new_consumption,
	((EXTRACT(epoch from min(time)) - lag(EXTRACT(epoch from min(time))) over (order by min(time) asc))) as seconds_since_last,
	((EXTRACT(epoch from min(time)) - lag(EXTRACT(epoch from min(time)), 2) over (order by min(time) asc))) as seconds_since_last,
	((EXTRACT(epoch from min(time)) - lag(EXTRACT(epoch from min(time)), 2) over (order by min(time) asc)) / 3600) as hours_since_last,
	(consumption*10 - lag(consumption*10) over (order by min(time) asc)) / (((EXTRACT(epoch from min(time)) - lag(EXTRACT(epoch from min(time))) over (order by min(time) asc)) / 3600)) as watt_hours,
	(consumption*10 - lag(consumption*10, 2) over (order by min(time) asc)) / (((EXTRACT(epoch from min(time)) - lag(EXTRACT(epoch from min(time)), 2) over (order by min(time) asc)) / 3600)) as watt_hours_smoothed
from readings 
group by consumption 
order by min(time) desc 
limit 100;

-- first 10 records
select * from readings order by time asc limit 10;

-- diff
select max(consumption), min(consumption) from readings;

-- ok, now with an interval:
with intervals as (
	select 
		count(*) as retransmit_count,
		min(time) as start_time,
		max(time) as end_time,
		consumption * 10 as watt_hours
	from readings 
	where time >= now() - interval '1 week'
	group by consumption
)
select 
	*,
	watt_hours - lag(watt_hours) over (order by start_time asc) as new_watt_hours,
	EXTRACT(epoch FROM (start_time - lag(start_time) over (order by start_time asc))) / 3600 as hours_since_last_update,
	(watt_hours - lag(watt_hours) over (order by start_time asc))
	/ 
	(EXTRACT(epoch FROM (start_time - lag(start_time) over (order by start_time asc))) / 3600) as avg_watts_in_interval
from intervals
order by start_time desc;
-- AND it's much faster, even with no index. Interesting.
