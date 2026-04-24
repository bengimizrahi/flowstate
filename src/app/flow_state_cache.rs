use crate::app::*;
use chrono::{NaiveDate, Utc, Duration, Datelike};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct FlowStateCache {
    start_date: NaiveDate,
    end_date: NaiveDate,
    pub date_to_milestones: BTreeMap<NaiveDate, Vec<Milestone>>,
    pub unassigned_tasks: Vec<TaskId>,
    pub task_alloc_rendering: HashMap<TaskId, HashMap<NaiveDate, Fraction>>,
    pub resource_absence_rendering: HashMap<ResourceId, HashMap<NaiveDate, Fraction>>,
    pub worklogs_on_others_tasks: HashMap<ResourceId, HashMap<NaiveDate, Fraction>>,
}

impl FlowStateCache {
    pub fn new() -> Self {
        FlowStateCache {
            start_date: Utc::now().date_naive(),
            end_date: Utc::now().date_naive(),
            date_to_milestones: BTreeMap::new(),
            unassigned_tasks: Vec::new(),
            task_alloc_rendering: HashMap::new(),
            resource_absence_rendering: HashMap::new(),
            worklogs_on_others_tasks: HashMap::new(),
        }
    }

    pub fn from(flow_state: &FlowState, date: NaiveDate) -> Self {
        let resource_absence_rendering: HashMap<ResourceId, HashMap<NaiveDate, Fraction>> = flow_state.resources.iter()
            .map(|(resource_id, resource)| {
                let absence_map = {
                    resource.absences.iter().fold(HashMap::new(), |mut acc, absence| {
                        let mut current_date = absence.start_date;
                        let mut remaining_days = absence.duration.days;
                        
                        while remaining_days > 0 {
                            if current_date.weekday() != chrono::Weekday::Sat && current_date.weekday() != chrono::Weekday::Sun {
                                acc.entry(current_date).or_insert(100);
                                remaining_days -= 1;
                            }
                            current_date = current_date + Duration::days(1);
                        }
                        
                        if absence.duration.fraction > 0 {
                            while current_date.weekday() == chrono::Weekday::Sat || current_date.weekday() == chrono::Weekday::Sun {
                                current_date = current_date + Duration::days(1);
                            }
                            acc.entry(current_date).or_insert(absence.duration.fraction);
                        }
                        acc
                    })
                };
                (*resource_id, absence_map)
            })
            .collect();
        let date_to_milestones = flow_state.milestones.iter()
            .fold(BTreeMap::new(), |mut acc, milestone| {
                acc.entry(milestone.date)
                    .or_insert_with(Vec::new)
                    .push(milestone.clone());
                acc
            });
        let unassigned_tasks : Vec<TaskId> = flow_state.tasks.iter()
            .filter(|(_, task)| task.assignee.is_none())
            .map(|(id, _)| *id)
            .collect();
        let total_worklogs: HashMap<TaskId, TaskDuration> = flow_state.worklogs.iter()
            .map(|(task_id, resource_map)| {
                let total = resource_map.values()
                    .flat_map(|date_map| date_map.values())
                    .fold(TaskDuration { days: 0, fraction: 0 }, |acc, worklog| {
                        acc + TaskDuration { days: 0, fraction: worklog.fraction }
                    });
                (*task_id, total)
            })
            .collect();
        let remaining_durations: HashMap<TaskId, TaskDuration> = flow_state.tasks.iter()
            .map(|(task_id, task)| {
                let total_worklog = total_worklogs.get(task_id)
                    .cloned()
                    .unwrap_or(TaskDuration { days: 0, fraction: 0 });
                (*task_id, TaskDuration::zero().max(task.duration.clone() - total_worklog))
            })
            .collect();

        let mut most_farther_alloc_date = date;
        let mut task_alloc_rendering: HashMap<TaskId, HashMap<NaiveDate, Fraction>> = HashMap::new();
        for (resource_id, resource) in &flow_state.resources {
            let mut cursor = AllocCursor::new(date);
            for task_id in &resource.assigned_tasks {
                if let Some(_task) = flow_state.tasks.get(task_id) {
                    let mut remaining_alloc = remaining_durations.get(task_id)
                        .cloned()
                        .unwrap_or(TaskDuration { days: 0, fraction: 0 });
                    while remaining_alloc > (TaskDuration { days: 0, fraction: 0 }) {
                        let absence_for_current_day = resource_absence_rendering.get(resource_id)
                            .and_then(|absence_map| absence_map.get(&cursor.date))
                            .copied()
                            .unwrap_or(0);
                        let total_worklog_for_current_day = flow_state.worklogs.iter()
                            .filter_map(|(_, resource_map)| resource_map.get(resource_id))
                            .filter_map(|date_map| date_map.get(&cursor.date))
                            .map(|w| w.fraction)
                            .sum::<Fraction>();
                        let remaining_alloc_for_current_day = TaskDuration { days: 1, fraction: 0 } 
                            - cursor.alloced_amount
                            - TaskDuration { days: 0, fraction: total_worklog_for_current_day }
                            - TaskDuration { days: 0, fraction: absence_for_current_day };
                        let work_to_allocate = remaining_alloc.min(remaining_alloc_for_current_day);
                        if work_to_allocate > (TaskDuration { days: 0, fraction: 0 }) {
                            task_alloc_rendering.entry(*task_id).or_default()
                                .insert(cursor.date, work_to_allocate.into());
                        }
                        remaining_alloc -= work_to_allocate;
                        if remaining_alloc == (TaskDuration { days: 0, fraction: 0 }) {
                            cursor += work_to_allocate;
                        } else {
                            cursor.advance_to_next_working_day();
                            most_farther_alloc_date = most_farther_alloc_date.max(cursor.date);
                        }
                    }
                }
            }
        }
        let today = date;
        for task_id in &unassigned_tasks {
            let mut remaining_alloc = remaining_durations.get(task_id)
                .cloned()
                .unwrap_or(TaskDuration { days: 0, fraction: 0 });
            let mut date = today;
            while remaining_alloc > (TaskDuration { days: 0, fraction: 0 }) {
                while date.weekday() == chrono::Weekday::Sat || date.weekday() == chrono::Weekday::Sun {
                    date += Duration::days(1);
                }
                let work_to_allocate = remaining_alloc.min(TaskDuration { days: 1, fraction: 0 });
                task_alloc_rendering.entry(*task_id).or_default().insert(date, work_to_allocate.into());
                remaining_alloc -= work_to_allocate;
                date += Duration::days(1);
            }
            most_farther_alloc_date = most_farther_alloc_date.max(date);
        }
        let mut worklogs_on_others_tasks: HashMap<ResourceId, HashMap<NaiveDate, Fraction>> = HashMap::new();
        for (task_id, task) in &flow_state.tasks {
            if let Some(resource_map) = flow_state.worklogs.get(task_id) {
                for (resource_id, date_map) in resource_map {
                    for (date, worklog) in date_map {
                        if task.assignee.is_none() || *resource_id != task.assignee.unwrap() {
                            *worklogs_on_others_tasks
                                .entry(*resource_id)
                                .or_insert_with(HashMap::new)
                                .entry(*date)
                                .or_insert(0) += worklog.fraction;
                        }
                    }
                }
            }
        }
        let mut start_date = flow_state.milestones.iter()
            .map(|m| m.date)
            .min()
            .unwrap_or(Utc::now().date_naive());
        start_date = start_date.min(flow_state.worklogs.iter()
            .flat_map(|(_, resource_map)| resource_map.values())
            .flat_map(|date_map| date_map.keys())
            .cloned()
            .min()
            .unwrap_or(start_date));
        start_date = start_date.min(flow_state.resources.iter()
            .flat_map(|(_, resource)| resource.absences.iter())
            .map(|absence| absence.start_date)
            .min()
            .unwrap_or(NaiveDate::MAX));
        start_date = start_date.checked_sub_signed(Duration::days(30))
            .unwrap_or(NaiveDate::MIN);
        let mut end_date = flow_state.milestones.iter()
            .map(|m| m.date)
            .max()
            .unwrap_or(Utc::now().date_naive());
        end_date = end_date.max(flow_state.worklogs.iter()
            .flat_map(|(_, resource_map)| resource_map.values())
            .flat_map(|date_map| date_map.keys())
            .cloned()
            .max()
            .unwrap_or(end_date));
        end_date = end_date.max(flow_state.resources.iter()
            .flat_map(|(_, resource)| resource.absences.iter())
            .map(|absence| absence.start_date)
            .max()
            .unwrap_or(NaiveDate::MIN));
        end_date = end_date.max(most_farther_alloc_date);
        end_date = end_date.checked_add_signed(Duration::days(30))
            .unwrap_or(NaiveDate::MAX);
        FlowStateCache {
            start_date,
            end_date,
            date_to_milestones,
            unassigned_tasks,
            task_alloc_rendering,
            resource_absence_rendering,
            worklogs_on_others_tasks,
        }
    }

    pub fn day(&self, index: usize) -> NaiveDate {
        self.start_date + Duration::days(index as i64)
    }

    pub fn num_days(&self) -> usize {
        self.end_date.signed_duration_since(self.start_date).num_days() as usize
    }
}