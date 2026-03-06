use async_trait::async_trait;
use oar_domain::resource::{
    Resource, ResourceError, ResourceLog,
    ports::{NewResource, ResourceRepository},
    value_objects::{ResourceId, ResourceNextState, ResourceState},
};
use sqlx::{FromRow, PgPool};

pub struct PostgresResourceRepository {
    pool: PgPool,
}

impl PostgresResourceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct ResourceRow {
    resource_id: i64,
    resource_type: String, // ← mapped from `type AS resource_type` in SQL
    network_address: String,
    state: String,
    next_state: String,
    finaud_decision: String,
    next_finaud_decision: String,
    state_num: i32,
    suspended_jobs: String,
    scheduler_priority: i32,
    cpuset: String,
    besteffort: String,
    deploy: String,
    expiry_date: i32,
    desktop_computing: String,
    last_job_date: i32,
    available_upto: i32,
    last_available_upto: i32,
    drain: String,
}

#[derive(FromRow)]
struct ResourceLogRow {
    resource_log_id: i64,
    attribute: String,
    value: String,
    date_start: i32,
    date_stop: i32,
    finaud_decision: String,
}

fn parse_state(s: &str) -> ResourceState {
    match s {
        "Alive" => ResourceState::Alive,
        "Dead" => ResourceState::Dead,
        "Suspected" => ResourceState::Suspected,
        "Absent" => ResourceState::Absent,
        _ => ResourceState::Dead,
    }
}

fn state_to_str(s: &ResourceState) -> &'static str {
    match s {
        ResourceState::Alive => "Alive",
        ResourceState::Dead => "Dead",
        ResourceState::Suspected => "Suspected",
        ResourceState::Absent => "Absent",
    }
}

fn parse_next_state(s: &str) -> ResourceNextState {
    match s {
        "Alive" => ResourceNextState::Alive,
        "Dead" => ResourceNextState::Dead,
        "Suspected" => ResourceNextState::Suspected,
        "Absent" => ResourceNextState::Absent,
        _ => ResourceNextState::UnChanged,
    }
}

fn next_state_to_str(s: &ResourceNextState) -> &'static str {
    match s {
        ResourceNextState::UnChanged => "UnChanged",
        ResourceNextState::Alive => "Alive",
        ResourceNextState::Dead => "Dead",
        ResourceNextState::Absent => "Absent",
        ResourceNextState::Suspected => "Suspected",
    }
}

fn row_to_resource(row: ResourceRow, logs: Vec<ResourceLog>) -> Resource {
    Resource {
        id: ResourceId(row.resource_id),
        resource_type: row.resource_type,
        network_address: row.network_address,
        state: parse_state(&row.state),
        next_state: parse_next_state(&row.next_state),
        finaud_decision: row.finaud_decision == "YES",
        next_finaud_decision: row.next_finaud_decision == "YES",
        state_num: row.state_num,
        suspended_jobs: row.suspended_jobs == "YES",
        scheduler_priority: row.scheduler_priority,
        cpuset: row.cpuset,
        besteffort: row.besteffort == "YES",
        deploy: row.deploy == "YES",
        expiry_date: row.expiry_date as i64,
        desktop_computing: row.desktop_computing == "YES",
        last_job_date: row.last_job_date as i64,
        available_upto: row.available_upto as i64,
        last_available_upto: row.last_available_upto as i64,
        drain: row.drain == "YES",
        logs,
    }
}

fn row_to_log(row: ResourceLogRow) -> ResourceLog {
    ResourceLog {
        id: row.resource_log_id,
        attribute: row.attribute,
        value: row.value,
        date_start: row.date_start as i64,
        date_stop: row.date_stop as i64,
        finaud_decision: row.finaud_decision == "YES",
    }
}

impl PostgresResourceRepository {
    async fn fetch_logs(&self, resource_id: i64) -> Result<Vec<ResourceLog>, ResourceError> {
        let rows = sqlx::query_as!(
            ResourceLogRow,
            r#"
            SELECT resource_log_id, attribute, value, date_start, date_stop, finaud_decision
            FROM resource_logs
            WHERE resource_id = $1
            ORDER BY date_start DESC
            "#,
            resource_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_log).collect())
    }
}

#[async_trait]
impl ResourceRepository for PostgresResourceRepository {
    async fn find_by_id(&self, id: &ResourceId) -> Result<Option<Resource>, ResourceError> {
        let row = sqlx::query_as!(
            ResourceRow,
            r#"
            SELECT resource_id, type AS resource_type, network_address, state, next_state,
                   finaud_decision, next_finaud_decision, state_num, suspended_jobs,
                   scheduler_priority, cpuset, besteffort, deploy, expiry_date,
                   desktop_computing, last_job_date, available_upto, last_available_upto, drain
            FROM resources
            WHERE resource_id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(r) => {
                let logs = self.fetch_logs(r.resource_id).await?;
                Ok(Some(row_to_resource(r, logs)))
            }
        }
    }

    async fn find_by_state(&self, state: &ResourceState) -> Result<Vec<Resource>, ResourceError> {
        let rows = sqlx::query_as!(
            ResourceRow,
            r#"
            SELECT resource_id, type AS resource_type, network_address, state, next_state,
                   finaud_decision, next_finaud_decision, state_num, suspended_jobs,
                   scheduler_priority, cpuset, besteffort, deploy, expiry_date,
                   desktop_computing, last_job_date, available_upto, last_available_upto, drain
            FROM resources
            WHERE state = $1
            "#,
            state_to_str(state)
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        let mut resources = Vec::new();
        for row in rows {
            let logs = self.fetch_logs(row.resource_id).await?;
            resources.push(row_to_resource(row, logs));
        }
        Ok(resources)
    }

    async fn find_by_network_address(&self, address: &str) -> Result<Vec<Resource>, ResourceError> {
        let rows = sqlx::query_as!(
            ResourceRow,
            r#"
            SELECT resource_id, type AS resource_type, network_address, state, next_state,
                   finaud_decision, next_finaud_decision, state_num, suspended_jobs,
                   scheduler_priority, cpuset, besteffort, deploy, expiry_date,
                   desktop_computing, last_job_date, available_upto, last_available_upto, drain
            FROM resources
            WHERE network_address = $1
            "#,
            address
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        let mut resources = Vec::new();
        for row in rows {
            let logs = self.fetch_logs(row.resource_id).await?;
            resources.push(row_to_resource(row, logs));
        }
        Ok(resources)
    }

    async fn find_all(&self) -> Result<Vec<Resource>, ResourceError> {
        let rows = sqlx::query_as!(
            ResourceRow,
            r#"
            SELECT resource_id, type AS resource_type, network_address, state, next_state,
                   finaud_decision, next_finaud_decision, state_num, suspended_jobs,
                   scheduler_priority, cpuset, besteffort, deploy, expiry_date,
                   desktop_computing, last_job_date, available_upto, last_available_upto, drain
            FROM resources
            ORDER BY resource_id
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        let mut resources = Vec::new();
        for row in rows {
            let logs = self.fetch_logs(row.resource_id).await?;
            resources.push(row_to_resource(row, logs));
        }
        Ok(resources)
    }

    async fn create(&self, resource: NewResource) -> Result<Resource, ResourceError> {
        let row = sqlx::query_as!(
            ResourceRow,
            r#"
            INSERT INTO resources (
                type, network_address, cpuset, besteffort, deploy,
                desktop_computing, available_upto
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING resource_id, type AS resource_type, network_address, state, next_state,
                      finaud_decision, next_finaud_decision, state_num, suspended_jobs,
                      scheduler_priority, cpuset, besteffort, deploy, expiry_date,
                      desktop_computing, last_job_date, available_upto, last_available_upto, drain
            "#,
            resource.resource_type,
            resource.network_address,
            resource.cpuset,
            if resource.besteffort { "YES" } else { "NO" },
            if resource.deploy { "YES" } else { "NO" },
            if resource.desktop_computing {
                "YES"
            } else {
                "NO"
            },
            resource.available_upto as i32,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        Ok(row_to_resource(row, vec![]))
    }

    async fn update_state(
        &self,
        id: &ResourceId,
        state: ResourceState,
    ) -> Result<(), ResourceError> {
        let result = sqlx::query!(
            r#"UPDATE resources SET state = $1 WHERE resource_id = $2"#,
            state_to_str(&state),
            id.0
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ResourceError::NotFound(id.clone()));
        }
        Ok(())
    }

    async fn update_next_state(
        &self,
        id: &ResourceId,
        next_state: ResourceNextState,
    ) -> Result<(), ResourceError> {
        let result = sqlx::query!(
            r#"UPDATE resources SET next_state = $1 WHERE resource_id = $2"#,
            next_state_to_str(&next_state),
            id.0
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ResourceError::NotFound(id.clone()));
        }
        Ok(())
    }

    async fn delete(&self, id: &ResourceId) -> Result<(), ResourceError> {
        let result = sqlx::query!(r#"DELETE FROM resources WHERE resource_id = $1"#, id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| ResourceError::InfrastructureError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ResourceError::NotFound(id.clone()));
        }
        Ok(())
    }
}
