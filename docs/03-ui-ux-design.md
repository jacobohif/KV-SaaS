# 3. UI/UX Design Documentation

This document provides unified guiding principles, a design system, and key interface design points for the system's UI/UX, ensuring a clear, consistent, and efficient user experience for all user roles.

---

### **1. User Personas**

To ensure targeted design decisions, we define three core user personas:

*   **Super Admin**:
    *   **Identity**: Internal employee of the SaaS provider (technical support, operations, business).
    *   **Core Needs**: Monitor the overall health of the platform, manage all tenants, handle global configurations and troubleshooting.
    *   **Use Cases**: Viewing the system overview dashboard, manually activating or disabling tenants, configuring service plans, resolving issues reported by tenants.
    *   **Design Focus**: High information density, powerful functionality, operational efficiency.

*   **Tenant Admin**:
    *   **Identity**: IT manager or business lead of a customer company.
    *   **Core Needs**: Manage their own company's account, subscription, users, and branding settings.
    *   **Use Cases**: Inviting/managing team members, upgrading/downgrading service plans, viewing bills, customizing the company logo and theme color.
    *   **Design Focus**: Self-service, clear workflows, intuitive information.

*   **Tenant User**:
    *   **Identity**: General employee of a customer company.
    *   **Core Needs**: Efficiently use the core business functions of the SaaS product to complete daily tasks.
    *   **Use Cases**: Logging into the core business application, manipulating business data, collaborating with team members.
    *   **Design Focus**: Simplicity, focus on core tasks, minimal distractions.

### **2. Design System Foundations**

To ensure product consistency, all interfaces must adhere to the following design system foundations.

*   **Color Palette**:
    *   **Primary**: `#4F46E5` (Indigo) - Used for key action buttons, links, navigation highlights, etc.
    *   **Secondary**: `#6B7280` (Neutral Gray) - Used for secondary text, borders, backgrounds.
    *   **Success**: `#10B981` (Green) - Used for success messages, validation passes.
    *   **Warning**: `#F59E0B` (Yellow) - Used for warnings, tips requiring user attention.
    *   **Danger**: `#EF4444` (Red) - Used for error messages, dangerous actions (like deletion).
    *   **Background**: `#F9FAFB` (Light Gray) - Used for the main page background to provide a soft visual effect.

*   **Typography**:
    *   **Font**: Prioritize system-default sans-serif fonts like `Inter`, `-apple-system`, `BlinkMacSystemFont`, `"Segoe UI"`, `Roboto`, `"Helvetica Neue"`, `Arial`, `sans-serif`.
    *   **Font Sizes**:
        *   Heading 1 (H1): `2.25rem` (36px)
        *   Heading 2 (H2): `1.875rem` (30px)
        *   Heading 3 (H3): `1.5rem` (24px)
        *   Body: `1rem` (16px)
        *   Caption: `0.875rem` (14px)

*   **Core Component Library**:
    *   All reusable UI elements (e.g., buttons, inputs, modals, cards) should be standardized based on **Ant Design** or **Material-UI**.
    *   **Button**: Includes primary, secondary, text, and danger styles.
    *   **Input**: Includes label, placeholder, helper text, and error states.
    *   **Card**: Used to contain information modules, should have uniform rounded corners (e.g., `8px`) and shadow effects.

### **3. Core Design Principles**

*   **Self-Service**: The design must be centered around enabling tenants to complete operations independently and easily, reducing reliance on customer support. Workflows should be clear, intuitive, and provide necessary guidance.
*   **Consistency**: The visual style, interaction patterns, and terminology should be highly consistent across the entire platform (including the customer portal and core application).
*   **Clarity**: The interface should be simple and clear, avoiding information overload. Important information and actions should be highlighted. Key data like quota usage and billing must be visualized.
*   **Branding & Personalization**: Allowing tenants a degree of branding customization is a crucial part of enhancing the value of a SaaS product.
*   **Responsive Design**: All interfaces must provide a good user experience on devices of different sizes (desktop, tablet, mobile).

### **4. Main User Flows**

*   **New Tenant Onboarding Flow**:
    1.  User clicks "Sign Up" or "Free Trial" on the official website.
    2.  Fills in basic information (name, email, company name) and creates a password.
    3.  Email verification.
    4.  After the first login, an "Initialization Wizard" is triggered to guide the user:
        *   Fill in detailed company information.
        *   Upload the company logo.
        *   Set the default timezone and language.
        *   Invite the first batch of team members.
    5.  After completing the wizard, the user is taken to the main customer portal dashboard.

*   **Plan Change Flow**:
    1.  The tenant admin clicks "Change Plan" on the "Subscription & Billing" page.
    2.  The system displays a clear table comparing the features and prices of different plans.
    3.  The user selects a new plan (upgrade or downgrade).
    4.  If upgrading, the user is guided to the payment page to complete the pro-rated payment.
    5.  If downgrading, the user is clearly informed that the change will take effect in the next billing cycle.
    6.  Upon completion, the system sends a confirmation notification.

### **5. Key Interface Designs (Wireframe Concepts)**

#### **5.1 Customer Portal**

This is where tenant admins manage their accounts. The focus should be on clear information presentation and ease of operation.

*   **Dashboard**:
    *   **Key Metric Cards**: Highlight critical data such as: current subscription plan, quota usage (Users: 5/10, Storage: 20GB/50GB), current billing cycle costs.
    *   **Quick Action Links**: "Invite User," "View Bills," "Upgrade Plan," etc.
    *   **Recent Activity**: Display a summary of recent audit logs.
    *   **System Notifications**: Show important system announcements or alerts.

*   **User Management Page**:
    *   Display all internal users in a list, including name, email, role, and status (active/disabled).
    *   Provide search and filtering functionality.
    *   A prominent "Invite User" button that opens a modal to enter the invitee's email and select a role.
    *   An action menu for each user row (edit role, disable/enable, delete).

*   **Subscription & Billing Page**:
    *   **Current Plan Module**: Clearly display the current plan name, price, billing cycle, and a "Change Plan" button.
    *   **Payment Methods Module**: List linked payment methods and allow adding or removing them.
    *   **Billing History Module**: List all past bills in a table, including date, amount, status (paid/pending), and a link to download the invoice.

*   **Branding Settings Page**:
    *   Provide a logo upload control with a real-time preview.
    *   Provide a theme color picker to allow tenants to customize the primary color of the interface.

#### **5.2 Core Business Application**

This is the product that tenants use daily. The design should focus on efficiency and smooth business workflows.

*   **Navigation Bar**:
    *   A fixed top or left navigation bar that clearly organizes core functional modules.
    *   The navigation bar should include the tenant's logo to reinforce branding.
    *   The top-right corner usually contains a user avatar menu (personal settings, logout) and a help center entry point.

*   **Data Tables/Lists**:
    *   All business data lists should support sorting, filtering, and searching.
    *   Support paginated loading.
    *   Provide bulk action capabilities (e.g., bulk delete, bulk status change).

*   **Forms**:
    *   Input fields should have clear labels and necessary helper text/validation rules.
    *   Primary action buttons like Save and Cancel should be in a fixed and easily discoverable location.
    *   For complex forms, a step-by-step wizard can be used to reduce the user's cognitive load.

---

### **6. Prototyping Tools**

*   It is recommended to use professional tools like **Figma**, **Sketch**, or **Adobe XD** for high-fidelity prototype design, so that the development and testing teams can accurately understand the design intent.