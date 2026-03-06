-- Add migration script here
ALTER TABLE resource_logs       ALTER COLUMN resource_id TYPE bigint;
ALTER TABLE assigned_resources  ALTER COLUMN resource_id TYPE bigint;
ALTER TABLE gantt_jobs_resources          ALTER COLUMN resource_id TYPE bigint;
ALTER TABLE gantt_jobs_resources_visu     ALTER COLUMN resource_id TYPE bigint;
ALTER TABLE gantt_jobs_resources_log      ALTER COLUMN resource_id TYPE bigint;