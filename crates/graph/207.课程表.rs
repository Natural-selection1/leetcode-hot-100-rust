/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */

// @lc code=start
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum VisitState {
    NotVisited,
    Visiting,
    Visited,
}

impl crate::Solution {
    pub fn can_finish(courses_count: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let courses_count = courses_count as usize;

        let successor_table: HashMap<usize, Vec<usize>> =
            prerequisites
                .into_iter()
                .fold(HashMap::new(), |mut table, relationship| {
                    let successor = relationship[0] as usize;
                    let prerequisite = relationship[1] as usize;
                    table.entry(prerequisite).or_default().push(successor);
                    table
                });

        // 深度优先判断是否有环
        let mut visit_states = vec![VisitState::NotVisited; courses_count];
        for course in 0..courses_count {
            if visit_states[course] == VisitState::NotVisited
                && has_cycle(course, &successor_table, &mut visit_states)
            {
                return false;
            }
        }

        true
    }
}

fn has_cycle(
    course: usize,
    successor_graph: &HashMap<usize, Vec<usize>>,
    visit_states: &mut [VisitState],
) -> bool {
    visit_states[course] = VisitState::Visiting;

    if let Some(successors) = successor_graph.get(&course) {
        for &successor in successors {
            match visit_states[successor] {
                VisitState::NotVisited => {
                    if has_cycle(successor, successor_graph, visit_states) {
                        return true;
                    }
                }
                VisitState::Visiting => return true,
                VisitState::Visited => continue,
            }
        }
    }

    visit_states[course] = VisitState::Visited;
    false
}
// @lc code=end
