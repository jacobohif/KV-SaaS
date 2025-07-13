use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            -- Enable UUID generation
            CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

            -- Table: tenants
            CREATE TABLE tenants (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                name VARCHAR(255) NOT NULL,
                status VARCHAR(50) NOT NULL DEFAULT 'trial',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            -- Table: plans
            CREATE TABLE plans (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                name VARCHAR(100) NOT NULL,
                price INT NOT NULL DEFAULT 0,
                user_limit INT NOT NULL DEFAULT 1,
                storage_limit_gb INT NOT NULL DEFAULT 1,
                features JSONB
            );

            -- Table: users
            CREATE TABLE users (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
                email VARCHAR(255) NOT NULL,
                password_hash VARCHAR(255) NOT NULL,
                status VARCHAR(50) NOT NULL DEFAULT 'invited',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE(tenant_id, email)
            );

            -- Table: subscriptions
            CREATE TABLE subscriptions (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
                plan_id UUID NOT NULL REFERENCES plans(id),
                status VARCHAR(50) NOT NULL,
                current_period_start TIMESTAMPTZ NOT NULL,
                current_period_end TIMESTAMPTZ NOT NULL
            );

            -- Table: roles
            CREATE TABLE roles (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
                name VARCHAR(100) NOT NULL
            );

            -- Table: permissions
            CREATE TABLE permissions (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                name VARCHAR(100) NOT NULL UNIQUE,
                description TEXT
            );

            -- Table: user_roles (Many-to-Many)
            CREATE TABLE user_roles (
                user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                PRIMARY KEY (user_id, role_id)
            );

            -- Table: role_permissions (Many-to-Many)
            CREATE TABLE role_permissions (
                role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
                permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
                PRIMARY KEY (role_id, permission_id)
            );

            -- Table: audit_logs
            CREATE TABLE audit_logs (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
                user_id UUID NOT NULL REFERENCES users(id),
                action VARCHAR(100) NOT NULL,
                details JSONB,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            -- Table: projects (Example Business Table)
            CREATE TABLE projects (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
                name VARCHAR(255) NOT NULL,
                created_by_user_id UUID NOT NULL REFERENCES users(id),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            -- Table: tasks (Example Business Table)
            CREATE TABLE tasks (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
                project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                title VARCHAR(255) NOT NULL,
                assignee_user_id UUID REFERENCES users(id),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );

            -- Add indexes for foreign keys and common lookups
            CREATE INDEX idx_users_tenant_id ON users(tenant_id);
            CREATE INDEX idx_subscriptions_tenant_id ON subscriptions(tenant_id);
            CREATE INDEX idx_roles_tenant_id ON roles(tenant_id);
            CREATE INDEX idx_audit_logs_tenant_id ON audit_logs(tenant_id);
            CREATE INDEX idx_projects_tenant_id ON projects(tenant_id);
            CREATE INDEX idx_tasks_tenant_id ON tasks(tenant_id);
            CREATE INDEX idx_tasks_project_id ON tasks(project_id);
        "#;
        
        let db = manager.get_connection();
        db.execute_unprepared(sql).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE IF EXISTS tasks;
            DROP TABLE IF EXISTS projects;
            DROP TABLE IF EXISTS audit_logs;
            DROP TABLE IF EXISTS role_permissions;
            DROP TABLE IF EXISTS user_roles;
            DROP TABLE IF EXISTS permissions;
            DROP TABLE IF EXISTS roles;
            DROP TABLE IF EXISTS subscriptions;
            DROP TABLE IF EXISTS users;
            DROP TABLE IF EXISTS plans;
            DROP TABLE IF EXISTS tenants;
        "#;

        let db = manager.get_connection();
        db.execute_unprepared(sql).await?;

        Ok(())
    }
}