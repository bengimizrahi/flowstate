use crate::app::*;
use chrono::NaiveDate;
use chrono::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TaskInspection {
    pub task_id: TaskId,
    pub allocations_history: HashMap<NaiveDate, HashMap<NaiveDate, Fraction>>,
    pub absences_history: HashMap<NaiveDate, HashMap<NaiveDate, Fraction>>,
    pub worklogs_history: HashMap<NaiveDate, HashMap<NaiveDate, Fraction>>,
    pub assignee_history: HashMap<NaiveDate, Option<ResourceId>>,
    pub flow_state: FlowState,
    pub start_date: NaiveDate,
}

impl TaskInspection {
    pub fn new(inspected_task_id: TaskId) -> Self {
        TaskInspection {
            task_id: inspected_task_id,
            allocations_history: HashMap::new(),
            absences_history: HashMap::new(),
            worklogs_history: HashMap::new(),
            assignee_history: HashMap::new(),
            flow_state: FlowState::new(),
            start_date: NaiveDate::MAX,
        }
    }

    pub fn from(inspected_task_id: TaskId, commands: Vec<Command>, date: NaiveDate) -> Self {
        let mut task_inspector = TaskInspection::new(inspected_task_id);
            let extract_create_date = |mut commands: Vec<Command>| -> Option<NaiveDate> {
                while let Some(c) = commands.pop() {
                    if let CommandDetails::CreateTask { id, .. } = c.details {
                        if id == inspected_task_id {
                            return Some(c.timestamp.date_naive());
                        }
                    } else if let CommandDetails::CompoundCommand { commands: inner_commands } = c.details {
                        commands.extend(inner_commands);
                    }
                }
                None
            };
        task_inspector.start_date = extract_create_date(commands.clone()).unwrap();

        let (start_date, end_date) = {
            let flow_state = FlowState::from_commands(&commands, date).unwrap();
            let s = flow_state.cache().day(0);
            let e = flow_state.cache().day(flow_state.cache().num_days() - 1);
            (s, e)
        };

        
        let mut commands_by_date = HashMap::new();
        for cmd in commands {
            let date = cmd.timestamp.date_naive();
            commands_by_date.entry(date).or_insert_with(Vec::new).push(cmd);
            
        }
        let mut flow_state = FlowState::new();
        let mut date_it = start_date;
        while date_it <= end_date {
            if let Some(cmds) = commands_by_date.get(&date_it) {
                for cmd in cmds {
                    flow_state.execute_command_and_generate_inverse(cmd.clone()).unwrap();
                }
            }
            flow_state.rebuild_cache(date_it);
            let assignee = flow_state.tasks.get(&inspected_task_id)
                .and_then(|task| task.assignee);
            task_inspector.assignee_history.insert(date_it, assignee);
            task_inspector.allocations_history.insert(date_it, 
                    flow_state.cache().task_alloc_rendering
                        .get(&inspected_task_id).cloned().unwrap_or_default());
            if let Some(assignee) = assignee {
                task_inspector.absences_history.insert(date_it, 
                        flow_state.cache().resource_absence_rendering
                            .get(&assignee).cloned().unwrap_or_default());
            }
            task_inspector.worklogs_history.insert(date_it, 
                    flow_state.worklogs.get(&inspected_task_id)
                        .and_then(|resource_map| resource_map.get(&assignee.unwrap_or(0)))
                        .map(|date_map| {
                            date_map.iter()
                                .map(|(d, w)| (*d, w.fraction))
                                .collect::<HashMap<NaiveDate, Fraction>>()
                        }).unwrap_or_default());
            date_it = date_it + Duration::days(1);
        }

        task_inspector.flow_state = flow_state;
        task_inspector
    }
}