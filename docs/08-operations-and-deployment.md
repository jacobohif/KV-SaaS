# 8. Operations & Maintenance Documentation

This manual provides guidance for the operations and technical support teams of the SaaS platform, covering key operations such as deployment, monitoring, maintenance, and disaster recovery.

---

### **1. System Deployment**

#### **1.1 Deployment Architecture and Tools**

*   **Containerization**: All services are packaged as **Docker** images.
*   **Container Orchestration**: The production environment uses **Kubernetes (K8s)** for deployment and management.
*   **Configuration Management**: **Helm Charts** are used for versioned management of K8s deployment configurations.
*   **Image Registry**: Use a private/public image registry like **Docker Hub**, **AWS ECR**, or **Harbor**.

#### **1.2 CI/CD Deployment Flow**

```mermaid
graph TD
    subgraph "Development Phase"
        A[1. Developer commits code to a feature branch] --> B{2. Create a Pull Request};
    end

    subgraph "Automated CI/CD Pipeline (GitHub Actions / GitLab CI)"
        B --> C[3. CI is automatically triggered];
        C --> D["- Run Unit Tests<br>- Run Integration Tests<br>- Code Quality Scan (Clippy)"];
        D -- Fails --> X[Notify developer to fix];
        D -- Succeeds --> E[4. Build Docker Image];
        E --> F[5. Push Image to Registry];
        F --> G[6. Automatically deploy to<br><b>Testing (QA) Environment</b>];
        G --> H[- Run automated E2E tests];
        H -- Fails --> X;
    end

    subgraph "Release Phase"
        H -- Succeeds --> I{7. <b>Manual Gate:</b><br>QA team/Product Owner<br>verifies on QA environment};
        I -- Not Approved --> X;
        I -- Approved --> J[8. Automatically deploy to<br><b>Staging Environment</b>];
        J --> K{9. <b>Manual Gate:</b><br>Perform final regression testing<br>or demo to stakeholders};
        K -- Not Approved --> X;
        K -- Approved --> L[10. <b>Manually Trigger</b><br>deployment to <b>Production</b>];
        L --> M[11. Use <b>Canary Release</b><br>to gradually shift traffic to the new version];
    end

    classDef manual fill:#F59E0B,color:#fff,stroke:#B45309;
    class I,K,L manual;
```

#### **1.3 Key Operation Command Examples**

*   **Build and Push Image**:
    ```bash
    # 1. Log in to the image registry
    docker login your-registry.com

    # 2. Build the image (assuming Dockerfile is in the current directory)
    # Use the Git commit hash as the tag for traceability
    export IMAGE_TAG=$(git rev-parse --short HEAD)
    docker build -t your-registry.com/your-app:$IMAGE_TAG .

    # 3. Push the image
    docker push your-registry.com/your-app:$IMAGE_TAG
    ```

*   **Deploy or Upgrade with Helm**:
    ```bash
    # Assume Helm Chart is located at ./charts/your-app
    # Update Helm dependencies
    helm dependency update ./charts/your-app

    # Deploy or upgrade the application
    # --install: install if the release does not exist
    # -n: specify the namespace
    # -f: specify a custom values file
    helm upgrade --install your-app-release ./charts/your-app \
      -n your-namespace \
      -f ./charts/your-app/values.staging.yaml \
      --set image.tag=$IMAGE_TAG
    ```

*   **Check Deployment Status**:
    ```bash
    # Check Pod status
    kubectl get pods -n your-namespace

    # View application logs (select a Pod)
    kubectl logs -f your-app-pod-name -n your-namespace

    # If a Pod fails to start, describe it to troubleshoot
    kubectl describe pod your-app-pod-name -n your-namespace
    ```

---

### **2. Monitoring, Logging, and Alerting**

#### **2.1 Monitoring System and Core Dashboards**

*   **Tool Stack**: **Prometheus** (metrics collection) + **Grafana** (visualization).
*   **Core Philosophy**: Monitoring should serve to quickly detect problems, locate their causes, and plan for capacity. We create dedicated dashboards for different levels.

*   **Grafana Dashboard 1: Global Business Overview**
    *   **Purpose**: For management and product owners to quickly understand the platform's health.
    *   **Key Metrics**:
        *   `Stat`: Total active tenants, total users, current total Monthly Recurring Revenue (MRR).
        *   `Graph`: New user/tenant growth trend (daily/weekly).
        *   `Table`: Top 10 most active tenants (by API call count).

*   **Grafana Dashboard 2: Application Performance Monitoring (APM)**
    *   **Purpose**: For development and operations teams to monitor application health.
    *   **Key Metrics (RED Method)**:
        *   **Rate**: QPS (Queries Per Second) for each API endpoint.
        *   **Errors**: HTTP 5xx and 4xx error rates.
        *   **Duration**: P99, P95, P50 quantiles of API response times.
        *   **Others**: Application CPU/memory usage, database connection pool status.

*   **Grafana Dashboard 3: Infrastructure Monitoring**
    *   **Purpose**: For the operations team to monitor underlying resources.
    *   **Key Metrics**:
        *   **K8s Cluster**: Node/Pod health status, CPU/memory/disk usage.
        *   **PostgreSQL**: Connection count, active queries, number of slow queries, replication lag.
        *   **Redis**: Memory usage, hit rate, connection count.

#### **2.2 Log Management**

*   **Tool Stack**: **Loki** + **Fluentd** (recommended for high integration with Prometheus/Grafana ecosystem) or **ELK Stack**.
*   **Log Format**: All application logs must be output to standard output (stdout) in **JSON** format.
*   **Core Fields**: Every log entry must include `timestamp`, `level`, `service_name`, `trace_id`, and `tenant_id` (if applicable) for easy filtering and analysis.
*   **Querying**: Operations personnel can perform efficient log queries through Grafana Explore or Kibana.

#### **2.3 Alert Management**

*   **Tool**: **Alertmanager**.
*   **Alert Tiers**:
    *   **P1 (Urgent/Wake-up Alert)**: Requires immediate action, notifies on-call engineers via phone call or SMS.
        *   *Example Rule*: Production API error rate > 5% for 5 minutes; primary database is down.
    *   **P2 (Warning)**: Needs attention but not immediate action, sent to the Slack alert channel.
        *   *Example Rule*: Deployment to staging environment failed; node disk space will be exhausted in 24 hours.
    *   **P3 (Informational)**: Routine information, such as successful deployment notifications.
*   **Notification Channels**: PagerDuty (P1), Slack (P2), Email (P3).

---

### **3. Standard Operating Procedures (SOPs)**

This section defines standardized procedures for key operational tasks to ensure accuracy, repeatability, and safety.

#### **SOP-01: Database Backup and Recovery Drill**

*   **Purpose**: To verify the validity of database backups and ensure the team is familiar with the recovery process.
*   **Frequency**: Once per quarter.
*   **Procedure**:
    1.  **Preparation**:
        *   Obtain the latest full database backup file from the production environment.
        *   Prepare a temporary, isolated K8s namespace and a new PostgreSQL instance.
    2.  **Execution**:
        *   Restore the backup file to the new PostgreSQL instance.
        *   Record the time taken for the restoration process.
    3.  **Verification**:
        *   Start a temporary application instance connected to the restored database.
        *   Execute a series of predefined read-only API calls to verify that core data (e.g., number of users, projects) is complete and consistent.
        *   Randomly sample data from several tenants for inspection.
    4.  **Cleanup**:
        *   Destroy the temporary environment and the restored database instance.
    5.  **Reporting**:
        *   Write a drill report, documenting the recovery time, verification results, and any issues encountered.

#### **SOP-02: Manual Primary-Replica Failover for Production Database**

*   **Scenario**: The primary database instance suffers an irreversible failure, and automatic failover has failed.
*   **Prerequisites**:
    *   A P1 alert "Primary database unavailable" has been received.
    *   It has been confirmed that the data replication lag of the replica is within an acceptable range (< 1 minute).
*   **Procedure**:
    1.  **Declare Incident**: Announce the start of the database failover in the #oncall Slack channel and create an incident war room.
    2.  **Isolate Application Layer**: Temporarily scale down the application deployment replicas to 0 or block application access to the database via network policies to prevent dirty writes.
    3.  **Promote Replica**: Execute the "promote replica to new primary" command provided by the cloud service provider or database management tool.
    4.  **Update Connection String**: In the application's configuration center (e.g., K8s ConfigMap/Secret), update the database connection address to the new primary's address.
    5.  **Restore Application**: Scale the application replicas back to their normal count.
    6.  **Functional Verification**:
        *   Observe the Grafana dashboard to confirm that the API error rate has returned to normal.
        *   Manually test core functions (e.g., login, creating data) to ensure they are working correctly.
    7.  **Declare End of Incident**: Announce the end of the incident in the war room and schedule a post-mortem.

### **4. Disaster Recovery (DR)**

*   **Core Objectives**:
    *   **RPO (Recovery Point Objective)**: < 15 minutes.
    *   **RTO (Recovery Time Objective)**: < 1 hour.
*   **Architectural Safeguards**:
    *   **Multi-AZ Deployment**: All components of the production environment (K8s nodes, database instances) are deployed across multiple Availability Zones (AZs).
    *   **Infrastructure as Code (IaC)**: Use Terraform or Pulumi to manage all cloud resources, ensuring the entire environment can be rebuilt quickly and repeatably.
*   **Recovery Plans**:
    *   **Scenario 1: Single Pod/Node Failure**: Handled automatically by K8s, no manual intervention required.
    *   **Scenario 2: Single Availability Zone (AZ) Failure**:
        *   **Plan**: The K8s scheduler will automatically recreate affected Pods in healthy AZs. The Load Balancer's health checks will automatically remove traffic from the failed AZ.
        *   **Manual Intervention Needed**: Check if the database has automatically completed a primary-replica failover. If not, execute **SOP-02**.
    *   **Scenario 3: Regional Failure (Extreme Case)**:
        *   **Plan**: Initiate the off-site disaster recovery process, using off-site backups to rebuild the entire environment in another region. This is a high-cost, complex process, and its implementation depends on the business's Service Level Agreement (SLA).