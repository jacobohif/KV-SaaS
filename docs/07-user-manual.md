# 7. User Manual

## **Introduction**

Welcome to [Your SaaS Product Name]!

This manual is designed to provide you with clear and comprehensive operational guidance. Whether you are a **Tenant Admin** responsible for your entire company's account, a **Regular User** utilizing our core features, or a **Super Admin** in charge of platform operations, you will find the information you need here.

**Manual Structure**:
*   **Part 1: Tenant Admin Guide**: Aimed at the managers of customer companies, this section explains how to manage accounts, users, subscriptions, and branding.
*   **Part 2: Regular User Guide**: For all end-users, this section explains how to log in, use core features, and manage personal settings.
*   **Part 3: Super Admin Guide**: For internal employees of the SaaS provider, this section explains how to perform platform-level management and maintenance.
*   **Part 4: Frequently Asked Questions (FAQ)**: Answers to some common questions.

---

## **Part 1: Tenant Admin Guide**

This section's content is visible and operable only by the **Tenant Admin**.

### **1. Quick Start: Registration and Initialization**

1.  **Register an Account**: Click "Free Trial" on our official website, fill in your name, work email, and company name, and set a password.
2.  **Verify Your Email**: Click the link in the verification email we send you to activate your account.
3.  **First Login & Initialization Wizard**:
    *   Log in for the first time using your email and password.
    *   The system will guide you through the initial setup:
        *   **Complete Company Information**: Fill in your company details.
        *   **Upload Brand Logo**: Upload your logo, which will be displayed at the top of your team's workspace. `[Screenshot Placeholder: Logo upload interface]`
        *   **Invite Team Members**: Enter your colleagues' email addresses and assign them roles (e.g., "Admin" or "Member").

### **2. User and Permission Management**

*   **Invite New Users**:
    1.  Go to "User Management" in the left-side navigation bar.
    2.  Click the "Invite User" button, enter the invitee's email, and select a role. `[Screenshot Placeholder: Invite user pop-up]`
*   **Manage Users**:
    *   In the user list, you can **edit** a user's role or **disable/enable** a user. A disabled user will not be able to log in.

### **3. Subscription and Billing**

*   **View and Change Plan**:
    *   On the "Subscription & Billing" page, you can view your current plan, price, and renewal date.
    *   Click the "Change Plan" button to upgrade or downgrade between different service plans. `[Screenshot Placeholder: Service plan comparison table]`
*   **Payment and Invoices**:
    *   You can manage your payment methods (e.g., credit card) on this page.
    *   In the "Billing History," you can view and download PDF files of all past invoices.

### **4. Audit Logs**

*   On the "Audit Logs" page, you can track key actions performed by all users within your account, such as "User Login," "Project Creation," and "Member Invitation," for security auditing purposes.

---

## **Part 2: Regular User Guide**

This section is intended for all **Regular Users** within a tenant.

### **1. Login and Access**

*   **Accept Invitation**: You will receive an invitation email from your administrator. Click the link in the email and set your password to complete registration.
*   **Login**: Visit the platform's login page and enter your email and password to log in.
    *   If you forget your password, you can click the "Forgot Password" link on the login page to reset it. `[Screenshot Placeholder: Login page card]`

### **2. Using Core Features**

*(Note: This section should be filled in with details about your SaaS product's core business. The following is an example for a project management tool.)*

*   **Navigation**: After logging in, you will see the main interface. The left side is the feature navigation bar, which includes modules like "Dashboard," "Projects," and "Tasks."
*   **Project Operations**:
    *   On the "Projects" page, you can view all the projects you are a part of.
    *   Click on a project name to go to the project details page, where you can see all the tasks under that project.
*   **Task Management**:
    *   On the project details page, you can create new tasks or update the status of existing tasks from "To Do" to "In Progress" or "Done." `[Screenshot Placeholder: Kanban-style task management interface]`

### **3. Personal Settings**

*   Click on your avatar in the top-right corner of the interface and select "Personal Settings" from the dropdown menu.
*   Here, you can:
    *   Change your name and avatar.
    *   Change your login password.

---

## **Part 3: Super Admin Guide**

This section's content can only be operated by a **Super Admin** in the backend management system.

### **1. Tenant Management**

*   **View Tenants**: In the "Tenant Management" module, you can search for and view a list of all tenants and their statuses (e.g., `active`, `trial`, `suspended`).
*   **Manual Operations**:
    *   You can manually **activate** a newly registered tenant or **suspend/terminate** a tenant for violations.
    *   You can manually adjust the subscription plan or resource quotas for a specific tenant.
*   **Impersonate Login**:
    *   To help customers solve problems, you can use the "Impersonate" feature to temporarily log into their account as a tenant admin to investigate issues. All impersonation actions are strictly logged by the system.

### **2. Service Plan Configuration**

*   In the "Service Plans" module, you can create or edit different subscription packages (e.g., `Free`, `Basic`, `Premium`).
*   You can define the price, resource quotas (like user count, storage GBs), and available feature modules for each package.

---

## **Part 4: Frequently Asked Questions (FAQ)**

*   **Q: Is our data secure?**
    *   **A**: Absolutely. We use strict multi-tenant isolation technology to ensure your data is completely isolated from other tenants, both physically and logically. All data is encrypted with high-strength encryption during transit and at rest.

*   **Q: What if I need help?**
    *   **A**: You can click the "Help" button in the top-right corner of the interface to access our knowledge base, or contact our customer support team through the "Support" page.