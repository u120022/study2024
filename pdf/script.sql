CREATE TEMPORARY TABLE tmp_table AS (
    SELECT 
        t2.u,
        t2.v,
        t2.key,
        t1.loc_code,
        ST_FrechetDistance(ST_Rotate(t1.geometry, -pi()/2, ST_StartPoint(t1.geometry)), t2.geometry) as dist,
        t1.geometry AS t1_geometry,
        t2.geometry AS t2_geometry
    FROM
        detector_lines AS t1 
    JOIN 
        edges AS t2 
    ON
        ST_DWithin(ST_StartPoint(t1.geometry)::geography, t2.geometry::geography, 30.0)
    WHERE
        t2.highway = 'trunk'
        OR t2.highway = 'primary'
        OR t2.highway = 'secondary'
        OR t2.highway = 'tertiary'
);

CREATE TABLE detector_edges AS (
    SELECT
        t2.u,
        t2.v,
        t2.key,
        t2.loc_code,
        t2.t2_geometry AS geometry
    FROM (
        SELECT
            loc_code,
            min(dist) as dist
        FROM
            tmp_table
        GROUP BY
            loc_code
    ) AS t1
    JOIN
        tmp_table AS t2
    ON
        t1.loc_code = t2.loc_code AND t1.dist = t2.dist
);
