/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */

// @lc code=start
#[derive(Clone, PartialEq)]
enum VisitState {
    NotVisited,
    Visiting,
    Visited,
}

impl crate::Solution {
    pub fn can_finish(courses_count: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let courses_count = courses_count as usize;
        let successor_graph = prerequisites.iter().fold(
            vec![vec![]; courses_count],
            |mut successors_of, relationship| {
                let successor = relationship[0] as usize;
                let prerequisite = relationship[1] as usize;
                successors_of[prerequisite].push(successor);
                successors_of
            },
        );

        let mut visit_states = vec![VisitState::NotVisited; courses_count];
        for course in 0..courses_count {
            if visit_states[course] == VisitState::NotVisited
                && has_cycle(course, &successor_graph, &mut visit_states)
            {
                return false;
            }
        }

        true
    }
}

fn has_cycle(
    course: usize,
    successor_graph: &[Vec<usize>],
    visit_states: &mut [VisitState],
) -> bool {
    visit_states[course] = VisitState::Visiting;

    for &successor in &successor_graph[course] {
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

    visit_states[course] = VisitState::Visited;
    false
}
// @lc code=end
