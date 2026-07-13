# Software Requirements Document

|Properties|Values|
|---|---|
|Project Name|FlowState|
|Author Name|Bengi Mizrahi|
|Author Email|bengimizrahi@gmail.com|

## 1. Executive Summary

### Problem Statement


#### Current Situation

Release management for complex software programs is currently performed using Atlassian Jira as the primary system of record. Jira provides strong issue-tracking capabilities, including support for epics, stories, tasks, assignments, due dates, sprints, work logs, comments, and workflow management. While these capabilities are effective for tracking individual work items, Jira is not well suited for planning and managing releases from a timeline and dependency perspective.

Existing Jira extensions such as Roadmaps, Tempo, and Structure provide partial solutions, but they are often cumbersome to use, insufficiently responsive, and do not offer the level of visibility required for effective release planning and execution.


#### Challenges

Release managers need to understand how work items relate to one another, identify dependencies, visualize delivery timelines, and determine critical paths that may impact release commitments. Current tools make it difficult to answer questions such as:

- Which work items are on the critical path?
- Which delays can jeopardize the release date?
- What is the current delivery risk?
- Is the team progressing at a pace that supports the planned release date?
- Which mitigation actions should be taken when delays occur?

As projects grow in size and complexity, manually identifying these risks becomes increasingly difficult and time-consuming.


#### Business Impact

Without accurate visibility into dependencies, critical paths, and delivery risk, release managers are forced to rely on manual analysis, personal experience, and fragmented reporting. This increases the likelihood of missed release commitments, late identification of schedule risks, inefficient resource allocation, and reactive decision-making.

Stakeholders may not receive sufficient early warning when delivery objectives are at risk, reducing the organization's ability to implement corrective actions before schedule slippage occurs.


#### Desired Outcome

The organization needs a release-management solution that uses Jira as the authoritative data source while augmenting it with a full-featured Gantt chart and planning experience comparable to professional project scheduling tools.

FlowState is delivered as a **web-native application** — accessed through a modern browser — enabling rapid UI iteration and continuous evolution without the release friction of native desktop clients.

The solution should:

- Synchronize with Jira and leverage existing Jira entities and relationships.
- Provide interactive Gantt chart visualization for the full Nokia issue hierarchy — from System Item through Epic down to User Stories and Tasks, plus Bugs.
- Model and visualize dependencies between work items.
- Automatically calculate and highlight critical paths.
- Continuously evaluate delivery risk based on actual team progress, historical velocity, work completion trends, and remaining scope.
- Predict the likelihood of meeting release milestones and target release dates.
- Alert users when delivery risks emerge.
- Recommend mitigation actions such as scope reduction, reprioritization, dependency resolution, resource adjustments, or schedule changes.
- Enable release managers to make proactive, data-driven decisions with a clear understanding of timeline impacts and project health.

The desired result is a release planning and tracking platform that combines Jira's issue-management strengths with advanced project scheduling, forecasting, risk analysis, and decision-support capabilities.

### Vision

FlowState is the operational command center for release managers, product owners, and delivery leaders.

While Jira remains the authoritative system for tracking work items, FlowState becomes the primary tool for planning, monitoring, forecasting, and steering software releases. It transforms Jira data into actionable delivery intelligence, enabling managers to focus on decisions rather than data collection and manual analysis.

FlowState continuously evaluates the health of a release by monitoring progress, dependencies, timelines, resource activity, and delivery trends. Instead of requiring managers to search for risks, the platform proactively identifies emerging issues, explains their potential impact, and recommends corrective actions before milestones are missed.

The platform acts as both a defensive shield and an offensive planning tool:

- It continuously scans for delivery risks, schedule slippage, dependency bottlenecks, inactive work streams, missing updates, and deviations from plan.
- It actively proposes mitigation strategies, schedule optimizations, resource adjustments, and scope changes that improve the probability of successful delivery.

FlowState is designed to function as an always-on release copilot. Every day, it analyzes live project data and asks critical questions that release managers would normally need to investigate manually:

- Are we still on track for the release date?
- Which work items are currently on the critical path?
- What has changed since yesterday?
- Which dependencies are blocking progress?
- Which work items are at risk of delay?
- Which project risks require immediate attention?
- What actions should be taken to increase confidence in delivery?

The platform detects operational anomalies and planning issues automatically, including:

- Work items that should have started but remain unopened.
- Work items that have reached or exceeded their target dates.
- Dependencies that threaten upcoming work.
- Missing work logs or status updates.
- Progress trends that indicate likely delays.
- Resource or team activity patterns that deviate from expectations.

When risks are identified, FlowState provides practical recommendations and can optionally synchronize approved plan updates back to Jira, ensuring alignment between planning and execution.

Success is achieved when release managers no longer spend their time gathering information, updating plans, and searching for risks. Instead, they use FlowState to gain immediate situational awareness, understand the most likely delivery outcome, and confidently take the actions necessary to ensure successful releases.

### Business Goals

#### Goal 1: Predictable and Successful Release Delivery

Enable release managers to consistently deliver releases on or before planned target dates by providing visibility into critical paths, dependencies, delivery risks, and schedule confidence.

Success criteria:

- Early identification of release risks before they become critical.
- Significant reduction in release delays.
- Improved release date predictability.
- Higher percentage of releases delivered on time.

#### Goal 2: Real-Time and Accurate Delivery Visibility

Provide a continuously updated and trustworthy representation of the actual state of delivery by synchronizing with Jira and reflecting project progress without loss, distortion, or manual interpretation.

Success criteria:

- Release status always reflects the current state of Jira data.
- Managers can understand project health without manual data collection.
- Dependencies, progress, blockers, and schedule impacts are visible at all times.
- Teams and stakeholders operate from a single, accurate view of delivery status.

#### Goal 3: Proactive Attention Management

Ensure that no significant delivery risk, anomaly, blocker, or schedule deviation goes unnoticed by continuously monitoring project activity and providing timely alerts, recommendations, and follow-up actions.

Success criteria:

- Critical issues are detected automatically.
- Managers receive actionable notifications before delivery dates are impacted.
- Blockers, dependency problems, and stalled work are surfaced immediately.
- Suggested mitigation actions are provided whenever risks are identified.
- The system becomes the primary mechanism through which managers maintain situational awareness of release health.


### Out of Scope

FlowState is not intended to replace Jira as the system of record for day-to-day issue management.

Out of scope:

- Creating a full alternative issue-tracking platform
- Managing workflows independent of Jira
- Replacing Jira boards, backlogs, or sprint planning
- Replacing Jira Query Language (JQL) for issue search, bug scrubbing, or bulk triage — users continue to use Jira for those workflows

FlowState focuses on release planning, delivery visibility, and risk management.

Out of scope:

- Source code management
- Continuous integration / continuous deployment (CI/CD)
- Test execution management
- Build orchestration

FlowState may analyze delivery capacity and team activity but is not intended to become a workforce management tool.

Out of scope:

- Payroll integration

FlowState may generate notifications and suggested communications but is not intended to replace collaboration tools.

Out of scope:

- Email platform replacement
- Instant messaging platform replacement
- Team chat functionality

Out of scope:

- Budget planning
- Cost accounting
- Financial forecasting
- Invoice management

The first version will focus on individual releases and delivery programs.

Out of scope:

- Enterprise portfolio optimization
- Strategic investment planning
- Cross-business-unit portfolio balancing
- Native desktop client application (V1 is **web-only**; see §7 User Interface, §12 Technology Constraints)

FlowState does not replace Jira. It augments Jira with planning, forecasting, risk detection, release intelligence, and decision support.

# 2. Stakeholders

## Business Owner

Organizations and teams responsible for planning, managing, and delivering software releases.

Typical examples:

- Product Management organizations
- Engineering organizations
- Program Management organizations

## End Users

Any project participant involved in planning, tracking, executing, or monitoring software delivery.

Examples:

- Release Managers
- Product Managers
- Product Owners
- Program Managers
- Scrum Masters
- Developers
- Test Engineers
- QA Leads
- UX/UI Designers
- Architects
- Engineering Managers
- Delivery Managers

All users are expected to work from the same project data and release plan.

End users are members of the internal delivery organization. Customers, external partners, and non-delivery business stakeholders are excluded (see Other Stakeholders).

## Administrators

Administrators are responsible only for technical configuration of the platform.

Typical responsibilities:

- Jira connectivity configuration
- API token management
- System settings
- Notification settings
- AI provider configuration
- Data synchronization settings

Administrators are not responsible for managing business-level access permissions.

## Other Stakeholders

FlowState is an internal delivery operations tool. It is not intended for customers, external partners, or business stakeholders who are outside the delivery organization.

Individuals and groups listed here may care about release outcomes, but they are **not** intended users of FlowState. They receive delivery information through deliberate, curated communication channels — not through direct access to operational planning data, risk analysis, or mitigation recommendations.

### Executive Leadership

Senior leaders who need awareness of release health and delivery outcomes.

They are not expected to use FlowState as a day-to-day tool. When visibility is required, it should be provided through summaries, reports, or presentations prepared by the delivery team — not through live access to internal risk signals, critical-path analysis, or draft mitigation plans.

### PMO and Program Governance (Internal)

Internal program or portfolio governance roles that may require periodic status input.

They are not primary FlowState users. Any reporting they receive should be based on curated exports or summaries, not unrestricted access to operational delivery intelligence.

### Out of Scope as FlowState Users

The following are explicitly **not** intended users of FlowState:

- Customers
- External partners
- Business stakeholders outside the delivery team
- Vendors or contractors without a direct delivery execution role

### External Communication Principle

Outward-facing delivery communication — release updates, milestone announcements, stakeholder briefings — is a separate concern from FlowState usage.

Release managers and delivery leads control what is shared externally, when it is shared, and in what form. FlowState supports internal decision-making; it does not replace judgment about what internal operational detail should be disclosed outside the organization.

# 3. User Types

FlowState distinguishes user types by **what each role needs to do**, not by what data they can see. All internal delivery participants share the same view of release plans, schedules, dependencies, risks, and forecasts. Differences between user types are limited to a small set of action-oriented capabilities — such as logging work or approving automated remediation suggestions.

## User Type: Developer

### Description

Developers execute assigned work within a release. They need clear visibility into their own commitments, how their work fits into the broader release timeline, and how delays or extra work affect delivery outcomes. FlowState should be a fast, practical daily tool — not only a planning view for managers.

### Goals

- Understand from the outset what work they have committed to for the release, including dates, dependencies, and downstream impact.
- See how their current progress compares to the plan without digging through Jira views and spreadsheets.
- Log work quickly and accurately so release health reflects reality.
- Understand when other team members are waiting on their work.
- Stay ahead of schedule pressure instead of discovering overload only near code freeze.

### Pain Points

- Developers are often not fully aware of everything they have implicitly promised when they accept work items, help colleagues, or pick up additional bug fixes along the way.
- Small additions feel manageable in isolation, but they accumulate and create unexpected pressure as the code freeze or release date approaches.
- It is difficult to see the combined effect of primary assignments, dependencies, and ad-hoc work on personal delivery risk.
- Logging work in Jira is slower and more cumbersome than it needs to be for frequent daily updates.
- Developers may not realize that another team member is blocked waiting on their task until the issue is raised manually in a meeting or comment thread.

### Capabilities

- View all release data, Gantt views, dependency information, risk indicators, and FlowState-generated insight reports.
- Add and remove **native labels** on work items without modifying Jira.
- Apply filters to resource-centric and release-centric Gantt views.
- Force a manual sync with Jira to pull the latest inbound data.
- View the same resource-centric work hierarchy as managers (see **Nokia Issue Type Hierarchy** below), with work item order reflecting Jira ranking.
- Log work time directly in FlowState on any **leaf work item** (User Story, Task, or Bug; assigned or not), with changes synchronized to Jira.
- View a clear picture of their own assigned work, commitments, and schedule impact within the release.
- Compare planned due dates (from effort estimates) against pace-based forecasts derived from worklogs.
- Export a FlowState pace-based due date to Jira when it differs from the current Jira due date.
- See which portions of the Gantt chart extend beyond the code freeze date and are marked at risk.
- Record own PTO and view PTO for all resources on the timeline.
- Add Jira comments and FlowState-only daily sync updates from the Gantt chart day menu.
- Send a kindly worded email to another team member warning them about an upcoming dependency, with FlowState pre-filling work item and schedule context.
- Request or generate a suggested email or Jira comment to kindly remind another developer about a dependent task, enriched with FlowState context (for example: blocking relationship, expected start date, current status, and delivery impact). See US-013.

### Permissions

Same read access as all internal users. Additional action permissions:

| Action | Allowed |
|--------|---------|
| View release plans, Gantt charts, risks, and reports | Yes |
| Log work time on any leaf work item (synced to Jira) | Yes |
| Generate suggested peer reminders (email or Jira comment) | Yes |
| Send dependency warning email | Yes |
| Export pace-based due date to Jira | Yes |
| Add Jira comment from Gantt chart | Yes |
| Add daily sync update from Gantt chart | Yes |
| Add/remove native labels on work items | Yes |
| Filter resource-centric and release-centric views | Yes |
| Force sync with Jira (inbound) | Yes |
| Export allocation-based due date to Jira | Yes |
| Record own PTO | Yes |
| Approve automated anomaly remediation | No |
| Push schedule or status changes to Jira without approval | No |
| Configure the application | No |

## User Type: Project Manager / Product Owner

### Description

Project managers and product owners steer the release: they monitor health, resolve planning inconsistencies, and decide when the system should intervene in Jira on the team's behalf. They use the same operational view as developers but take responsibility for approving corrective actions that FlowState proposes when anomalies are detected.

### Goals

- Maintain an accurate, live picture of release health without manual detective work.
- Assess overall release progress by feature (Epic) and, when needed, higher-level groupings (System Item, Entity Item, Competance Area).
- Identify critical paths and dependency bottlenecks — for example, when a UI team is blocked waiting for a backend API.
- Detect and resolve planning inconsistencies before they become delivery risks.
- Keep Jira aligned with reality through timely, appropriate status updates and schedule adjustments.
- Warn team members about foreseen risks with context-rich, actionable communication.
- Reduce time spent chasing updates, interpreting fragmented signals, and deciding what to do next.

### Pain Points

- Jira data is often technically present but operationally misleading — for example, work is being logged but the issue status has not been updated.
- Work items pass their planned start dates while remaining in an Open state, silently distorting schedule confidence.
- Manually writing follow-up comments, emails, and schedule corrections is repetitive and easy to defer.
- It is time-consuming to determine whether an anomaly should be fixed with a status change, a communication nudge, or a schedule shift.
- Risk signals are spread across issues, worklogs, dependencies, and dates, making it hard to prioritize intervention.

### Capabilities

- Everything available to developers (full visibility, reports, worklog entry if desired).
- Switch between resource-centric and release-centric Gantt views to monitor individual workload and overall release progress.
- Toggle critical-path highlighting to distinguish schedule-driving work items from non-critical work.
- Compare planned due dates against pace-based forecasts and push revised due dates to Jira when appropriate.
- Define custom milestones and view Jira sprint boundaries on the timeline.
- Trigger automatic updates of Jira **start sprint** and **end sprint** fields from work item schedule dates.
- Set daily allocation caps on work items and export allocation-based due dates to Jira.
- Record and display resource PTO (paid time off) on the timeline.
- Inspect historical changes to work items, resources, and parent items (estimates, due dates, assignees, schedule shifts).
- Import meeting notes and use AI to generate daily sync updates across multiple work items.
- Send dependency warning emails to team members (same capability as all users; see US-013).
- Review FlowState-detected anomalies and approve or reject proposed remediation actions before they are applied or sent.
- Approve automated suggestions such as:
  - **Status mismatch:** A developer logs work daily but the work item remains in Open → FlowState proposes a Jira comment suggesting it be moved to In Progress.
  - **Missed start:** The planned start date has arrived but the work item is still Open → FlowState proposes either a Jira comment suggesting In Progress, or a right-shift of the work item's start and end dates, cascading to dependent downstream work items.
  - **Foreseen risk:** FlowState proposes an email or Jira comment kindly warning a developer about a foreseen delivery risk, including relevant schedule, dependency, and pace context.
  - **Incomplete daily worklog:** A resource logged less than their expected hours for a day (accounting for PTO) → FlowState proposes a reminder email to complete their worklog (see US-024).
- Export approved plan or communication changes to Jira.

### Permissions

Same read access as all internal users. Additional action permissions:

| Action | Allowed |
|--------|---------|
| View release plans, Gantt charts, risks, and reports | Yes |
| Log work time on any leaf work item (synced to Jira) | Yes |
| Generate suggested peer reminders (email or Jira comment) | Yes |
| Send dependency warning email | Yes |
| Review and approve/reject automated anomaly remediation | Yes |
| Push approved status, schedule, and comment changes to Jira | Yes |
| Export pace-based due date to Jira | Yes |
| Add Jira comment from Gantt chart | Yes |
| Add daily sync update from Gantt chart | Yes |
| Import meeting notes and approve AI-generated daily sync updates | Yes |
| Configure daily allocation cap on work item | Yes |
| Record PTO for any resource | Yes |
| Trigger sprint assignment update (start/end sprint) | Yes |
| Open historical inspection views | Yes |
| Configure the application | No |

---

## User Type: Administrator

### Description

Technical owner responsible for connecting FlowState to Jira and external services. Not a delivery role, but listed here for completeness alongside the Access Philosophy role model.

### Goals

- Keep Jira connectivity, synchronization, and notification settings reliable.
- Enable the delivery team to use FlowState without managing infrastructure details.

### Pain Points

- Misconfigured integrations cause stale data, which undermines trust in the entire platform.
- Token expiry or API changes can silently break synchronization.

### Capabilities

- Configure Jira connectivity, API tokens, sync settings, notification settings, and AI provider configuration.
- Configure Jira field mapping for **start sprint** and **end sprint** issue fields.
- Configure **scheduled sprint assignment** trigger interval.
- Manage system-level settings only; not responsible for business-level release decisions or anomaly approvals.

### Permissions

| Action | Allowed |
|--------|---------|
| All User capabilities | Yes |
| Configure application and integrations | Yes |

# 4. User Stories

## Epic: Worklog Management

### US-001: Log Work from the Gantt Chart

As a **developer**,  
I want to log work in FlowState,  
so that I can update progress faster than in Jira.

#### Acceptance Criteria

- [ ] User can click on a day cell for a work item in the Gantt chart to open a **day action menu** (worklog, comments, daily sync — see US-010, US-011).
- [ ] User can set worklog duration using a slider or by selecting preset values (for example: 1h, 2h, 4h, full day).
- [ ] User can log work on **any leaf work item** (User Story, Task, or Bug) — **not only work items they are assigned to**. FlowState does **not** accept worklog entry on parent items (Epic, CA, EI, SI); see US-014 for how parent worklogs from Jira are displayed.
- [ ] Entered worklog is saved in FlowState and synchronized to the corresponding Jira issue (attributed to the logging user).
- [ ] The Gantt chart reflects the logged time after entry (visual indication on the task timeline).

---

### US-014: View Aggregated Worklogs on Parent Items

As a **delivery team member**,  
I want parent items in the hierarchy to show aggregated worklog totals from their children (and any direct worklogs from Jira),  
so that I can see how much effort went into an Epic, Competance Area, Entity Item, or System Item on any given day.

#### Example 1 — Child worklogs only

Hierarchy: 1 SI → 1 EI → 1 CA → 1 Epic → 3 User Stories (each assigned to a different resource).

On day **D**:

| Resource | Work item | Logged |
|----------|--------|--------|
| A | US-1 | 8h |
| B | US-2 | 4h |
| C | US-3 | 8h |

- Each **User Story** shows its value in **hours**: `8h`, `4h`, `8h`.
- The **Epic** shows **2.5d** (20h ÷ 8h/day).
- The same **2.5d** rolls up through **CA → EI → SI** (aggregation continues to System Item level).

#### Example 2 — Jira worklog on parent item

Jira allows worklogs on parent items; FlowState does not. If the **Epic** also has an **8h** worklog logged directly in Jira on day D (in addition to the 20h from children):

| Source | Hours |
|--------|-------|
| US-1 + US-2 + US-3 (children) | 20h |
| Epic (direct, from Jira) | 8h |
| **Total** | **28h** |

The **Epic** row on day D shows **3.5d** (28h ÷ 8h/day). This total rolls up to CA, EI, and SI as **3.5d**.

#### Worklog Entry vs Display

| Where | FlowState entry | Jira worklogs synced | Display unit |
|-------|-----------------|----------------------|--------------|
| User Story, Task, Bug (leaf) | Yes | Yes | **Hours** (`h`) |
| Epic, CA, EI, SI (parent) | No | Yes (read-only) | **Days** (`d`, aggregated) |

#### Display Rules

| Level | Unit | Example on day D |
|-------|------|------------------|
| User Story, Task, Bug (leaf) | **Hours** | `8h`, `4h` |
| Epic, Competance Area, Entity Item, System Item (parent) | **Days** (aggregated) | `2.5d` or `3.5d` |

- Parent aggregation = (**direct worklogs on that item from Jira** + **sum of all descendant worklog hours**) on that day ÷ configured hours-per-day (default **8h**).
- Aggregation rolls up through every parent level: Epic → CA → EI → **SI**.
- The UI must make the unit unambiguous — always show `h` suffix for hours and `d` suffix for days.
- Hovering a parent day cell shows the breakdown: direct hours on the item, per-child contributions, total hours, and the day conversion.

#### Acceptance Criteria

- [ ] Leaf work items (User Story, Task, Bug) display worklog totals per day in **hours**. FlowState accepts worklog entry on leaf items only.
- [ ] Parent items (Epic, CA, EI, SI) display the aggregated total for that day in **days**, rolling up through to **System Item** level.
- [ ] Parent aggregation includes **all descendant worklog hours** plus any **direct worklogs on the parent item synced from Jira**.
- [ ] Aggregation includes worklogs from all assignees across all child work items.
- [ ] Hours and days are visually distinct and clearly labeled (`h` vs `d`).
- [ ] Hover or drill-down on a parent day cell reveals: direct worklogs on the item (if any), per-child breakdown, total hours, and converted days.
- [ ] Aggregation updates immediately when a worklog is added, edited, or removed (in FlowState or on Jira sync).
- [ ] Hours-per-day divisor is configurable (default 8h); changing it recalculates parent day values.

---
## Epic: Gantt Views and Work Hierarchy

### Nokia Issue Type Hierarchy

FlowState reflects the **Nokia Jira issue hierarchy**. Actual delivery work — scheduling, worklogs, dependencies, and Gantt bars — is performed from the **Epic level downward**. Higher levels (System Item, Entity Item, Competance Area) are organizational groupings that users may show or hide.

#### Full hierarchy (parent → child)

```text
System Item (SI)
└── Entity Item (EI)
    └── Competance Area (CA)
        └── Epic
            ├── User Story
            └── Task

Bug  ← independent; not part of the SI → EI → CA → Epic chain
```

#### Show / hide organizational levels

Users can independently toggle visibility of **System Items (SI)**, **Entity Items (EI)**, and **Competance Areas (CA)**. When a level is hidden, the tree skips it and connects the next visible ancestor directly to its descendants.

| Level | Default | Rationale |
|-------|---------|-----------|
| System Item (SI) | Hidden | Organizational; work starts at Epic |
| Entity Item (EI) | Hidden | Organizational; work starts at Epic |
| Competance Area (CA) | Hidden | Organizational; work starts at Epic |
| Epic | Shown | Feature-level planning and tracking |
| User Story / Task | Shown | Executable work items |
| Bug | Shown | Independent work items |

**Default view:** Epic → User Story / Task, plus Bugs. SI, EI, and CA are available on demand via toggles.

#### Independent Bugs

Bugs are **not** children of Epic in the Nokia hierarchy. In the Gantt tree they appear as direct children of a **Resource** (resource-centric view) or at the **release root** alongside System Items / Epics (release-centric view).

---

### US-002: View Resource Work in Hierarchical Order

As a **delivery team member**,  
I want to view all work items organized in a resource-centric hierarchy,  
so that when I look at a resource's timeline, I can see the order of work items they will handle.

#### Hierarchy

Expandable tree view — all resources are visible at the top level. Example with SI/EI/CA **hidden** (default):

```text
▼ Resources
├── ▼ Alice
│   ├── ▼ Epic: Payment Checkout
│   │   ├── User Story: US-101
│   │   ├── User Story: US-102
│   │   └── Task: TASK-42
│   ├── ▶ Epic: Performance
│   ├── Bug: BUG-7                   ← direct child of Resource
│   └── Bug: BUG-12
├── ▶ Bob
└── ▶ Carol
```

Example with SI/EI/CA **shown**:

```text
▼ Resources
├── ▼ Alice
│   ├── ▼ System Item: Core Platform
│   │   └── ▼ Entity Item: Billing
│   │       └── ▼ Competance Area: Payments
│   │           └── ▼ Epic: Payment Checkout
│   │               ├── User Story: US-101
│   │               └── Task: TASK-42
│   └── Bug: BUG-7
└── ▶ Bob
```

Structural summary (full Nokia chain under each resource):

```text
Resources (all shown)
└── Resource                         expand →
    ├── System Item (SI)             expand →  [toggleable]
    │   └── Entity Item (EI)         expand →  [toggleable]
    │       └── Competance Area (CA) expand →  [toggleable]
    │           └── Epic             expand →
    │               ├── User Story
    │               └── Task
    └── Bug                          direct child of Resource
```

#### Metadata Columns

The tree is accompanied by the same Jira-synced columns as the release-centric view, **except Assignee** — in this view the resource is already implied by the row's position in the hierarchy.

| Column | Source |
|--------|--------|
| Labels | Jira labels (`J:…`) and FlowState native labels |
| Status | Jira workflow status |
| Priority | Jira priority |
| Start / Due date | Jira or FlowState schedule fields |

Additional Jira fields may be shown as configurable columns.

#### Acceptance Criteria

- [ ] Gantt chart supports a resource-centric view where each row represents a resource (team member).
- [ ] Work items under a resource follow the **Nokia issue type hierarchy** (SI → EI → CA → Epic → User Story / Task). Bugs are direct children of the Resource.
- [ ] User can independently **show or hide** System Items (SI), Entity Items (EI), and Competance Areas (CA). When hidden, Epics appear directly under the Resource (default).
- [ ] Expanding an Epic reveals its User Stories and Tasks. User Story and Task are sibling types under Epic.
- [ ] Within a resource's timeline, work items are ordered by **Jira ranking** (FlowState reflects Jira's rank order, not a separate sort).
- [ ] All internal users — developers and managers alike — see the same resource hierarchy view.
- [ ] Each row displays metadata columns alongside the hierarchy tree, including at minimum **Labels**, **Status**, **Priority**, and **Start / Due date**. Assignee is omitted because the resource-centric grouping already identifies who owns the work.
- [ ] Column values are synchronized from Jira and reflect the current state of each issue.
- [ ] User can expand and collapse any visible hierarchy level.
- [ ] User can show or hide metadata columns as needed.
- [ ] Dependencies and schedule position remain visible alongside ranked order for context.

---

### US-003: View Release Progress (Release-Centric Hierarchy)

As a **manager**,  
I want to view all work items organized in a release-centric hierarchy with metadata columns,  
so that I can assess overall release progress without focusing on individual resources.

#### Hierarchy

Expandable tree view following the Nokia hierarchy. Example with SI/EI/CA **hidden** (default):

```text
▼ Release
├── ▼ Epic: Payment Checkout
│   ├── User Story: US-101
│   ├── User Story: US-102
│   └── Task: TASK-42
├── ▶ Epic: Performance
├── Bug: BUG-7                         ← release root, independent of Epic chain
└── Bug: BUG-12
```

Example with SI/EI/CA **shown**:

```text
▼ Release
├── ▼ System Item: Core Platform
│   └── ▼ Entity Item: Billing
│       └── ▼ Competance Area: Payments
│           ├── ▼ Epic: Payment Checkout
│           │   ├── User Story: US-101
│           │   └── Task: TASK-42
│           └── ▶ Epic: Performance
├── ▶ System Item: Infrastructure
└── Bug: BUG-7
```

Structural summary:

```text
Release scope
├── System Item (SI)                 expand →  [toggleable]
│   └── Entity Item (EI)             expand →  [toggleable]
│       └── Competance Area (CA)     expand →  [toggleable]
│           └── Epic                 expand →
│               ├── User Story
│               └── Task
└── Bug                                release root, independent
```

#### Metadata Columns

The tree is accompanied by columns synced from Jira. The release-centric view includes **Assignee**; the resource-centric view omits it (see US-002).

| Column | Source | Resource view | Release view |
|--------|--------|---------------|--------------|
| Assignee | Jira assignee | — | Yes |
| Labels | Jira labels (`J:…`) and FlowState native labels | Yes | Yes |
| Status | Jira workflow status | Yes | Yes |
| Priority | Jira priority | Yes | Yes |
| Start / Due date | Jira or FlowState schedule fields | Yes | Yes |

Additional Jira fields may be shown as configurable columns in either view.

#### Acceptance Criteria

- [ ] Gantt chart supports a release-centric view rooted at the release scope.
- [ ] Work items follow the **Nokia issue type hierarchy** (SI → EI → CA → Epic → User Story / Task). Bugs appear at the release root, independent of the SI chain.
- [ ] User can independently **show or hide** System Items (SI), Entity Items (EI), and Competance Areas (CA). When hidden, Epics appear at the release root (default).
- [ ] Expanding an Epic reveals its User Stories and Tasks.
- [ ] Work items are ordered by **Jira ranking** within each hierarchy level.
- [ ] Each row displays metadata columns alongside the hierarchy tree, including at minimum **Assignee** and **Labels**.
- [ ] Column values are synchronized from Jira and reflect the current state of each issue.
- [ ] Manager can use this view to assess release-wide progress, scope completion, and feature health without switching to a resource-centric layout.
- [ ] User can expand and collapse any visible hierarchy level and show or hide metadata columns as needed.

---

## Epic: Labels and View Filtering

### US-020: Jira Labels and Native Labels

As a **delivery team member**,  
I want to use both Jira labels and FlowState-only native labels on work items,  
so that I can classify and filter work in FlowState without necessarily changing anything in Jira.

#### Label Types

| Type | Display | Stored in | Written to Jira |
|------|---------|-----------|-----------------|
| **Jira label** | `J:<label name>` | Jira (synced inbound) | No (read-only in FlowState) |
| **Native label** | `<label name>` | FlowState database | No |

#### Examples

- Jira label `backend` → displayed as **`J:backend`**
- FlowState native label `needs-review` → displayed as **`needs-review`**
- A work item may carry both: `J:release-2.0`, `needs-review`

#### Acceptance Criteria

- [ ] Jira labels are synchronized from Jira and displayed with the **`J:`** prefix.
- [ ] Users can add and remove **native labels** on any work item without modifying the Jira issue.
- [ ] Native labels are stored in the FlowState database only.
- [ ] Labels column in both Gantt views shows Jira and native labels together, visually distinguishable by prefix.
- [ ] Removing a native label does not affect Jira. FlowState does not add, remove, or edit Jira labels unless explicitly scoped in a future feature.

---

### US-021: Filter Gantt Views

As a **delivery team member**,  
I want to filter the **resource-centric** and **release-centric** Gantt views with a simple boolean expression,  
so that I can focus on relevant work items without using JQL or leaving FlowState.

#### Scope

- Filtering applies to **US-002 (resource-centric)** and **US-003 (release-centric)** views only.
- Purpose: narrow what is shown on the Gantt chart and hierarchy tree — **not** replace Jira for bug scrubbing, triage, or bulk search.
- **No sorting** — work item order remains Jira ranking.
- **No full JQL** — only a focused filter grammar for Gantt views.

#### Supported Operators

| Operator | Meaning |
|----------|---------|
| `AND` | Both conditions must match |
| `OR` | Either condition matches |
| `NOT` | Negates the condition |
| `XOR` | Exactly one of the conditions matches |
| `IN` | Value is in a list |
| `NOT IN` | Value is not in a list |
| `( )` | Groups sub-expressions |

#### Filterable Fields (minimum)

- Labels (Jira and native): `J:backend`, `needs-review`
- Assignee
- Status
- Priority
- Issue type (Epic, User Story, Task, Bug, …)

#### Example Expressions

```text
J:backend AND NOT needs-review
(J:release-2.0 OR internal-review) AND status IN (Open, "In Progress")
assignee IN (john.smith) XOR issue-type = Bug
NOT status IN (Done, Closed)
```

#### Acceptance Criteria

- [ ] User can enter a filter expression on resource-centric and release-centric Gantt views.
- [ ] Filter supports **AND**, **OR**, **NOT**, **XOR**, **IN**, **NOT IN**, and **parentheses**.
- [ ] Filter matches against Jira labels (`J:…`), native labels, and other supported metadata fields.
- [ ] Filtered view hides non-matching work items; parent rows remain visible if a descendant matches (ancestors shown for context).
- [ ] Filter does **not** change Jira ranking order among visible items.
- [ ] Filter does **not** provide sorting.
- [ ] Invalid expressions show a clear parse error without affecting the underlying data.
- [ ] User can clear the filter to restore the full view.

---

### US-022: Force Sync with Jira

As a **delivery team member**,  
I want to manually trigger a sync with Jira to pull the latest data,  
so that FlowState reflects the current state of issues, worklogs, and metadata when I need it.

#### Background

FlowState caches Jira data locally for responsiveness. A **scheduled background sync** keeps data reasonably fresh, but users sometimes need the **latest** data immediately after changes made directly in Jira.

Force sync duration depends on Jira API response time and release size. FlowState cannot guarantee speed when Jira is slow.

#### Acceptance Criteria

- [ ] User can trigger a **force sync** (for example: Sync button in the toolbar or menu).
- [ ] Force sync pulls the latest inbound data from Jira: issues, worklogs, comments, dependencies, sprints, Jira labels, ranks, and metadata.
- [ ] While sync runs, the UI shows a **progress indicator** and does **not freeze** — user can continue viewing cached data or other interactions where feasible.
- [ ] UI displays **last synced** timestamp and a **stale data** indicator when cache is older than the configured threshold.
- [ ] Force sync **may exceed 1 second** (and may take much longer on large projects or when Jira is slow); this is expected and acceptable.
- [ ] On success, cached data and derived calculations (aggregations, forecasts, critical path) refresh.
- [ ] On failure, user sees a clear error and can retry; cached data remains available.
- [ ] Force sync does not push local changes to Jira — it is **inbound only**. Outbound writes remain explicit user actions (worklog entry, due date export, etc.).

---

## Epic: Schedule Intelligence and Critical Path

### US-004: Highlight Critical Path

As a **manager**,  
I want to visually distinguish work items on the critical path from other work,  
so that I can pinpoint which work is driving the release date and where delays will have the greatest impact.

#### Example

The UI team is waiting for a backend API to be completed before resuming development. The work items representing backend API work lie on the critical path; if they slip, downstream UI work and the release date are directly affected. These work items should stand out visually compared to work items not on the critical path.

#### Acceptance Criteria

- [ ] FlowState calculates the critical path from task dependencies and schedule data.
- [ ] Work items on the critical path are visually distinguishable on the Gantt chart (for example: distinct color, border, or badge).
- [ ] A toggle switch enables or disables critical-path highlighting; when off, all work items use the default appearance.
- [ ] Critical-path highlighting is available in both resource-centric and release-centric views.
- [ ] Hovering or selecting a critical-path work item shows why it is on the critical path (for example: blocking dependents, zero slack).

---

### US-005: Compare Planned vs Pace-Based Due Dates

As a **manager**,  
I want to see two due dates for each work item — one based on the assignee's effort estimate and one based on actual development pace,  
so that I can tell whether we are ahead of plan, on track, or falling behind.

#### Due Date Types

| Due date | Basis | Source |
|----------|-------|--------|
| **Planned due date** | Assignee effort estimate and scheduled start | Original plan / Jira due date |
| **Forecast due date** | Remaining work ÷ pace derived from worklogs | FlowState calculation |
| **Allocation-based due date** | Remaining effort ÷ daily allocation cap schedule | FlowState calculation (when cap is set; recalculates on each worklog; see US-015) |

#### Acceptance Criteria

- [ ] Each work item displays a **planned due date** derived from the assignee's effort estimate and schedule position.
- [ ] Each work item displays a **forecast due date** calculated from logged work, remaining effort, and current development pace.
- [ ] Both dates are visually distinguishable on the Gantt chart (for example: planned bar vs forecast marker, or dual end markers on the same row).
- [ ] When forecast due date is later than planned due date, the slippage is visually obvious (for example: red forecast marker or delta label such as "+3 days").
- [ ] Metadata columns or tooltips show both dates and the delta between them.
- [ ] Forecast due date recalculates as new worklogs are entered.

---

### US-006: Export Pace-Based Due Date to Jira

As an **assignee or manager**,  
I want to push a FlowState forecast due date to Jira in one or two clicks,  
so that Jira reflects realistic delivery expectations when pace-based estimates differ from the current due date.

#### Example

A work item's due date in Jira is **D**, but FlowState's pace-based forecast is **D+3**. The user selects the work item and exports the forecast date **D+3** to Jira with minimal friction.

#### Acceptance Criteria

- [ ] When a work item's forecast due date differs from its Jira due date, the UI indicates that an update is available.
- [ ] User can export the forecast due date to Jira in **one or two clicks** (for example: action button on the work item row or context menu → confirm).
- [ ] Both the **assignee** and **managers** can perform this export.
- [ ] After export, the Jira issue due date is updated via REST API and the Gantt chart reflects the new Jira value as the planned due date.
- [ ] User sees confirmation of success or a clear error if the Jira update fails.

---

### US-015: Daily Allocation Cap and Allocation-Based Due Date

As a **manager**,  
I want to set a daily worklog cap on a work item and have FlowState calculate the realistic end date,  
so that scheduling reflects partial availability (for example, customer support or recurring training) that Jira cannot represent.

#### Background

Jira does not support per-work-item daily effort caps. FlowState stores this as a **planning assumption** locally and uses it for Gantt visualization, schedule calculation, and due date export. The cap guides scheduling; it does **not** block worklog entry.

#### Allocation Types

| Type | Example | Meaning |
|------|---------|---------|
| **Fixed rate over interval** | 0.5 day every calendar day from Mar 1–Mar 31 | Developer spends half of each day on customer support for this work item |
| **Recurring pattern** | 0.5 day every Monday | Developer is in training every Monday afternoon; only half a day available for this work item on Mondays |

#### Visual Representation

- The Gantt bar **vertical thickness** reflects the daily allocation fraction.
  - Full allocation (1.0 day/day) → full row height
  - Half allocation (0.5 day/day) → **half row height**
- Thickness may vary by day when a recurring pattern applies (for example, thin on Mondays, full height on other days).

#### Allocation vs Actual Worklogs

The daily allocation cap is a **planning target**, not a hard limit.

- Developers **may log more** than the cap on any day (for example, log a full 8h when allocation is 0.5 day).
- FlowState **does not block** excess worklog entry.
- When work is logged, FlowState recalculates the **allocation-based due date** from **remaining person-day effort** and the allocation schedule (see below).
- The UI may optionally indicate when logged time on a day exceeds the planned allocation (informational only).

#### Due Date Calculation

FlowState calculates an **allocation-based due date** from effort (person-days) and the allocation schedule.

**Initial calculation** (no worklogs yet):

```text
remaining effort = total effort estimate (person-days)
due date         = start date + slots needed at allocation rate
```

**After worklogs** (recalculated on every log):

```text
remaining effort = total effort estimate − work completed (person-days from worklogs)
due date         = today + slots needed for remaining effort at allocation rate
```

**Example 1 — Fixed half-day rate**

- Effort: **2 person-days**
- Allocation: **0.5 day per calendar day**
- Initial span: 2 ÷ 0.5 = **4 calendar days**
- If Jira due date **E** assumed full-time (2 calendar days), the allocation-based due date is **E + 2 days**
- After **1.5 person-days** logged (including days where the developer exceeded the 0.5 cap): remaining = **0.5 person-days** → **1 more calendar day** at 0.5/day

**Example 2 — Recurring half-day (Mondays only)**

- Effort: **3 person-days**
- Allocation: **0.5 day every Monday**
- Initial: 3 ÷ 0.5 = **6 Mondays** → **6 weeks**
- After **1 person-day** logged: remaining = **2 person-days** → **4 more Mondays**

#### Acceptance Criteria

- [ ] Manager can define a **daily allocation cap** on a leaf work item (FlowState database only; not synced from Jira).
- [ ] Supported allocation types: **fixed rate over a date interval** and **recurring day-of-week pattern**.
- [ ] Gantt bar vertical thickness reflects the **planned** allocation fraction for each day (0.5 → half height).
- [ ] Developers **may log work in excess of** the daily allocation cap; FlowState does not block excess entry.
- [ ] FlowState calculates an initial **allocation-based due date** from total effort estimate + allocation schedule.
- [ ] After each worklog, FlowState **recalculates** the allocation-based due date from **remaining person-day effort** and the allocation schedule.
- [ ] Calculated due date is shown alongside the Jira planned due date and is visually distinguishable.
- [ ] Manager or assignee can **export the allocation-based due date to Jira** (one or two clicks), updating the issue due date via REST API.
- [ ] Allocation-based due date recalculates when allocation rules, effort estimate, start date, or worklogs change.

---

## Epic: Timeline Context

### US-007: Custom Milestones

As a **manager**,  
I want to define and display custom milestones on the timeline,  
so that I can mark important release events that are not represented as Jira issues.

#### Acceptance Criteria

- [ ] Manager can create, edit, and delete custom milestones (name, date, optional description).
- [ ] Milestones are stored in the **FlowState database** (not in Jira).
- [ ] Milestones appear as vertical markers or labels on the Gantt timeline.
- [ ] Milestones are visible in both resource-centric and release-centric views.
- [ ] Milestones persist across sessions and are scoped to a release or project.

---

### US-008: Sprint Boundaries on Timeline

As a **manager**,  
I want the Gantt timeline divided into sprints obtained from Jira,  
so that I can see how work aligns with sprint cadence.

#### Acceptance Criteria

- [ ] FlowState retrieves sprint definitions (name, start date, end date) from Jira.
- [ ] The Gantt timeline displays sprint boundaries as labeled regions or vertical dividers.
- [ ] Sprint boundaries update when Jira sprint data changes on sync.
- [ ] User can toggle sprint overlay visibility on or off.
- [ ] Work items show which sprint they belong to (from Jira) in metadata columns or tooltips.

---

### US-023: Auto-Assign Start and End Sprints

As a **manager**,  
I want FlowState to update each work item's **start sprint** and **end sprint** in Jira based on its schedule dates,  
so that sprint fields stay aligned with the Gantt plan without manual scrubbing in Jira.

#### Background

The organization requires periodic updates to issue sprint fields in Jira. FlowState derives **start sprint** and **end sprint** from the work item's **start date** and **due date** (planned, allocation-based, or exported values — configurable which date source drives the mapping) and the **Jira sprint calendar** (sprint name, start date, end date) synced into FlowState.

#### Mapping Rules

| Field | Derived from | Rule |
|-------|--------------|------|
| **Start sprint** | Work item **start date** | The Jira sprint whose date range contains the start date (or the first sprint that overlaps the scheduled work if the start date falls between sprints). |
| **End sprint** | Work item **due date** | The Jira sprint whose date range contains the due date (or the last sprint that overlaps the scheduled work). |

If start and due dates fall in the same sprint, both fields may reference that sprint.

#### Triggers

| Trigger | Description |
|---------|-------------|
| **Scheduled** | Periodic run (interval configured by Administrator) — satisfies the organizational mandate for regular sprint field updates. |
| **Manual** | Manager clicks **Update sprints** to run immediately for the current release scope. |
| **On schedule change** | Optional: run automatically when a work item's start or due date changes in FlowState (export to Jira or local recalculation). |

#### Acceptance Criteria

- [ ] FlowState reads Jira sprint definitions (name, start date, end date) and maps work item dates to **start sprint** and **end sprint**.
- [ ] Manager can trigger sprint assignment **manually** for the current release.
- [ ] Administrator can configure a **scheduled trigger** (for example: daily or weekly) for automatic sprint updates.
- [ ] Before writing to Jira, FlowState presents a **preview** of proposed changes (work item, current start/end sprint → proposed start/end sprint).
- [ ] Manager can **approve** the batch (or per-item) before updates are pushed to Jira.
- [ ] Approved updates are written to Jira via REST API (start sprint and end sprint fields — field IDs configured by Administrator).
- [ ] Work items with missing start or due dates are skipped or flagged for manual review.
- [ ] After update, a force sync or targeted refresh reflects the new sprint values in FlowState.

---

### US-016: Represent PTO on the Timeline

As a **manager or developer**,  
I want to record paid time off (PTO) for resources and see it on the Gantt timeline,  
so that scheduling accounts for days or partial days when a person is unavailable.

#### Background

Jira does not represent PTO. FlowState stores PTO in its local database and uses it for timeline visualization and schedule calculations (allocation-based due dates, resource capacity).

#### PTO Amount

PTO is expressed as a **fraction of a day** unavailable:

| Value | Meaning |
|-------|---------|
| 0.25 | Quarter day off |
| 0.5 | Half day off |
| 1.0 | Full day off |
| 1.5 | Day and a half off (spans into next calendar day per configured hours-per-day) |

#### PTO Patterns

| Pattern | Example |
|---------|---------|
| **Single date** | 1.0 day off on Apr 15 |
| **Fixed duration** | 0.5 day off for **4 consecutive calendar days** (Apr 10–Apr 13) |
| **Recurring with limit** | 0.5 day off **every Friday for 4 weeks** |

#### Visual Representation

- PTO appears on the **resource row** in the resource-centric Gantt view (and affects assignee availability in schedule calculations).
- Full-day PTO (1.0): day cell visually marked as unavailable (for example: shaded or striped background).
- Partial-day PTO (0.25, 0.5): day cell shows the **unavailable portion** (for example: shaded band covering the top 50% for 0.5 day).
- Hovering a PTO day cell shows: resource name, PTO amount, pattern description, date(s).

#### Schedule Impact

- PTO reduces the assignee's **available capacity** on affected days.
- **Allocation-based due date** calculation (US-015) skips or reduces slots on PTO days when spreading remaining effort for work items assigned to that resource.
- For example: 0.5 day PTO on a Friday means only 0.5 day of capacity remains for scheduled work that day (before work-item allocation caps are applied).

#### Acceptance Criteria

- [ ] User can create, edit, and delete PTO entries for a resource (FlowState database only; not synced to Jira).
- [ ] PTO amount supports fractional days: **0.25, 0.5, 1.0, 1.5** (and other values as needed).
- [ ] Supported patterns: **single date**, **fixed duration** (for example, 0.5 day for 4 consecutive days), and **recurring with limit** (for example, 0.5 day every Friday for 4 weeks).
- [ ] PTO is visible on the resource timeline in the resource-centric Gantt view.
- [ ] Partial-day PTO is visually distinguishable from full-day PTO.
- [ ] Allocation-based due dates for work items assigned to the resource **account for PTO** when calculating remaining schedule slots.
- [ ] Developers can record **their own** PTO; managers can record PTO for **any resource**.
- [ ] All internal users can view PTO on the timeline.

---

## Epic: Historical Inspection

FlowState captures a **daily snapshot** of each work item's state: assignee, effort estimate, due date, scheduled bar position, and work logged. Managers use inspection views to replay how the plan evolved over time.

#### Cell notation (all inspection types)

| Symbol | Meaning |
|--------|---------|
| **O** | Future / unworked portion of the scheduled bar. Consecutive **O** cells form the rectangle representing remaining work. |
| **0.0, 1.0, …** | Person-days logged on that **calendar column** for the work item (or resource), as recorded on the **snapshot row's date**. |
| **T1, T2, …** | Work item labels on the resource inspection view (one resource's assigned items shown inline). |

- **Rows** = snapshot date ("as of this date, the plan looked like this").
- **Columns** = calendar dates on the timeline.
- **Assignee column** = who was assigned on the snapshot date.

---

### US-017: Work Item Inspection

As a **manager**,  
I want to inspect the daily history of a single work item as a sequence of mini-Gantt rows,  
so that I can see how its estimate, due date, assignee, and schedule shifted over time.

#### Example 1 — On time (4 person-days)

| Snapshot date | Assignee | 6 Sep | 7 Sep | 8 Sep | 9 Sep | 10 Sep |
|---------------|----------|-------|-------|-------|-------|--------|
| 5 Sep | John | O | O | O | O | |
| 6 Sep | John | 1.0 | O | O | O | |
| 7 Sep | John | 1.0 | 1.0 | O | O | |
| 8 Sep | John | 1.0 | 1.0 | 1.0 | O | |
| 9 Sep | John | 1.0 | 1.0 | 1.0 | 1.0 | |

Completed on schedule. Each row adds the day's logged work; **O** cells show the remaining bar.

#### Example 2 — Slipped (2 person-days)

| Snapshot date | Assignee | 6 Sep | 7 Sep | 8 Sep | 9 Sep | 10 Sep |
|---------------|----------|-------|-------|-------|-------|--------|
| 5 Sep | John | O | O | | | |
| 6 Sep | John | 0.0 | O | O | | |
| 7 Sep | John | 0.0 | 1.0 | O | | |
| 8 Sep | John | 0.0 | 1.0 | 0.0 | O | |

John did not work on **6 Sep** or **8 Sep** (**0.0**). The bar right-shifted; due date moved **2 days** later than planned.

#### Acceptance Criteria

- [ ] Manager can open a **work item inspection** view for any leaf or parent work item.
- [ ] View renders as a table: rows = snapshot dates, columns = calendar days, plus assignee column.
- [ ] Each row shows the work item's Gantt bar **as it was known on that snapshot date**.
- [ ] Logged person-days appear as numeric values; future work appears as **O** cells forming the bar rectangle.
- [ ] Assignee column reflects assignee on each snapshot date.
- [ ] Manager can see when estimate, due date, or assignee changed between rows.
- [ ] Inspection data is derived from FlowState's daily snapshot history (see §8).

---

### US-018: Resource Inspection

As a **manager**,  
I want to inspect the daily history of all work items assigned to a resource, shown inline on one timeline,  
so that I can see how slippage on one item affected others.

#### Example

Work items **T1**, **T2** assigned to John. T1 not worked on 6 Sep → both T1 and T2 right-shift from 7 Sep onward.

| Snapshot date | Assignee | 6 Sep | 7 Sep | 8 Sep | 9 Sep | 10 Sep |
|---------------|----------|-------|-------|-------|-------|--------|
| 5 Sep | John | T1 | T1 | T2 | T2 | |
| 6 Sep | John | 1.0 | T1 | T2 | T2 | |
| 7 Sep | John | 1.0 | 0.0 | T1 | T2 | T2 |
| 8 Sep | John | 1.0 | 0.0 | 1.0 | T2 | T2 |
| 9 Sep | John | 1.0 | 0.0 | 1.0 | 1.0 | T2 |

- **T1 / T2** labels = scheduled bar for that work item on that day.
- **1.0 / 0.0** = person-days John logged on that item that calendar day.
- Row **7 Sep**: **0.0** on 7 Sep = no work on T1; T1 and T2 bars both shift right from this snapshot onward.

#### Acceptance Criteria

- [ ] Manager can open a **resource inspection** view for any resource.
- [ ] All work items assigned to the resource are shown **inline** on the same calendar columns, ordered by Jira ranking.
- [ ] Rows = snapshot dates; columns = calendar days; cells use **T1/T2 labels** for scheduled bars and **numeric values** for logged work.
- [ ] Schedule right-shifts on one work item (due to missed work) are visible as shifts in subsequent snapshot rows for **downstream items on the same resource**.
- [ ] PTO and daily allocation caps are reflected in the historical view when they affected the schedule.

---

### US-019: Parent Work Item Inspection

As a **manager**,  
I want to inspect the aggregated daily history of a parent work item (Epic, CA, EI, SI),  
so that I can see how combined team effort across child work items evolved.

#### Example

A parent row shows **aggregated person-days** logged across all child work items on each calendar day. A value of **5.5** on a column means 5.5 person-days of total team effort on child items that day (for example, 6 developers each logged 1.0 day, one logged 0.5 day).

| Snapshot date | Assignee | 6 Sep | 7 Sep | 8 Sep | 9 Sep |
|---------------|----------|-------|-------|-------|-------|
| 6 Sep | — | 4.0 | O | O | O |
| 7 Sep | — | 4.0 | 5.5 | O | O |
| 8 Sep | — | 4.0 | 5.5 | 3.0 | O |

- **O** = remaining aggregated bar (future work across children).
- **Numeric values** = sum of child worklogs converted to person-days for that calendar day, as of the snapshot date.

#### Acceptance Criteria

- [ ] Manager can open a **parent work item inspection** view for Epic, CA, EI, or SI.
- [ ] View aggregates child work item history into a single inline timeline per snapshot row.
- [ ] Each calendar column shows **total person-days** logged across all descendants on that day.
- [ ] Future work appears as **O** cells forming the aggregated remaining bar.
- [ ] Multiple developers' contributions are combined (consistent with US-014 aggregation rules).
- [ ] Manager can drill down from a parent inspection cell to the contributing child work items.

---

## Epic: Anomaly Detection and Remediation

FlowState continuously monitors delivery activity and proposes remediation actions. Managers **review and approve** before emails are sent or Jira is updated (except user-initiated actions such as US-013).

### US-024: Incomplete Daily Worklog Reminder

As a **manager**,  
I want FlowState to detect when a resource has not logged a full day's work and prepare a reminder email,  
so that missing worklogs are caught early without manual checking.

#### Detection Rule

For each resource on each **working day**:

```text
expected hours = (hours-per-day) − (PTO hours for that day)
                 = 8 − (PTO fraction × 8)    [default hours-per-day = 8]

logged hours   = sum of all worklogs by that resource across all work items on that day

anomaly        = logged hours < expected hours
```

**Examples:**

| PTO | Expected hours | Logged | Anomaly? |
|-----|----------------|--------|----------|
| None | 8h | 8h | No |
| None | 8h | 5h | Yes — 3h idle |
| 0.5 day (4h PTO) | 4h | 4h | No |
| 0.5 day (4h PTO) | 4h | 2h | Yes — 2h idle |
| 1.0 day (full PTO) | 0h | 0h | No — exempt |

When an anomaly is detected, the resource is viewed as having been **idle for part of that day** (expected hours minus logged hours).

#### Proposed Action

FlowState generates a **kind, professional email draft** to the resource, for example:

> Hi Alex, your worklogs for **Tuesday 9 Sep** total **5h**; **3h** remain unlogged based on your expected availability that day (8h expected, no PTO). Please update your worklogs in FlowState or Jira when you have a moment.

The manager reviews, edits if needed, and approves before send.

#### Acceptance Criteria

- [ ] FlowState evaluates each resource's total logged hours per day against **expected hours** (hours-per-day minus PTO).
- [ ] PTO on a day reduces expected hours proportionally (for example: 0.5 day PTO → 4h expected at 8h/day).
- [ ] Full-day PTO (1.0) exempts the resource from incomplete-worklog detection for that day.
- [ ] When `logged hours < expected hours`, FlowState flags an **incomplete daily worklog** anomaly.
- [ ] FlowState generates a **ready-to-send email draft** naming the date, hours logged, hours short, and PTO adjustment if any.
- [ ] Manager can **review, edit, approve, or reject** the email before it is sent.
- [ ] Approved email is sent via the configured mail integration (see §11 Email).
- [ ] Detection runs on a **scheduled trigger** (for example: end of day or next morning) and may also surface in the anomaly review queue.
- [ ] Hours-per-day default (8h) is configurable; uses the same setting as worklog aggregation (US-014).

---

## Epic: Risk Visualization

### US-009: Code Freeze At-Risk Highlighting

As a **delivery team member**,  
I want to see which portion of each work item's timeline extends beyond the code freeze date,  
so that I can immediately understand what work is at risk of missing the freeze.

#### Acceptance Criteria

- [ ] The release has a configurable **code freeze date** (set as a milestone or release setting).
- [ ] The code freeze date appears as a vertical marker on the Gantt timeline.
- [ ] For any work item whose planned or forecast end date exceeds the code freeze date, the portion of the bar **beyond the code freeze date** is rendered with an **at-risk** visual style (for example: red shading, hatched pattern, or risk overlay).
- [ ] At-risk rendering is visible to **all internal users** in both resource-centric and release-centric views.
- [ ] Hovering the at-risk portion shows why it is at risk (for example: "Extends 3 days past code freeze").
- [ ] At-risk highlighting updates automatically when schedule data, forecast dates, or the code freeze date changes.

---

## Epic: Comments and Daily Sync

### US-010: Add and View Comments from the Gantt Chart

As a **delivery team member**,  
I want to view and add Jira comments from a day cell on the Gantt chart,  
so that I can discuss progress in context without leaving the timeline.

#### Acceptance Criteria

- [ ] Clicking a day cell on a work item opens a **day action menu** with options including worklog, comments, and daily sync.
- [ ] The **Comments** option shows existing Jira comments for that issue (synchronized from Jira).
- [ ] User can **add a new comment**, which is posted to the corresponding Jira issue via REST API.
- [ ] New comments appear in the menu immediately after posting and on subsequent Jira sync.
- [ ] All internal users can view and add comments.

---

### US-011: Daily Sync Updates on the Gantt Chart

As a **delivery team member**,  
I want to add a special **daily sync update** to a specific day on a work item's timeline,  
so that the team can record concise standup-style progress notes tied to a date.

#### Acceptance Criteria

- [ ] The day action menu includes a **Daily Sync** option distinct from regular Jira comments.
- [ ] User can enter a daily sync update for a specific work item and date.
- [ ] Daily sync updates are stored in the **FlowState database only** — they are **not** posted to Jira and Jira cannot distinguish them from other FlowState data.
- [ ] Days with a daily sync update show a visual indicator on the Gantt chart (for example: dot or icon on the day cell).
- [ ] **Hovering** over a day with a daily sync update displays the update text in a tooltip or popover.
- [ ] All internal users can add, view (via hover), and edit their own daily sync updates.

---

### US-012: Import Meeting Notes as Daily Sync Updates

As a **manager**,  
I want to import meeting notes into FlowState and have an AI agent convert them into daily sync updates for multiple work items,  
so that standup and meeting outcomes are captured on the right work items without manual copy-paste.

#### Acceptance Criteria

- [ ] Manager can paste or upload meeting notes into FlowState.
- [ ] FlowState invokes an **AI agent** with access to release context via the **FlowState MCP server** (work item list, assignees, statuses, dependencies, schedule).
- [ ] The AI proposes a set of **daily sync updates** mapped to specific work items and dates, derived from the meeting notes.
- [ ] Manager can **review, edit, approve, or reject** each proposed daily sync update before it is saved.
- [ ] Approved updates are stored as FlowState daily sync updates (not as Jira comments).
- [ ] The Gantt chart reflects the new daily sync indicators on the corresponding day cells.
- [ ] If the AI cannot confidently map a note to a work item, it flags the item for manual assignment rather than guessing.

---

## Epic: Communication and Notifications

### US-013: Send Dependency Warning Email

As a **delivery team member**,  
I want to send a kindly worded email to another user warning them about an upcoming dependency,  
so that blockers are surfaced early without awkward manual chasing.

#### Example

> Hi John, kind reminder for ETA of work item **T**, which is a dependency to work item **W** that needs to start on date **D**.

FlowState pre-fills the recipient, issue keys, dependency relationship, and required start date from release data. The user can review and edit before sending.

#### Acceptance Criteria

- [ ] Any internal user can initiate a dependency warning email from a work item's context menu or dependency view.
- [ ] FlowState identifies the **blocking work item** (T), the **dependent work item** (W), the **recipient** (assignee of T), and the **required start date** (D) of W from dependency and schedule data.
- [ ] FlowState generates a **kind, professional email draft** using the template pattern above, pre-filled with live data.
- [ ] User can **review and edit** the recipient, subject, and body before sending.
- [ ] User sends the email in **one or two clicks** after review (for example: Send → confirm).
- [ ] Email is sent via the configured mail integration (SMTP or system email service).
- [ ] Sent emails are logged in FlowState (recipient, work items referenced, timestamp, sender) for audit purposes.
- [ ] If the blocking work item has no assignee or email address, FlowState prompts the user to specify the recipient manually.

---

# 5. Detailed Use Cases

## Use Case: <Name>

### Primary Actor
-

### Goal
-

### Preconditions
What must already be true?

### Main Flow

1. User does ...
2. System does ...
3. User confirms ...
4. System saves ...

### Alternative Flows

#### Alternative Flow 1
-

#### Alternative Flow 2
-

### Error Cases

#### Error Case 1
-

#### Error Case 2
-

### Success Outcome
-

---

# 6. Functional Requirements

## FR-001
Description:

### Inputs
-

### Processing
-

### Outputs
-

### Priority
Must Have / Should Have / Nice to Have

---

## FR-002
...

---

# 7. Non-Functional Requirements

## Performance

FlowState must feel **highly responsive**. Users interact with the Gantt chart continuously — expanding rows, logging work, reading comments — and must not be kept waiting on routine actions.

### Response Time Targets

| Category | Target | Notes |
|----------|--------|-------|
| **Interactive operations** | **≤ 1 second** | Acceptable maximum for any routine user action |
| **Unacceptable** | **2–3+ seconds** | Routine operations must **not** block the user this long |
| **Investigation / calculation tasks** | May exceed 1 second | Exception — see below |

### Interactive Operations (≤ 1 second)

The following must complete within 1 second under normal conditions:

- Opening, scrolling, and navigating the Gantt chart
- Expanding / collapsing hierarchy levels (SI, EI, CA, Epic, etc.)
- Toggling views (resource-centric ↔ release-centric), critical path, sprint overlay, SI/EI/CA visibility
- Opening the day action menu (worklog, comments, daily sync)
- Entering and saving a worklog, comment, or daily sync update
- Displaying aggregated worklog values (hours / days) on day cells
- Exporting a due date to Jira
- Generating a pre-filled dependency warning email draft
- Switching between work items, hovering tooltips, showing/hiding columns

### Investigation / Calculation Tasks (exception)

Operations that require non-trivial computation **may exceed 1 second**. These include:

- Critical path calculation across a large release
- Pace-based forecast due date computation
- Release-wide risk assessment and anomaly detection
- AI processing of meeting notes into daily sync updates
- Initial or full Jira sync of a large project
- **Manual force sync with Jira** (user-initiated; duration bounded by Jira API performance)

**Requirements for long-running tasks:**

- Must **not block the entire UI** — run asynchronously where possible
- Must show a **progress indicator** or loading state so the user knows work is in progress
- Results should be **cached** so repeat access is instant (≤ 1 second)
- User can continue other interactions while calculation runs in the background when feasible

### Caching Strategy

FlowState must use **caching** as a primary mechanism for responsiveness.

- **Jira data** (issues, worklogs, comments, sprints, dependencies) is cached locally after sync. Reads served from cache; UI does not call Jira on every interaction.
- **FlowState-owned data** (milestones, daily sync updates, meeting note imports) is stored in the local database and served immediately.
- **Derived calculations** (aggregated worklogs, critical path, forecast dates, at-risk segments) are computed on data change and **cached**; invalidated and recomputed when underlying data changes (new worklog, Jira sync, schedule edit).
- **Stale-while-revalidate:** show cached data immediately (≤ 1s), refresh in background when sync is triggered.
- **Force sync:** user-initiated inbound pull from Jira; may be slow — UI remains usable with progress feedback (see US-022).
- Cache invalidation is **targeted** — only affected items are recomputed, not the entire release, where possible.

### Expected Load

- Expected concurrent users: small to medium delivery team per release (tens, not thousands)
- Data volume: hundreds to low thousands of issues per release; worklogs and comments proportional to team size and release duration

## Scalability

- Architecture must support multiple releases / projects without degrading interactive response times for a single active release view.
- Caching and incremental sync prevent full re-fetch on every user action.
- Investigation tasks may scale with release size; performance targets for calculations are best-effort with progress feedback, not hard ≤ 1s.

## User Interface

FlowState must be a **web-native** application — the primary user interface runs in a modern web browser, not as a installed native desktop client.

### Rationale

Complex, fast-evolving products benefit from web delivery. Products such as Microsoft Teams and Outlook on the web iterate and ship UI improvements far more rapidly than legacy native clients (for example, Skype for Business or Outlook desktop). FlowState's Gantt views, inspection tools, and planning workflows are expected to evolve frequently; a web-native UI reduces the cost and cycle time of those changes.

### Requirements

- [ ] Primary UI is accessed via **modern web browser** (no native desktop install required for V1).
- [ ] UI architecture supports **frequent iteration** — layout, components, and interactions can be updated without a desktop release cycle.
- [ ] Web client communicates with FlowState backend APIs; heavy work (sync, calculations) runs server-side or asynchronously with progress feedback.
- [ ] Responsive layout supports typical desktop browser widths used for planning work (primary target); mobile-native UI is out of scope for V1 unless added later.
- [ ] Supported browsers: current versions of major engines (Chromium, Firefox, Safari, Edge) — exact matrix defined during implementation.

### Frontend Technology Stack

FlowState's web client uses Microsoft's recommended modern web stack:

| Technology | Package / version | Role |
|------------|-------------------|------|
| **React** | React (current stable) | UI framework — component model, state, rendering |
| **TypeScript** | TypeScript (current stable) | Type-safe client code |
| **Fluent UI React v9** | `@fluentui/react-components` | Component library — buttons, inputs, dialogs, layout, theming (aligned with Microsoft 365 design language) |
| **Fluent UI Icons** | `@fluentui/react-icons` | Icon set consistent with Fluent UI v9 |

New UI work should use these libraries rather than introducing parallel component or icon systems, unless a specific requirement cannot be met (for example, a custom Gantt canvas layer may use lower-level rendering while chrome and controls use Fluent UI).

## Availability
-

## Security
-

## Privacy
-

## Compliance
-

## Accessibility
-

## Localization
-

## Observability
- Log operation durations; flag interactive operations exceeding 1 second
- Monitor cache hit rates and Jira sync latency
- Monitor investigation task durations (critical path, forecast, AI processing)
- Audit requirements: worklog changes, Jira exports, emails sent, anomaly approvals, daily snapshots for inspection

---

# 8. Data Model (Business View)

## Entity: Work Item (Jira Issue)

### Description

A Jira **issue** synchronized into FlowState, displayed as a **work item** row in the Gantt chart. See *Naming Conventions* in Terminology. Issue types follow the **Nokia hierarchy** (see Gantt Views epic). Schedule, assignment, and metadata originate from Jira; derived intelligence is computed by FlowState.

### Issue Types

| Type | Abbreviation | Role in hierarchy |
|------|--------------|-------------------|
| System Item | SI | Top-level organizational grouping |
| Entity Item | EI | Child of System Item |
| Competance Area | CA | Child of Entity Item |
| Epic | — | Child of Competance Area; feature-level planning unit |
| User Story | — | Child of Epic; executable work |
| Task | — | Child of Epic; executable work (sibling of User Story) |
| Bug | — | **Independent**; not in the SI → EI → CA → Epic chain |

### Important Attributes

- Issue key, type, summary, status, assignee, priority
- Jira labels (synced; displayed as `J:<name>`)
- Native labels (FlowState-only; displayed as `<name>`)
- Parent issue reference (Jira parent link, per Nokia hierarchy)
- Planned start date, planned due date (from Jira / effort estimate)
- Remaining effort estimate
- Jira rank (ordering)
- Sprint assignment (from Jira): start sprint, end sprint
- Dependencies (blocks / is blocked by)
- Daily allocation cap (FlowState-only; see US-015)

### Scheduling Scope

Gantt bars, worklogs, dependencies, and schedule intelligence apply from **Epic downward** (Epic, User Story, Task, Bug). **Worklog entry in FlowState is restricted to leaf items** (User Story, Task, Bug). Parent items (Epic, CA, EI, SI) display **aggregated worklogs in days**, including direct worklogs synced from Jira on the parent itself plus all descendant hours. Aggregation rolls up to System Item level.

### Relationships

- One-to-many: Work Item → Worklogs
- Many-to-many: Work Item ↔ Work Item (dependencies)

---

## Entity: Worklog

### Description

Time logged against a work item. May be entered in FlowState or synchronized from Jira. Any internal user may log work on any leaf work item, regardless of assignee.

### Important Attributes

- Date, duration (hours), author, description
- Source (FlowState or Jira)
- Target work item (User Story, Task, or Bug — **leaf items only** for FlowState entry)

### Entry Rules

- **FlowState:** worklogs may be created only on leaf items (User Story, Task, Bug).
- **Jira:** worklogs may exist on any issue type, including parent items. These are synced inbound and included in parent aggregation (see US-014).

### Relationships

- Many-to-one: Worklog → Work Item

---

## Entity: Daily Allocation Cap

### Description

A FlowState-only **planning assumption** on how much of each day is expected to be spent on a specific work item. Jira does not store this; FlowState uses it for Gantt bar thickness and allocation-based due date calculation. Does not block worklog entry.

### Important Attributes

- Work item reference
- Allocation type: **fixed rate over interval** or **recurring day-of-week pattern**
- Allocation fraction per applicable day (for example: 0.5 = half day)
- Date interval (start, end) — for fixed-rate type
- Day-of-week pattern (for example: Monday) — for recurring type
- Optional description (for example: "Customer support", "Monday training")

### Relationships

- Many-to-one: Daily Allocation Cap → Work Item

---

## Entity: PTO (Paid Time Off)

### Description

A FlowState-only record of time a resource is unavailable. Not stored in Jira. Used for timeline visualization and schedule calculations.

### Important Attributes

- Resource (team member) reference
- PTO amount (fraction of day unavailable: 0.25, 0.5, 1.0, 1.5, …)
- Pattern type: **single date**, **fixed duration** (consecutive days), or **recurring with limit** (day-of-week + week count)
- Start date; end date or recurrence count (as applicable)
- Optional description (for example: "Vacation", "Dental appointment")

### Relationships

- Many-to-one: PTO → Resource

---

## Entity: Work Item Daily Snapshot

### Description

A point-in-time record of a work item's state captured at the end of each day (or on significant change). Powers the historical inspection views (US-017, US-018, US-019). Stored in FlowState only.

### Important Attributes

- Snapshot date (the "as of" date for this row)
- Work item reference
- Assignee (at snapshot time)
- Effort estimate (person-days, at snapshot time)
- Due date (at snapshot time)
- Scheduled bar: start date, end date, per-day work logged values, remaining **O** bar cells
- For resource inspection: work item labels (T1, T2, …) per calendar column
- For parent inspection: aggregated person-days per calendar column (sum of children)

### Relationships

- Many-to-one: Work Item Daily Snapshot → Work Item
- Many-to-one: Work Item Daily Snapshot → Resource (for resource-scoped queries)

### Capture Triggers

- End-of-day automatic snapshot
- On significant change: assignee change, estimate change, due date change, worklog entry, schedule recalculation

---

### Description

Not a stored entity. Computed from remaining person-day effort, start date (or current date after work begins), and daily allocation cap schedule.

**Initial:** `remaining = total effort estimate`

**After worklogs:** `remaining = total effort estimate − logged work (person-days)`

The due date is the calendar date when the remaining effort is exhausted at the allocation rate. Recalculates on every worklog entry.

### Examples

- 2 person-days at 0.5 day/calendar day → initially 4 calendar days; if Jira full-time due was E, adjusted due ≈ E + 2 days.
- 3 person-days at 0.5 day every Monday → initially 6 Mondays (6 weeks).
- After 1.5 person-days logged on the 2-day example: remaining 0.5 → 1 more calendar day at 0.5/day.

---

## Derived: Aggregated Worklog (per day, per parent item)

### Description

Not a stored entity. Computed for parent items (Epic, CA, EI, SI) on each day:

```text
total hours = direct worklogs on this item (from Jira)
            + sum of all descendant worklog hours
display     = total hours ÷ hours-per-day (default 8)  →  shown as days (d)
```

Aggregation rolls up through CA, EI, and SI to System Item level.

### Examples

- Three User Stories log 8h + 4h + 8h = 20h on day D, no direct Epic worklog → Epic shows **2.5d**.
- Same children (20h) plus 8h worklog on the Epic itself (logged in Jira) → Epic shows **3.5d** (28h ÷ 8).

---

## Entity: Comment (Jira)

### Description

A standard Jira issue comment. Synchronized bidirectionally; created in FlowState is posted to Jira.

### Important Attributes

- Author, timestamp, body text
- Jira comment ID

### Relationships

- Many-to-one: Comment → Work Item

---

## Entity: Native Label

### Description

A classification tag attached to a work item in FlowState only. Does not modify Jira. Displayed without prefix (unlike Jira labels which use `J:`).

### Important Attributes

- Label name
- Work item reference
- Created by, created date

### Relationships

- Many-to-many: Native Label ↔ Work Item

---

## Entity: Daily Sync Update

### Description

A standup-style progress note tied to a specific work item and date. Stored in FlowState only — not visible in Jira.

### Important Attributes

- Work item reference, date, author, body text
- Source (manual entry or AI-generated from meeting notes)

### Relationships

- Many-to-one: Daily Sync Update → Work Item

---

## Entity: Meeting Notes Import

### Description

A batch of meeting notes submitted by a manager for AI processing into daily sync updates.

### Important Attributes

- Raw text, import date, author
- Processing status (pending, proposed, approved, rejected)
- Linked proposed daily sync updates

### Relationships

- One-to-many: Meeting Notes Import → proposed Daily Sync Updates

---

## Entity: Milestone

### Description

A custom release event defined and stored in FlowState. Not synchronized to or from Jira. The **code freeze date** is typically represented as a milestone (or dedicated release setting).

### Important Attributes

- Name, date, description (optional)
- Type (optional: for example, code freeze, release, custom)
- Release or project scope

### Relationships

- Many-to-one: Milestone → Release / Project

---

## Entity: Sprint (from Jira)

### Description

A Jira sprint used to segment the timeline. Sprint **definitions** (name, dates) are read from Jira on sync. FlowState may **write** start sprint and end sprint assignments back to Jira when the manager approves a sprint update run (US-023).

### Important Attributes

- Sprint name, start date, end date, state (active / future / closed)
- Jira sprint ID

### Relationships

- One-to-many: Sprint → Work Items (via Jira start sprint / end sprint fields)

---

## Derived: Critical Path

### Description

Not a stored entity. Computed from the dependency graph and schedule. Identifies the sequence of work items with zero slack that determines the earliest possible release completion date.

---

## Derived: Forecast Due Date

### Description

Not a stored entity. Computed per work item from remaining effort and historical pace (worklogs). May differ from the Jira planned due date.

---

# 9. Business Rules

## BR-001: Critical Path Calculation

The critical path is derived from work item dependencies and schedule data. A work item is on the critical path when any delay to that work item would delay the release completion date. Work items with dependent work blocked behind them (for example, a backend API blocking UI development) are candidates for critical-path membership.

## BR-002: Planned Due Date

The planned due date reflects the assignee's effort estimate and scheduled position. It is sourced from Jira (or the original plan) and represents what was committed to — not what current pace suggests.

## BR-003: Forecast Due Date

The forecast due date is calculated from remaining effort and the rate of progress inferred from worklogs. It answers: "At the current pace, when will this actually be done?" The forecast recalculates whenever worklogs change.

## BR-004: Due Date Export to Jira

When a user exports a forecast due date to Jira, FlowState overwrites the Jira issue due date with the forecast value. The export requires explicit user action (one or two clicks); it is never automatic.

## BR-005: Milestone Storage

Custom milestones exist only in the FlowState database. They are not created, updated, or deleted in Jira.

## BR-006: Sprint Definitions and Assignments

Sprint **definitions** (name, start date, end date) are owned by Jira and read by FlowState on sync. **Start sprint** and **end sprint** on each issue may be **updated by FlowState** when a manager approves a sprint assignment run (US-023). FlowState derives these from work item start/due dates and the sprint calendar; it does not invent sprint definitions.

## BR-025: Start and End Sprint Mapping

- **Start sprint** = the Jira sprint whose date range contains the work item's start date.
- **End sprint** = the Jira sprint whose date range contains the work item's due date.
- Mapping runs on **scheduled** and **manual** triggers. Proposed changes require manager approval before writing to Jira.
- Work items without a resolvable start or due date are excluded from automatic assignment unless manually overridden.

## BR-007: Code Freeze At-Risk Rendering

Any portion of a work item's timeline that extends beyond the code freeze date is rendered as at-risk for all users. The at-risk segment is calculated from the later of the planned end date and the forecast end date.

## BR-008: Comment vs Daily Sync Update

Regular comments are Jira comments — they sync to Jira and are visible in Jira. Daily sync updates are FlowState-only records tied to a work item and a specific date; they are never posted to Jira.

## BR-009: Daily Sync Update Visibility

Daily sync updates are displayed on the Gantt chart via day-cell indicators and revealed on hover. They provide standup-style context without cluttering Jira comment threads.

## BR-010: AI-Generated Daily Sync Updates

Meeting notes processed by the AI agent produce proposed daily sync updates. No update is saved until a manager reviews and approves it. The AI agent uses the FlowState MCP server to access release and work item context.

## BR-011: Dependency Warning Emails

Any internal user may send a dependency warning email. FlowState pre-fills the draft from live dependency and schedule data (blocking work item, dependent work item, required start date, recipient). The email is sent only after explicit user review and confirmation — never automatically.

## BR-012: Nokia Issue Type Hierarchy

FlowState mirrors the Nokia Jira parent-child hierarchy: System Item → Entity Item → Competance Area → Epic → User Story / Task. Bugs are independent and are not placed under Epic in the tree. Parent-child relationships are sourced from Jira issue links.

## BR-013: Show / Hide Organizational Levels

Users can independently toggle visibility of System Items (SI), Entity Items (EI), and Competance Areas (CA). All three are **hidden by default** because delivery work is tracked from the Epic level onward. When a level is hidden, its descendants connect to the next visible ancestor in the tree.

## BR-014: Worklog Entry — Leaf Only in FlowState

FlowState accepts worklog entry only on **leaf work items** (User Story, Task, Bug). Any internal user may log work on any leaf work item, regardless of assignee. The worklog is attributed to the logging user and synchronized to Jira.

Jira may contain worklogs on parent items (Epic, CA, EI, SI). FlowState does not allow creating worklogs on parents, but **does sync and display** them as part of parent aggregation.

## BR-015: Worklog Display — Hours at Leaf, Days at Parent

- **Leaf items** (User Story, Task, Bug): worklog totals per day are displayed in **hours** (suffix `h`).
- **Parent items** (Epic, Competance Area, Entity Item, System Item): worklog totals per day are displayed as **aggregated days** (suffix `d`), calculated as:

  ```text
  (direct Jira worklogs on this item + sum of all descendant worklog hours) ÷ hours-per-day
  ```

- Aggregation rolls up through every parent level to **System Item**.
- The UI must always make the unit explicit. Hovering a parent day cell shows direct worklogs, per-child breakdown, total hours, and converted days.

## BR-016: Interactive Responsiveness

Routine user operations must complete within **1 second**. Operations that routinely take 2–3 seconds or longer are unacceptable for interactive use. Investigation and calculation tasks (critical path, forecasting, risk analysis, AI processing) are exempt but must run asynchronously with progress feedback and cache their results for subsequent access.

## BR-017: Daily Allocation Cap

Daily allocation caps exist only in FlowState. They represent the **planned** daily effort available for a work item and determine Gantt bar vertical thickness (allocation fraction × full row height). The cap is a scheduling assumption — it does **not** prevent developers from logging more work than the cap on any day.

## BR-018: Allocation-Based Due Date

The allocation-based due date is calculated by spreading **remaining person-day effort** across the available allocation slots defined by the cap:

```text
remaining effort = total effort estimate − work completed (from worklogs)
required slots   = remaining effort ÷ allocation per slot (person-days)
due date         = current date + mapped calendar slots
```

Before any work is logged, `remaining effort = total effort estimate`.

On each calendar day, the assignee's **available capacity** is:

```text
available          = 1.0 − PTO fraction (if any)
effective slot     = min(work-item allocation cap, available capacity)
```

Full-day PTO (1.0) provides no scheduling slot for that resource on that day.

- **Fixed rate:** slots are consecutive calendar days at the given rate, **skipping or reducing PTO days**.
- **Recurring pattern:** slots are only the matching weekdays, **accounting for PTO on those days**.

The allocation-based due date **recalculates after every worklog**, when PTO changes, and when logged work exceeds the daily allocation cap. It may be exported to Jira on explicit user action. It is distinct from the pace-based forecast due date (BR-003).

## BR-019: Excess Worklogs and Due Date Recalculation

Developers may log more work on a day than the daily allocation cap specifies. FlowState accepts the worklog, updates work completed, and immediately recalculates the allocation-based due date from the new remaining effort. Logging above the cap may shorten the projected end date; falling behind the planned allocation may lengthen it.

## BR-020: PTO Storage and Capacity

PTO exists only in FlowState (not Jira). PTO reduces a resource's available capacity on affected days. Supported amounts include fractional days (0.25, 0.5, 1.0, 1.5) and patterns include single dates, fixed multi-day spans, and recurring rules with a limit. PTO is displayed on the resource timeline and factored into allocation-based due date calculation for work items assigned to that resource.

## BR-021: Daily Snapshot Capture for Inspection

FlowState captures a daily snapshot of each work item's assignee, effort estimate, due date, scheduled bar, and per-day work logged values. Snapshots power the three inspection views:

1. **Work item inspection** — one work item's mini-Gantt per snapshot row.
2. **Resource inspection** — all assigned work items inline per snapshot row.
3. **Parent work item inspection** — aggregated child effort per snapshot row.

Snapshots are retained for the life of the release (or per configured retention policy). Inspection views are read-only.

## BR-022: Jira Labels vs Native Labels

Jira labels are read-only in FlowState (synced inbound) and displayed with the `J:` prefix. Native labels are FlowState-only; adding or removing them does not write to Jira. Both label types appear in the Labels column and are available in filter expressions.

## BR-023: Gantt View Filtering

Filtering applies only to resource-centric and release-centric Gantt views. Supported boolean operators: AND, OR, NOT, XOR, IN, NOT IN, and parentheses. Filtering narrows visible work items; it does not sort, reorder (Jira ranking is preserved), or replace JQL for general issue search or bug scrubbing in Jira.

## BR-024: Force Sync with Jira

Any user may trigger a manual **inbound** force sync to pull the latest data from Jira. Force sync duration is not subject to the 1-second interactive response target — it depends on release size and Jira API performance. The UI must show progress and last-synced status; cached data remains available during and after failed syncs. Force sync does not push changes to Jira.

## BR-026: Incomplete Daily Worklog Detection

For each resource on each working day, FlowState compares total logged hours (summed across all work items) against expected hours:

```text
expected hours = hours-per-day − (PTO fraction × hours-per-day)
```

If logged hours are less than expected hours, the resource is treated as idle for the difference. FlowState proposes a reminder email; the manager must approve before send. Full-day PTO (1.0) sets expected hours to zero and suppresses the anomaly for that day.

---

# 10. Permissions and Roles

| Role | Create | Read | Update | Delete | Approve | Admin |
|--------|--------|--------|--------|--------|--------|--------|
| User | | | | | | |
| Manager | | | | | | |
| Admin | | | | | | |

---

# 11. Integrations

## Integration: Atlassian Jira

### Purpose

Jira is the system of record for work items, worklogs, dependencies, sprints, and issue metadata. FlowState reads this data to power Gantt views, schedule intelligence, and risk analysis; it writes back selective updates (worklogs, due dates, comments, status) when the user approves or explicitly exports.

### Data Exchanged

**Inbound (Jira → FlowState):**

- Issues: System Item, Entity Item, Competance Area, Epic, User Story, Task, Bug — key, type, summary, status, assignee, Jira labels, priority, due date, rank, parent link
- Dependencies (blocks / is blocked by)
- Worklogs: date, duration, author
- Comments: author, timestamp, body
- Sprints: name, start date, end date, state, issue membership

**Outbound (FlowState → Jira):**

- Worklog entries created in FlowState
- Due date updates (pace-based forecast export, allocation-based due date export)
- Comments added from the Gantt chart
- Status changes (via approved anomaly remediation)
- Start sprint and end sprint (via approved sprint assignment run — US-023)

### Trigger

- Scheduled background sync (interval configured by Administrator)
- **User-initiated force sync** (inbound only; see US-022)
- **Scheduled sprint assignment run** (US-023; outbound after manager approval)
- Immediate push on user actions (worklog entry, comment, due date export, approved remediation, approved sprint updates)

### Failure Handling

- Failed writes are surfaced to the user with a clear error; no silent data loss
- Stale data is indicated in the UI when sync fails or Jira is unreachable
- Retry on transient API errors; user can manually retry failed exports or **retry force sync**
- Slow Jira responses during force sync are not treated as FlowState defects; progress is shown until complete or failed

---

## Integration: FlowState MCP Server

### Purpose

Provides structured release and work item context to the AI agent. Enables intelligent mapping of meeting notes to the correct Jira issues when generating daily sync updates.

### Data Exposed

- Work items: keys, summaries, types, assignees, statuses, dates
- Dependencies and critical-path membership
- Existing daily sync updates and recent worklogs
- Release milestones (including code freeze date)

### Trigger

- Invoked when a manager imports meeting notes for AI processing (US-012)
- Invoked by other AI-assisted features (anomaly remediation, risk warnings) as needed

### Failure Handling

- If MCP server is unavailable, AI processing is disabled with a clear message; manual daily sync entry remains available
- AI proposals that cannot be confidently mapped are flagged for manual review

---

## Integration: Email (SMTP)

### Purpose

Sends user-initiated emails such as dependency warnings (US-013) and manager-approved risk notifications. FlowState is not an email platform; it sends targeted messages on behalf of users.

### Data Exchanged

**Outbound:**

- Dependency warning emails: recipient, subject, body, referenced issue keys
- Manager-approved risk notification emails (from anomaly remediation)
- Incomplete daily worklog reminder emails (US-024)

**Stored in FlowState (audit log):**

- Sender, recipient, timestamp, work items referenced, email type

### Trigger

- Manager reviews and confirms a dependency warning email (US-013)
- Manager approves an automated risk notification for email delivery
- Manager approves an incomplete daily worklog reminder email (US-024)

### Failure Handling

- Failed sends are reported to the user with a clear error; the draft is preserved for retry
- If email is not configured, FlowState offers to copy the draft to clipboard as a fallback

---

# 12. External Constraints

## Technology Constraints

- FlowState V1 is a **web-native application**. The client is a browser-based UI; V1 does not ship a native desktop executable.
- FlowState must maintain a **local cache** of Jira data and derived calculations to meet interactive response time targets (≤ 1 second).
- The application must not require a round-trip to Jira for routine UI interactions (navigation, display, worklog entry UI).
- UI changes should be deployable without requiring users to install or update a desktop client — consistent with the web-native delivery model.

### Frontend Stack (V1)

| Layer | Choice |
|-------|--------|
| Language | **TypeScript** |
| UI framework | **React** |
| Component library | **`@fluentui/react-components`** (Fluent UI **v9**) |
| Icons | **`@fluentui/react-icons`** |

This stack matches Microsoft's recommended approach for modern internal web applications and supports the rapid UI iteration goals described in §7 User Interface.

## Infrastructure Constraints

- Deployed as a web application accessible to internal users on the company network (or VPN), with browser clients connecting to a FlowState backend service.
- Jira connectivity and sync run server-side; users authenticate with Jira credentials or tokens as configured by the Administrator.

## Regulatory Constraints
-

## Organizational Constraints
-

---

# 13. Reporting & Analytics

## Required Reports
-

## Dashboards
-

## KPIs
-

---

# 14. Acceptance Criteria

## Release Acceptance

### Must Be True Before Launch
- [ ]
- [ ]
- [ ]

### Business Acceptance
- [ ]
- [ ]
- [ ]

---

# 15. Edge Cases

## Edge Case 1
Description

Expected behavior

## Edge Case 2
Description

Expected behavior

---

# 16. Open Questions

## Question 1
-

## Question 2
-

---

# 17. Future Enhancements

## Version 2 Ideas
-

## Version 3 Ideas
-

---

# Appendix


## Access Philosophy

FlowState is designed for **internal delivery transparency**, not universal stakeholder access.

### Internal-Only by Design

FlowState contains sensitive operational information: delivery risks, schedule confidence, critical paths, dependency bottlenecks, resource activity patterns, anomaly detection, and proposed mitigation actions. This information is intended for people directly involved in planning, executing, and steering a release.

Customers, external partners, and business stakeholders outside the delivery organization are not intended users. They must not receive direct access to FlowState or its operational views.

### Shared Visibility Within the Delivery Team

Inside the delivery organization, FlowState intentionally minimizes role-based restrictions. All authenticated internal users — release managers, product owners, developers, testers, designers, and other delivery participants — should generally have access to the same release plans, schedules, dependencies, risks, forecasts, and delivery metrics.

The goal is a single, consistent internal view of reality so the team can plan and respond from the same facts.

### Simple Role Model

FlowState does not implement complex identity or permission management. Authorization is inherited from Jira connectivity: users authenticate with Jira credentials or tokens sufficient to access the relevant project data.

For V1, two roles are sufficient:

| Role | Purpose |
|------|---------|
| **Administrator** | Configure the application — Jira connectivity, sync settings, notifications, AI provider, system settings |
| **User** | Use all planning, Gantt, risk analysis, forecasting, and notification features |

Administrators configure the system; they do not govern business-level access to individual releases. There is no per-release or per-feature permission matrix for internal users in the initial version.

### External Disclosure

Any information shared outside the delivery team — to executives, PMO, customers, or partners — is intentional and curated. FlowState does not expose operational delivery intelligence to external audiences by default.

Export, reporting, and communication features may support preparing outward-facing summaries, but the release manager retains control over what crosses the organizational boundary.

## Terminology

### Naming Conventions

FlowState uses consistent terms to avoid confusion with Jira issue types (especially **Task**, which is a specific type under Epic).

| Term | Meaning | Use in |
|------|---------|--------|
| **Work item** | Any row FlowState displays or operates on — SI, EI, CA, Epic, User Story, Task, Bug | User stories, UI, requirements, Gantt views |
| **Issue** | The corresponding record in Jira that FlowState syncs with | Jira integration, REST API, sync, parent links |
| **Issue type** | The Jira classification: System Item, Entity Item, Competance Area, Epic, User Story, Task, Bug | When distinguishing *what kind* of work item |
| **Leaf work item** | User Story, Task, or Bug — executable work; FlowState worklog entry applies here | Worklog, scheduling at the lowest level |
| **Parent work item** | Epic, CA, EI, or SI — grouping rows with aggregated worklogs | Aggregation, roll-up display |
| **Ticket** (informal) | Spoken synonym for *work item* | Avoid in requirements; acceptable in quoted user language |

**Rule of thumb:** say **work item** in FlowState context; say **issue** when referring to Jira data or the API. Never use **task** generically — reserve it for the **Task** issue type only.

### Web-native UI

FlowState is delivered as a browser-based application (not a native desktop client for V1). Chosen to enable faster UI evolution and deployment, similar to how Teams and Outlook on the web outpace their legacy desktop counterparts in iteration speed. Built with **React**, **TypeScript**, **`@fluentui/react-components` (v9)**, and **`@fluentui/react-icons`**.

---

### System Item (SI)

Top-level organizational issue type in the Nokia Jira hierarchy. Groups Entity Items. Hidden by default in FlowState Gantt views.

### Entity Item (EI)

Second-level organizational issue type. Child of System Item; parent of Competance Area. Hidden by default in FlowState Gantt views.

### Competance Area (CA)

Third-level organizational issue type in Nokia Jira. Child of Entity Item; parent of Epic. Hidden by default in FlowState Gantt views. (Spelling follows Nokia internal convention.)

### Epic

Feature-level issue type. Child of Competance Area. Primary planning unit in FlowState — scheduling and progress tracking begin here.

### User Story

Executable work item. Child of Epic. Sibling of Task.

### Task (issue type)

Executable work item. Child of Epic. Sibling of User Story. **Not** a generic term — see *Naming Conventions* above.

### Bug

Independent issue type — not part of the SI → EI → CA → Epic chain. Appears at the Resource level (resource-centric view) or release root (release-centric view).

### Worklog (hours)

Time logged against a leaf work item (User Story, Task, or Bug). Displayed on the Gantt day cell in **hours** (for example: `8h`).

### Aggregated worklog (days)

For parent items (Epic, CA, EI, SI): the sum of **direct worklogs on the item from Jira** plus **all descendant worklog hours** on a given day, converted to days by dividing by hours-per-day (default 8). Displayed with a `d` suffix (for example: `2.5d` without direct parent worklog; `3.5d` when the Epic itself has an additional 8h from Jira). Rolls up to System Item level.

### Daily allocation cap

A FlowState-only **planned** daily effort for a work item (for example: 0.5 day/day for customer support, or 0.5 day every Monday for training). Affects Gantt bar thickness and allocation-based due date calculation. Does not block worklog entry — developers may log more than the cap. Not stored in Jira.

### Allocation-based due date

A due date computed from **remaining person-day effort** and the daily allocation cap schedule. Recalculates after every worklog. For example: 3 person-days at half-day Mondays only → initially 6 weeks out; shrinks as work is logged. May be exported to Jira.

### PTO (paid time off)

FlowState-only record of time a resource is unavailable (fraction of a day: 0.25, 0.5, 1.0, 1.5, etc.). Patterns include single dates, fixed multi-day spans (for example, 0.5 day for 4 days), and recurring rules (for example, 0.5 day every Friday for 4 weeks). Shown on the resource timeline; reduces available capacity for schedule calculations.

### Inspection view

A read-only historical replay of daily snapshots. Rows = snapshot dates; columns = calendar days. Three types: **work item** (US-017), **resource** (US-018), **parent work item** (US-019). Cells use **O** for future bar portions and numeric values for logged person-days.

### Jira label

A label synchronized from Jira. Displayed in FlowState with the **`J:`** prefix (for example: `J:backend`). Read-only in FlowState.

### Native label

A label stored in FlowState only. Displayed **without** prefix (for example: `needs-review`). Can be added or removed without modifying the Jira issue.

### Force sync

A user-initiated inbound pull of the latest data from Jira. May take significant time when Jira is slow or the release is large; not subject to the 1-second interactive response target.

### Start sprint / End sprint

Jira fields on an issue indicating which sprint contains the work item's **start date** and **due date**, respectively. FlowState can compute and update these from the work item schedule and Jira sprint calendar (US-023).

### Incomplete daily worklog (anomaly)

A resource logged fewer hours than expected on a working day: `logged < (hours-per-day − PTO hours)`. Treated as partial idle time; FlowState may propose a manager-approved reminder email (US-024).

---

## Example Scenarios

Scenario 1

Scenario 2
