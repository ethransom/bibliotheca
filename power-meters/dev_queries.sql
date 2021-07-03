create table readings(
	time timestamp,
	unknown1 integer,
	unknown2 integer,
	id integer,
	type integer,
	tamper_phy text,
	tamper_enc text,
	consumption integer,
	crc text
);

select count(*) from readings;

select id, time, consumption from readings limit 10;

select count(distinct id) from readings;
select distinct id from readings;

with diffs as (select type, max(consumption) - min(consumption) as diff from readings group by type, id) select type, max(diff) from diffs group by type;

/*
 * FROM: https://web.stanford.edu/class/ee26n/Assignments/Assignment6.html
 *
 * Electric: 04, 05, 07, 08
 * Gas: 02, 09, 12
 * Water: 11, 13
*/


-- consumption
select id, count(*), min(consumption), max(consumption), max(consumption) - min(consumption) as diff from readings group by id order by diff desc;

-- recent consumption
select id, count(*), min(consumption), max(consumption), max(consumption) - min(consumption) as diff from readings where time > '2021-06-23 00:43:00' group by id order by diff desc;

-- readings per minute
select count(*), date_trunc('minute', time) as minute from readings group by minute order by minute asc;

-- histogrammed consumptions
select (diff / 10) * 10 as bucket, count(*) as count from (select id, count(*), min(consumption), max(consumption), max(consumption) - min(consumption) as diff from readings group by id order by diff desc) as meters group by bucket order by bucket desc;

-- datapoint spreads (more distant meters have sparser readings?)
select id, count(*), min(time), max(time), max(time) - min(time) as diff from readings group by id order by max; 

-- hisogrammed readings
select (count / 5) * 5 as bucket, count(*) as count from (select id, count(*) as count, min(consumption), max(consumption), max(consumption) - min(consumption) as diff from readings group by id order by diff desc) as meters group by bucket order by bucket desc;

-- illustrative consumption for Comstock
explain analyze select meters.*, "total consumption" / "minutes of data" as "avg consumption per minute" from (
	select 
		id as "meter serial number", 
		count(*) as "number of readings", 
		max(time) - min(time) as "reading time period", 
		max(consumption) - min(consumption) as "total consumption",
		(EXTRACT(EPOCH from (max(time) - min(time))) / 60) as "minutes of data"
	from readings 
	group by id 
) as meters where "minutes of data" != 0 order by "reading time period" desc, "total consumption";





----------------------------------- PARENTS ------------------------------------------

select count(*) from readings_parents;

select id, time, consumption from readings_parents limit 10;

select count(distinct id) from readings_parents;
select distinct id from readings_parents where id::text like '%427%';
select distinct type from readings_parents;

select time, consumption from readings_parents where id = 63021097 and type in (4, 5, 7, 8) order by time asc;

select date_trunc('minute', time) as minute, max(consumption) from readings_parents where id = 63021097 group by minute order by minute asc;

select max(time) - min(time) as timerange, max(consumption) - min(consumption) as consumption from readings_parents where id = 63021097;

-- perform a discrete derivative and running total
select 
	time, 
	consumption reading, 
	consumption - lag(consumption) over (order by time) diff,
	min(consumption) over(order by time rows between 1 preceding and current row) prev,
	sum(consumption) over(order by time rows between unbounded preceding and current row) running_total
from readings_parents 
where id = 63021097 
order by time asc;

-- just how often are these updated??
select 
	min(time) as start, 
	max(time) - min(time) duration,
	consumption reading
from readings_parents 
where id = 63021097
group by consumption
order by start asc;

-- maybe to simplify we only calculate the discrete derivative over these updates?
select 
	count(*),
	min(time) as start, 
	max(time) - min(time) duration,
	consumption * 10 watt_hours,
	consumption * 10 - lag(consumption * 10) over (order by min(time)) diff
	-- (consumption * 10 - lag(consumption * 10) over (order by min(time))) / (extract(epoch from max(time) - min(time)) / 3600) diff
from readings_parents 
where id = 63021097
group by consumption
order by start asc;

-- Ok, simplifying a bit for deployment
create table readings(
	time timestamp,
	id integer,
	type integer,
	consumption integer
);

select * from readings where time > '2021-06-27' order by time desc;

	select 
		min(time) as start_time,
		EXTRACT(epoch from min(time)) as start_time_unix,
		consumption * 10 as watt_hours
	from readings 
	group by consumption;